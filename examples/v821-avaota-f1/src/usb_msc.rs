//! Read-only USB mass-storage Bulk-Only/SCSI transport.

use crate::usb::UsbMassStorageTransport;

pub const BLOCK_SIZE: usize = 512;

/// Supplies one logical block to the SCSI transport.
pub type ReadSector = fn(u32, &mut [u8; BLOCK_SIZE]);

const CSW_PASSED: u8 = 0;
const CSW_FAILED: u8 = 1;
const CSW_PHASE_ERROR: u8 = 2;

const SENSE_NONE: Sense = Sense::new(0, 0, 0);
const SENSE_INVALID_COMMAND: Sense = Sense::new(0x05, 0x20, 0);
const SENSE_INVALID_FIELD: Sense = Sense::new(0x05, 0x24, 0);
const SENSE_LBA_OUT_OF_RANGE: Sense = Sense::new(0x05, 0x21, 0);
const SENSE_WRITE_PROTECTED: Sense = Sense::new(0x07, 0x27, 0);

#[derive(Clone, Copy)]
struct Sense {
    key: u8,
    asc: u8,
    ascq: u8,
}

impl Sense {
    const fn new(key: u8, asc: u8, ascq: u8) -> Self {
        Self { key, asc, ascq }
    }
}

#[derive(Clone, Copy)]
enum BotState {
    Command,
    DiscardOut {
        remaining: u32,
        tag: u32,
        residue: u32,
        status: u8,
    },
    ResetRecovery,
}

#[derive(Clone, Copy)]
struct CommandBlockWrapper {
    tag: u32,
    transfer_length: u32,
    flags: u8,
    command_length: u8,
    command: [u8; 16],
}

impl CommandBlockWrapper {
    fn parse(packet: &[u8]) -> Option<Self> {
        if packet.len() != 31 || &packet[..4] != b"USBC" {
            return None;
        }
        let command_length = packet[14] & 0x1f;
        if packet[13] & 0x0f != 0 || packet[12] & 0x7f != 0 || !(1..=16).contains(&command_length) {
            return None;
        }

        let mut command = [0; 16];
        command.copy_from_slice(&packet[15..31]);
        Some(Self {
            tag: u32::from_le_bytes(packet[4..8].try_into().ok()?),
            transfer_length: u32::from_le_bytes(packet[8..12].try_into().ok()?),
            flags: packet[12],
            command_length,
            command,
        })
    }

    fn data_in(self) -> bool {
        self.flags & 0x80 != 0
    }

    fn has_command_bytes(self, count: u8) -> bool {
        self.command_length >= count
    }
}

/// Polling USB mass-storage device backed by a 512-byte sector reader.
pub struct UsbMassStorage {
    transport: UsbMassStorageTransport,
    state: BotState,
    sense: Sense,
    exit_requested: bool,
    block_count: u32,
    read_sector: ReadSector,
    sector: [u8; BLOCK_SIZE],
}

impl UsbMassStorage {
    /// Maps V821 USB0 after the BootROM hands the E907 to this payload.
    ///
    /// # Safety
    ///
    /// USB0, APP-CCU, and AON-CCU must be exclusively owned by this E907
    /// payload with interrupts disabled.
    pub unsafe fn from_v821_mmio(block_count: u32, read_sector: ReadSector) -> Self {
        assert!(block_count != 0);
        Self {
            // SAFETY: forwarded from this function's ownership contract.
            transport: unsafe { UsbMassStorageTransport::from_v821_mmio() },
            state: BotState::Command,
            sense: SENSE_NONE,
            exit_requested: false,
            block_count,
            read_sector,
            sector: [0; BLOCK_SIZE],
        }
    }

    pub fn initialize(&mut self) {
        self.state = BotState::Command;
        self.sense = SENSE_NONE;
        self.exit_requested = false;
        self.transport.initialize();
    }

    /// Services one USB packet and reports a host-requested safe eject.
    pub fn poll(&mut self) -> bool {
        if self.transport.take_class_reset() {
            self.state = BotState::Command;
            self.sense = SENSE_NONE;
            self.exit_requested = false;
        } else if !self.transport.is_configured() {
            self.state = BotState::Command;
        }

        let mut packet = [0; 64];
        let count = self.transport.poll(&mut packet);
        if count == 0 {
            return false;
        }

        match self.state {
            BotState::Command => {
                let Some(cbw) = CommandBlockWrapper::parse(&packet[..count]) else {
                    self.transport.stall_bulk_in(true);
                    self.state = BotState::ResetRecovery;
                    return false;
                };
                self.handle_command(cbw);
            }
            BotState::DiscardOut {
                remaining,
                tag,
                residue,
                status,
            } => {
                if count as u32 > remaining {
                    self.transport.stall_bulk_in(true);
                    self.state = BotState::ResetRecovery;
                    return false;
                }
                let remaining = remaining - count as u32;
                if remaining == 0 {
                    self.state = BotState::Command;
                    self.send_csw(tag, residue, status);
                } else {
                    self.state = BotState::DiscardOut {
                        remaining,
                        tag,
                        residue,
                        status,
                    };
                }
            }
            BotState::ResetRecovery => {}
        }

        if self.exit_requested {
            self.exit_requested = self.transport.flush();
        }
        self.exit_requested
    }

    fn handle_command(&mut self, cbw: CommandBlockWrapper) {
        if cbw.command[0] != 0x03 {
            self.sense = SENSE_NONE;
        }
        match cbw.command[0] {
            0x00 if cbw.has_command_bytes(6) => self.send_no_data(cbw), // TEST UNIT READY
            0x03 if cbw.has_command_bytes(6) => self.request_sense(cbw),
            0x08 if cbw.has_command_bytes(6) => self.read_6(cbw),
            0x12 if cbw.has_command_bytes(6) => self.inquiry(cbw),
            0x1a if cbw.has_command_bytes(6) => self.mode_sense_6(cbw),
            0x1b if cbw.has_command_bytes(6) => self.start_stop_unit(cbw),
            0x1e if cbw.has_command_bytes(6) => self.send_no_data(cbw), // PREVENT/ALLOW
            0x23 if cbw.has_command_bytes(10) => self.read_format_capacities(cbw),
            0x25 if cbw.has_command_bytes(10) => self.read_capacity_10(cbw),
            0x28 if cbw.has_command_bytes(10) => self.read_10(cbw),
            0x2f if cbw.has_command_bytes(10) && cbw.command[1] & 0x02 == 0 => {
                self.send_no_data(cbw) // VERIFY(10), without BYTCHK
            }
            0x35 if cbw.has_command_bytes(10) => self.send_no_data(cbw), // SYNCHRONIZE CACHE
            0x5a if cbw.has_command_bytes(10) => self.mode_sense_10(cbw),
            0x88 if cbw.has_command_bytes(16) => self.read_16(cbw),
            0x9e if cbw.has_command_bytes(16) && cbw.command[1] & 0x1f == 0x10 => {
                self.read_capacity_16(cbw)
            }
            0xa0 if cbw.has_command_bytes(12) => self.report_luns(cbw),
            0xa8 if cbw.has_command_bytes(12) => self.read_12(cbw),
            0x04 | 0x0a | 0x15 | 0x2a | 0x55 | 0x8a | 0xaa => {
                self.fail_command(cbw, SENSE_WRITE_PROTECTED, CSW_FAILED)
            }
            _ if !cbw.has_command_bytes(Self::minimum_command_length(cbw.command[0])) => {
                self.fail_command(cbw, SENSE_INVALID_FIELD, CSW_FAILED)
            }
            _ => self.fail_command(cbw, SENSE_INVALID_COMMAND, CSW_FAILED),
        }
    }

    const fn minimum_command_length(opcode: u8) -> u8 {
        match opcode >> 5 {
            0 => 6,
            1 | 2 => 10,
            4 => 16,
            5 => 12,
            _ => 1,
        }
    }

    fn inquiry(&mut self, cbw: CommandBlockWrapper) {
        let mut reply = [0; 64];
        let length = match (cbw.command[1] & 1 != 0, cbw.command[2]) {
            (false, 0) => {
                reply[0] = 0;
                reply[1] = 0x80;
                reply[2] = 0x04;
                reply[3] = 0x02;
                reply[4] = 31;
                reply[8..16].copy_from_slice(b"Avaota  ");
                reply[16..32].copy_from_slice(b"F1              ");
                reply[32..36].copy_from_slice(b"1.00");
                36
            }
            (true, 0x00) => {
                reply[..7].copy_from_slice(&[0, 0, 0, 3, 0, 0x80, 0x83]);
                7
            }
            (true, 0x80) => {
                reply[..4].copy_from_slice(&[0, 0x80, 0, 12]);
                reply[4..16].copy_from_slice(b"0821F1000001");
                16
            }
            (true, 0x83) => {
                reply[..8].copy_from_slice(&[0, 0x83, 0, 12, 0x02, 0x01, 0, 8]);
                reply[8..16].copy_from_slice(b"AVAOTAF1");
                16
            }
            _ => {
                self.fail_command(cbw, SENSE_INVALID_FIELD, CSW_FAILED);
                return;
            }
        };
        let allocation = cbw.command[4] as usize;
        self.send_reply(cbw, &reply[..length.min(allocation)]);
    }

    fn request_sense(&mut self, cbw: CommandBlockWrapper) {
        let mut reply = [0; 18];
        reply[0] = 0x70;
        reply[2] = self.sense.key;
        reply[7] = 10;
        reply[12] = self.sense.asc;
        reply[13] = self.sense.ascq;
        let allocation = cbw.command[4] as usize;
        self.send_reply(cbw, &reply[..reply.len().min(allocation)]);
        self.sense = SENSE_NONE;
    }

    fn read_capacity_10(&mut self, cbw: CommandBlockWrapper) {
        let mut reply = [0; 8];
        reply[..4].copy_from_slice(&(self.block_count - 1).to_be_bytes());
        reply[4..].copy_from_slice(&(BLOCK_SIZE as u32).to_be_bytes());
        self.send_reply(cbw, &reply);
    }

    fn read_capacity_16(&mut self, cbw: CommandBlockWrapper) {
        let mut reply = [0; 32];
        reply[..8].copy_from_slice(&u64::from(self.block_count - 1).to_be_bytes());
        reply[8..12].copy_from_slice(&(BLOCK_SIZE as u32).to_be_bytes());
        let allocation = u32::from_be_bytes(cbw.command[10..14].try_into().unwrap()) as usize;
        self.send_reply(cbw, &reply[..reply.len().min(allocation)]);
    }

    fn read_format_capacities(&mut self, cbw: CommandBlockWrapper) {
        let mut reply = [0; 12];
        reply[3] = 8;
        reply[4..8].copy_from_slice(&self.block_count.to_be_bytes());
        reply[8] = 0x02;
        reply[9..12].copy_from_slice(&[0, 2, 0]);
        let allocation = u16::from_be_bytes(cbw.command[7..9].try_into().unwrap()) as usize;
        self.send_reply(cbw, &reply[..reply.len().min(allocation)]);
    }

    fn mode_sense_6(&mut self, cbw: CommandBlockWrapper) {
        let reply = [3, 0, 0x80, 0];
        let allocation = cbw.command[4] as usize;
        self.send_reply(cbw, &reply[..reply.len().min(allocation)]);
    }

    fn mode_sense_10(&mut self, cbw: CommandBlockWrapper) {
        let reply = [0, 6, 0, 0x80, 0, 0, 0, 0];
        let allocation = u16::from_be_bytes(cbw.command[7..9].try_into().unwrap()) as usize;
        self.send_reply(cbw, &reply[..reply.len().min(allocation)]);
    }

    fn report_luns(&mut self, cbw: CommandBlockWrapper) {
        let mut reply = [0; 16];
        reply[3] = 8;
        let allocation = u32::from_be_bytes(cbw.command[6..10].try_into().unwrap()) as usize;
        self.send_reply(cbw, &reply[..reply.len().min(allocation)]);
    }

    fn start_stop_unit(&mut self, cbw: CommandBlockWrapper) {
        // A normal unplug or bus reset never reaches this path.  Windows sends
        // LOEJ=1, START=0 only after the user safely ejects the volume.
        let eject = cbw.command[4] & 0x03 == 0x02;
        self.send_no_data(cbw);
        if cbw.transfer_length == 0 {
            self.exit_requested = eject;
        }
    }

    fn read_6(&mut self, cbw: CommandBlockWrapper) {
        let lba = u32::from(cbw.command[1] & 0x1f) << 16
            | u32::from(cbw.command[2]) << 8
            | u32::from(cbw.command[3]);
        let blocks = if cbw.command[4] == 0 {
            256
        } else {
            u32::from(cbw.command[4])
        };
        self.read_blocks(cbw, u64::from(lba), blocks);
    }

    fn read_10(&mut self, cbw: CommandBlockWrapper) {
        let lba = u32::from_be_bytes(cbw.command[2..6].try_into().unwrap());
        let blocks = u16::from_be_bytes(cbw.command[7..9].try_into().unwrap());
        self.read_blocks(cbw, u64::from(lba), u32::from(blocks));
    }

    fn read_12(&mut self, cbw: CommandBlockWrapper) {
        let lba = u32::from_be_bytes(cbw.command[2..6].try_into().unwrap());
        let blocks = u32::from_be_bytes(cbw.command[6..10].try_into().unwrap());
        self.read_blocks(cbw, u64::from(lba), blocks);
    }

    fn read_16(&mut self, cbw: CommandBlockWrapper) {
        let lba = u64::from_be_bytes(cbw.command[2..10].try_into().unwrap());
        let blocks = u32::from_be_bytes(cbw.command[10..14].try_into().unwrap());
        self.read_blocks(cbw, lba, blocks);
    }

    fn read_blocks(&mut self, cbw: CommandBlockWrapper, lba: u64, blocks: u32) {
        let Some(end) = lba.checked_add(u64::from(blocks)) else {
            self.fail_command(cbw, SENSE_LBA_OUT_OF_RANGE, CSW_FAILED);
            return;
        };
        if end > u64::from(self.block_count) {
            self.fail_command(cbw, SENSE_LBA_OUT_OF_RANGE, CSW_FAILED);
            return;
        }
        if cbw.transfer_length != 0 && !cbw.data_in() {
            self.fail_command(cbw, SENSE_INVALID_FIELD, CSW_PHASE_ERROR);
            return;
        }

        let available = u64::from(blocks) * BLOCK_SIZE as u64;
        let mut remaining = u64::from(cbw.transfer_length).min(available) as usize;
        let mut current = lba as u32;
        let mut sent = 0_u32;
        while remaining != 0 {
            (self.read_sector)(current, &mut self.sector);
            let count = remaining.min(BLOCK_SIZE);
            self.transport.write(&self.sector[..count]);
            remaining -= count;
            sent += count as u32;
            current += 1;
        }
        let status = if u64::from(cbw.transfer_length) < available {
            CSW_PHASE_ERROR
        } else {
            CSW_PASSED
        };
        self.finish_data_in(cbw, sent, status);
    }

    fn send_reply(&mut self, cbw: CommandBlockWrapper, reply: &[u8]) {
        if cbw.transfer_length != 0 && !cbw.data_in() {
            self.fail_command(cbw, SENSE_INVALID_FIELD, CSW_PHASE_ERROR);
            return;
        }
        let count = reply.len().min(cbw.transfer_length as usize);
        self.transport.write(&reply[..count]);
        let status = if cbw.transfer_length < reply.len() as u32 {
            CSW_PHASE_ERROR
        } else {
            CSW_PASSED
        };
        self.finish_data_in(cbw, count as u32, status);
    }

    fn finish_data_in(&mut self, cbw: CommandBlockWrapper, sent: u32, status: u8) {
        let residue = cbw.transfer_length - sent;
        if residue != 0 {
            if sent.is_multiple_of(64) {
                self.transport.write_zero_length_packet();
            }
            if !self.transport.flush() {
                return;
            }
            self.transport.stall_bulk_in(false);
        }
        self.send_csw(cbw.tag, residue, status);
    }

    fn send_no_data(&mut self, cbw: CommandBlockWrapper) {
        if cbw.transfer_length == 0 {
            self.send_csw(cbw.tag, 0, CSW_PASSED);
        } else if cbw.data_in() {
            self.finish_data_in(cbw, 0, CSW_PASSED);
        } else {
            self.state = BotState::DiscardOut {
                remaining: cbw.transfer_length,
                tag: cbw.tag,
                residue: cbw.transfer_length,
                status: CSW_PASSED,
            };
        }
    }

    fn fail_command(&mut self, cbw: CommandBlockWrapper, sense: Sense, status: u8) {
        self.sense = sense;
        if cbw.transfer_length != 0 && !cbw.data_in() {
            self.state = BotState::DiscardOut {
                remaining: cbw.transfer_length,
                tag: cbw.tag,
                residue: cbw.transfer_length,
                status,
            };
        } else {
            self.finish_data_in(cbw, 0, status);
        }
    }

    fn send_csw(&mut self, tag: u32, residue: u32, status: u8) {
        let mut csw = [0; 13];
        csw[..4].copy_from_slice(b"USBS");
        csw[4..8].copy_from_slice(&tag.to_le_bytes());
        csw[8..12].copy_from_slice(&residue.to_le_bytes());
        csw[12] = status;
        self.transport.write(&csw);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_command_block_wrapper() {
        let mut packet = [0; 31];
        packet[..4].copy_from_slice(b"USBC");
        packet[4..8].copy_from_slice(&0x1234_5678_u32.to_le_bytes());
        packet[8..12].copy_from_slice(&512_u32.to_le_bytes());
        packet[12] = 0x80;
        packet[14] = 10;
        packet[15] = 0x28;
        let cbw = CommandBlockWrapper::parse(&packet).unwrap();
        assert_eq!(cbw.tag, 0x1234_5678);
        assert_eq!(cbw.transfer_length, 512);
        assert!(cbw.data_in());
        assert_eq!(cbw.command[0], 0x28);
    }
}
