//! Minimal read-only SCSI/Bulk-Only mass-storage class.

use usb_device::{
    UsbDirection, UsbError,
    bus::{InterfaceNumber, UsbBus, UsbBusAllocator},
    class::{ControlIn, ControlOut, UsbClass},
    control::{Recipient, RequestType},
    descriptor::DescriptorWriter,
    endpoint::{EndpointAddress, EndpointIn, EndpointOut},
};

pub const BLOCK_SIZE: usize = 512;

const CLASS_MASS_STORAGE: u8 = 0x08;
const SUBCLASS_SCSI: u8 = 0x06;
const PROTOCOL_BULK_ONLY: u8 = 0x50;
const GET_MAX_LUN: u8 = 0xfe;
const BULK_ONLY_RESET: u8 = 0xff;
const BULK_PACKET_SIZE: usize = 64;

const SENSE_NONE: Sense = Sense::new(0, 0, 0);
const SENSE_INVALID_COMMAND: Sense = Sense::new(0x05, 0x20, 0);
const SENSE_INVALID_FIELD: Sense = Sense::new(0x05, 0x24, 0);
const SENSE_LBA_OUT_OF_RANGE: Sense = Sense::new(0x05, 0x21, 0);
const SENSE_WRITE_PROTECTED: Sense = Sense::new(0x07, 0x27, 0);

pub type ReadSector = fn(u32, &mut [u8; BLOCK_SIZE]);

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

#[repr(u8)]
#[derive(Clone, Copy)]
enum CswStatus {
    Passed = 0,
    Failed = 1,
    PhaseError = 2,
}

#[derive(Clone, Copy)]
struct Csw {
    tag: u32,
    residue: u32,
    status: CswStatus,
}

impl Csw {
    fn bytes(self) -> [u8; 13] {
        let mut bytes = [0; 13];
        bytes[..4].copy_from_slice(b"USBS");
        bytes[4..8].copy_from_slice(&self.tag.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.residue.to_le_bytes());
        bytes[12] = self.status as u8;
        bytes
    }
}

#[derive(Clone, Copy)]
enum BotState {
    Command,
    DataIn {
        remaining: u32,
        csw: Csw,
        stall_after_data: bool,
    },
    DiscardOut {
        remaining: u32,
        csw: Csw,
    },
    StallInPending(Csw),
    WaitClearIn(Csw),
    SendCsw(Csw),
    CswPending,
    ResetRecovery,
}

#[derive(Clone, Copy)]
enum ScsiState {
    Idle,
    Reply {
        length: u8,
        offset: u8,
    },
    Read {
        next_lba: u32,
        blocks: u32,
        offset: u16,
        loaded: bool,
    },
}

#[derive(Clone, Copy)]
struct CommandOutcome {
    data_length: u32,
    status: CswStatus,
}

impl CommandOutcome {
    const fn passed(data_length: u32) -> Self {
        Self {
            data_length,
            status: CswStatus::Passed,
        }
    }

    const fn failed() -> Self {
        Self {
            data_length: 0,
            status: CswStatus::Failed,
        }
    }
}

struct CommandBlockWrapper {
    tag: u32,
    transfer_length: u32,
    data_in: bool,
    command_length: usize,
    command: [u8; 16],
}

impl CommandBlockWrapper {
    fn parse(packet: &[u8]) -> Option<Self> {
        if packet.len() != 31
            || &packet[..4] != b"USBC"
            || packet[12] & 0x7f != 0
            // This class advertises bMaxLUN = 0, so any nonzero bCBWLUN makes
            // the CBW invalid rather than an unsupported SCSI command.
            || packet[13] != 0
            || packet[14] & 0xe0 != 0
            || !(1..=16).contains(&packet[14])
        {
            return None;
        }

        let mut command = [0; 16];
        command.copy_from_slice(&packet[15..31]);
        Some(Self {
            tag: u32::from_le_bytes(packet[4..8].try_into().ok()?),
            transfer_length: u32::from_le_bytes(packet[8..12].try_into().ok()?),
            data_in: packet[12] & 0x80 != 0,
            command_length: usize::from(packet[14]),
            command,
        })
    }
}

/// A single-LUN, read-only USB mass-storage class.
pub struct UsbMassStorage<'a, B: UsbBus> {
    interface: InterfaceNumber,
    bulk_in: EndpointIn<'a, B>,
    bulk_out: EndpointOut<'a, B>,
    bot: BotState,
    scsi: ScsiState,
    sense: Sense,
    eject_after_csw: bool,
    eject_ready: bool,
    block_count: u32,
    read_sector: ReadSector,
    reply: [u8; 64],
    sector: [u8; BLOCK_SIZE],
}

impl<'a, B: UsbBus + 'a> UsbMassStorage<'a, B> {
    pub fn new(alloc: &'a UsbBusAllocator<B>, block_count: u32, read_sector: ReadSector) -> Self {
        assert!(block_count != 0);
        Self {
            interface: alloc.interface(),
            bulk_in: alloc.bulk(BULK_PACKET_SIZE as u16),
            bulk_out: alloc.bulk(BULK_PACKET_SIZE as u16),
            bot: BotState::Command,
            scsi: ScsiState::Idle,
            sense: SENSE_NONE,
            eject_after_csw: false,
            eject_ready: false,
            block_count,
            read_sector,
            reply: [0; 64],
            sector: [0; BLOCK_SIZE],
        }
    }

    /// Advances at most one bulk packet without waiting for an endpoint.
    ///
    /// Returns `true` only after the host's safe-eject CSW has completed on the
    /// bulk IN endpoint.
    pub fn poll(&mut self) -> bool {
        self.advance();
        core::mem::take(&mut self.eject_ready)
    }

    fn advance(&mut self) {
        match self.bot {
            BotState::Command => self.read_cbw(),
            BotState::DataIn {
                remaining,
                csw,
                stall_after_data,
            } => self.send_data(remaining, csw, stall_after_data),
            BotState::DiscardOut { remaining, csw } => self.discard_out(remaining, csw),
            BotState::SendCsw(csw) => match self.bulk_in.write(&csw.bytes()) {
                Ok(13) => self.bot = BotState::CswPending,
                Err(UsbError::WouldBlock) => {}
                _ => self.enter_reset_recovery(),
            },
            BotState::StallInPending(_)
            | BotState::WaitClearIn(_)
            | BotState::CswPending
            | BotState::ResetRecovery => {}
        }
    }

    fn read_cbw(&mut self) {
        let mut packet = [0; BULK_PACKET_SIZE];
        match self.bulk_out.read(&mut packet) {
            Ok(count) => {
                let Some(cbw) = CommandBlockWrapper::parse(&packet[..count]) else {
                    self.enter_reset_recovery();
                    return;
                };
                self.start_cbw(cbw);
            }
            Err(UsbError::WouldBlock) => {}
            Err(_) => self.enter_reset_recovery(),
        }
    }

    fn start_cbw(&mut self, cbw: CommandBlockWrapper) {
        self.scsi = ScsiState::Idle;
        self.eject_after_csw = false;
        let command = cbw.command;
        let outcome = self.start_command(&command[..cbw.command_length]);
        let mut csw = Csw {
            tag: cbw.tag,
            residue: cbw.transfer_length,
            status: outcome.status,
        };

        if outcome.data_length == 0 {
            self.scsi = ScsiState::Idle;
            self.bot = if cbw.transfer_length == 0 {
                BotState::SendCsw(csw)
            } else if cbw.data_in {
                self.bulk_in.stall();
                BotState::WaitClearIn(csw)
            } else {
                BotState::DiscardOut {
                    remaining: cbw.transfer_length,
                    csw,
                }
            };
            return;
        }

        if cbw.transfer_length == 0 {
            self.scsi = ScsiState::Idle;
            csw.status = CswStatus::PhaseError;
            self.bot = BotState::SendCsw(csw);
        } else if !cbw.data_in {
            self.scsi = ScsiState::Idle;
            csw.status = CswStatus::PhaseError;
            self.bot = BotState::DiscardOut {
                remaining: cbw.transfer_length,
                csw,
            };
        } else {
            let send = cbw.transfer_length.min(outcome.data_length);
            csw.residue = cbw.transfer_length - send;
            if cbw.transfer_length < outcome.data_length {
                csw.status = CswStatus::PhaseError;
            }
            self.bot = BotState::DataIn {
                remaining: send,
                csw,
                stall_after_data: cbw.transfer_length > send,
            };
        }
    }

    fn send_data(&mut self, remaining: u32, csw: Csw, stall_after_data: bool) {
        if remaining == 0 {
            self.scsi = ScsiState::Idle;
            self.bot = if stall_after_data {
                BotState::StallInPending(csw)
            } else {
                BotState::SendCsw(csw)
            };
            return;
        }

        let count = remaining.min(BULK_PACKET_SIZE as u32) as usize;
        match self.write_scsi_packet(count) {
            Ok(written) if written != 0 => {
                let remaining = remaining - written as u32;
                if remaining == 0 {
                    self.scsi = ScsiState::Idle;
                    self.bot = if stall_after_data {
                        BotState::StallInPending(csw)
                    } else {
                        BotState::SendCsw(csw)
                    };
                } else {
                    self.bot = BotState::DataIn {
                        remaining,
                        csw,
                        stall_after_data,
                    };
                }
            }
            Err(UsbError::WouldBlock) => {}
            _ => self.enter_reset_recovery(),
        }
    }

    fn write_scsi_packet(&mut self, count: usize) -> Result<usize, UsbError> {
        match self.scsi {
            ScsiState::Idle => Err(UsbError::InvalidState),
            ScsiState::Reply { length, offset } => {
                let start = usize::from(offset);
                let end = (start + count).min(usize::from(length));
                if start == end {
                    return Err(UsbError::InvalidState);
                }
                let written = self.bulk_in.write(&self.reply[start..end])?;
                self.scsi = ScsiState::Reply {
                    length,
                    offset: offset + written as u8,
                };
                Ok(written)
            }
            ScsiState::Read {
                next_lba,
                blocks,
                offset,
                loaded,
            } => {
                if blocks == 0 {
                    return Err(UsbError::InvalidState);
                }
                if !loaded {
                    (self.read_sector)(next_lba, &mut self.sector);
                    self.scsi = ScsiState::Read {
                        next_lba,
                        blocks,
                        offset,
                        loaded: true,
                    };
                }
                let start = usize::from(offset);
                let end = (start + count).min(BLOCK_SIZE);
                let written = self.bulk_in.write(&self.sector[start..end])?;
                let offset = offset + written as u16;
                self.scsi = if usize::from(offset) == BLOCK_SIZE {
                    if blocks == 1 {
                        ScsiState::Idle
                    } else {
                        ScsiState::Read {
                            next_lba: next_lba + 1,
                            blocks: blocks - 1,
                            offset: 0,
                            loaded: false,
                        }
                    }
                } else {
                    ScsiState::Read {
                        next_lba,
                        blocks,
                        offset,
                        loaded: true,
                    }
                };
                Ok(written)
            }
        }
    }

    fn discard_out(&mut self, remaining: u32, mut csw: Csw) {
        let mut packet = [0; BULK_PACKET_SIZE];
        match self.bulk_out.read(&mut packet) {
            Ok(count) if count as u32 <= remaining => {
                let remaining = remaining - count as u32;
                if remaining == 0 {
                    self.bot = BotState::SendCsw(csw);
                } else if count < BULK_PACKET_SIZE {
                    csw.status = CswStatus::PhaseError;
                    self.bot = BotState::SendCsw(csw);
                } else {
                    self.bot = BotState::DiscardOut { remaining, csw };
                }
            }
            Err(UsbError::WouldBlock) => {}
            _ => self.enter_reset_recovery(),
        }
    }

    fn enter_reset_recovery(&mut self) {
        self.bulk_in.stall();
        self.bulk_out.stall();
        self.clear_transfer_state();
        self.bot = BotState::ResetRecovery;
    }

    fn begin_bulk_only_reset(&mut self) {
        // Preserve the endpoints' current HALT conditions. In reset recovery
        // they remain halted until the host clears them; a reset received in
        // normal operation must not introduce a new halt.
        self.clear_transfer_state();
        self.bot = BotState::Command;
    }

    fn reset_bus(&mut self) {
        // UsbBus::reset has already flushed endpoints and reset their toggles.
        self.clear_transfer_state();
        self.bot = BotState::Command;
    }

    fn clear_transfer_state(&mut self) {
        self.scsi = ScsiState::Idle;
        self.sense = SENSE_NONE;
        self.eject_after_csw = false;
        self.eject_ready = false;
        self.reply.fill(0);
        self.sector.fill(0);
    }

    fn start_command(&mut self, command: &[u8]) -> CommandOutcome {
        let opcode = command[0];
        if opcode != 0x03 {
            self.sense = SENSE_NONE;
        }
        match opcode {
            0x00 if command.len() >= 6 => CommandOutcome::passed(0),
            0x03 if command.len() >= 6 => self.request_sense(command),
            0x08 if command.len() >= 6 => self.read_6(command),
            0x12 if command.len() >= 6 => self.inquiry(command),
            0x1a if command.len() >= 6 => self.mode_sense_6(command),
            0x1b if command.len() >= 6 => self.start_stop_unit(command),
            0x1e if command.len() >= 6 => CommandOutcome::passed(0),
            0x23 if command.len() >= 10 => self.read_format_capacities(command),
            0x25 if command.len() >= 10 => self.read_capacity_10(),
            0x28 if command.len() >= 10 => self.read_10(command),
            0x2f if command.len() >= 10 && command[1] & 0x02 == 0 => CommandOutcome::passed(0),
            0x35 if command.len() >= 10 => CommandOutcome::passed(0),
            0x5a if command.len() >= 10 => self.mode_sense_10(command),
            0x88 if command.len() >= 16 => self.read_16(command),
            0x9e if command.len() >= 16 && command[1] & 0x1f == 0x10 => {
                self.read_capacity_16(command)
            }
            0xa0 if command.len() >= 12 => self.report_luns(command),
            0xa8 if command.len() >= 12 => self.read_12(command),
            0x04 | 0x0a | 0x15 | 0x2a | 0x55 | 0x8a | 0xaa => self.fail(SENSE_WRITE_PROTECTED),
            _ if command.len() < minimum_command_length(opcode) => self.fail(SENSE_INVALID_FIELD),
            _ => self.fail(SENSE_INVALID_COMMAND),
        }
    }

    fn inquiry(&mut self, command: &[u8]) -> CommandOutcome {
        self.reply.fill(0);
        let length = match (command[1] & 1 != 0, command[2]) {
            (false, 0) => {
                self.reply[0] = 0;
                self.reply[1] = 0x80;
                self.reply[2] = 0x04;
                self.reply[3] = 0x02;
                self.reply[4] = 31;
                self.reply[8..16].copy_from_slice(b"Avaota  ");
                self.reply[16..32].copy_from_slice(b"F1              ");
                self.reply[32..36].copy_from_slice(b"1.00");
                36
            }
            (true, 0x00) => {
                self.reply[..7].copy_from_slice(&[0, 0, 0, 3, 0, 0x80, 0x83]);
                7
            }
            (true, 0x80) => {
                self.reply[..4].copy_from_slice(&[0, 0x80, 0, 12]);
                self.reply[4..16].copy_from_slice(b"0821F1000001");
                16
            }
            (true, 0x83) => {
                self.reply[..8].copy_from_slice(&[0, 0x83, 0, 12, 0x02, 0x01, 0, 8]);
                self.reply[8..16].copy_from_slice(b"AVAOTAF1");
                16
            }
            _ => return self.fail(SENSE_INVALID_FIELD),
        };
        self.queue_reply(length.min(usize::from(command[4])))
    }

    fn request_sense(&mut self, command: &[u8]) -> CommandOutcome {
        self.reply.fill(0);
        self.reply[0] = 0x70;
        self.reply[2] = self.sense.key;
        self.reply[7] = 10;
        self.reply[12] = self.sense.asc;
        self.reply[13] = self.sense.ascq;
        let outcome = self.queue_reply(18.min(usize::from(command[4])));
        self.sense = SENSE_NONE;
        outcome
    }

    fn read_capacity_10(&mut self) -> CommandOutcome {
        self.reply[..4].copy_from_slice(&(self.block_count - 1).to_be_bytes());
        self.reply[4..8].copy_from_slice(&(BLOCK_SIZE as u32).to_be_bytes());
        self.queue_reply(8)
    }

    fn read_capacity_16(&mut self, command: &[u8]) -> CommandOutcome {
        self.reply.fill(0);
        self.reply[..8].copy_from_slice(&u64::from(self.block_count - 1).to_be_bytes());
        self.reply[8..12].copy_from_slice(&(BLOCK_SIZE as u32).to_be_bytes());
        let allocation = u32::from_be_bytes(command[10..14].try_into().unwrap()) as usize;
        self.queue_reply(32.min(allocation))
    }

    fn read_format_capacities(&mut self, command: &[u8]) -> CommandOutcome {
        self.reply[..12].fill(0);
        self.reply[3] = 8;
        self.reply[4..8].copy_from_slice(&self.block_count.to_be_bytes());
        self.reply[8] = 0x02;
        self.reply[9..12].copy_from_slice(&[0, 2, 0]);
        let allocation = u16::from_be_bytes(command[7..9].try_into().unwrap()) as usize;
        self.queue_reply(12.min(allocation))
    }

    fn mode_sense_6(&mut self, command: &[u8]) -> CommandOutcome {
        self.reply[..4].copy_from_slice(&[3, 0, 0x80, 0]);
        self.queue_reply(4.min(usize::from(command[4])))
    }

    fn mode_sense_10(&mut self, command: &[u8]) -> CommandOutcome {
        self.reply[..8].copy_from_slice(&[0, 6, 0, 0x80, 0, 0, 0, 0]);
        let allocation = u16::from_be_bytes(command[7..9].try_into().unwrap()) as usize;
        self.queue_reply(8.min(allocation))
    }

    fn report_luns(&mut self, command: &[u8]) -> CommandOutcome {
        self.reply[..16].fill(0);
        self.reply[3] = 8;
        let allocation = u32::from_be_bytes(command[6..10].try_into().unwrap()) as usize;
        self.queue_reply(16.min(allocation))
    }

    fn start_stop_unit(&mut self, command: &[u8]) -> CommandOutcome {
        self.eject_after_csw = command[4] & 0x03 == 0x02;
        CommandOutcome::passed(0)
    }

    fn read_6(&mut self, command: &[u8]) -> CommandOutcome {
        let lba =
            u32::from(command[1] & 0x1f) << 16 | u32::from(command[2]) << 8 | u32::from(command[3]);
        let blocks = if command[4] == 0 {
            256
        } else {
            u32::from(command[4])
        };
        self.read_blocks(u64::from(lba), blocks)
    }

    fn read_10(&mut self, command: &[u8]) -> CommandOutcome {
        self.read_blocks(
            u64::from(u32::from_be_bytes(command[2..6].try_into().unwrap())),
            u32::from(u16::from_be_bytes(command[7..9].try_into().unwrap())),
        )
    }

    fn read_12(&mut self, command: &[u8]) -> CommandOutcome {
        self.read_blocks(
            u64::from(u32::from_be_bytes(command[2..6].try_into().unwrap())),
            u32::from_be_bytes(command[6..10].try_into().unwrap()),
        )
    }

    fn read_16(&mut self, command: &[u8]) -> CommandOutcome {
        self.read_blocks(
            u64::from_be_bytes(command[2..10].try_into().unwrap()),
            u32::from_be_bytes(command[10..14].try_into().unwrap()),
        )
    }

    fn read_blocks(&mut self, lba: u64, blocks: u32) -> CommandOutcome {
        let Some(end) = lba.checked_add(u64::from(blocks)) else {
            return self.fail(SENSE_LBA_OUT_OF_RANGE);
        };
        let Some(length) = blocks.checked_mul(BLOCK_SIZE as u32) else {
            return self.fail(SENSE_INVALID_FIELD);
        };
        if end > u64::from(self.block_count) {
            self.fail(SENSE_LBA_OUT_OF_RANGE)
        } else if blocks == 0 {
            CommandOutcome::passed(0)
        } else {
            self.scsi = ScsiState::Read {
                next_lba: lba as u32,
                blocks,
                offset: 0,
                loaded: false,
            };
            CommandOutcome::passed(length)
        }
    }

    fn queue_reply(&mut self, length: usize) -> CommandOutcome {
        self.scsi = if length == 0 {
            ScsiState::Idle
        } else {
            ScsiState::Reply {
                length: length as u8,
                offset: 0,
            }
        };
        CommandOutcome::passed(length as u32)
    }

    fn fail(&mut self, sense: Sense) -> CommandOutcome {
        self.sense = sense;
        self.scsi = ScsiState::Idle;
        CommandOutcome::failed()
    }

    fn is_interface_request(
        &self,
        request_type: RequestType,
        recipient: Recipient,
        index: u16,
    ) -> bool {
        request_type == RequestType::Class
            && recipient == Recipient::Interface
            && index == u16::from(u8::from(self.interface))
    }
}

impl<B: UsbBus> UsbClass<B> for UsbMassStorage<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<(), UsbError> {
        writer.interface(
            self.interface,
            CLASS_MASS_STORAGE,
            SUBCLASS_SCSI,
            PROTOCOL_BULK_ONLY,
        )?;
        writer.endpoint(&self.bulk_in)?;
        writer.endpoint(&self.bulk_out)
    }

    fn reset(&mut self) {
        self.reset_bus();
    }

    fn poll(&mut self) {
        self.advance();
    }

    fn endpoint_in_complete(&mut self, address: EndpointAddress) {
        if address == self.bulk_in.address() {
            match self.bot {
                BotState::StallInPending(csw) => {
                    self.bulk_in.stall();
                    self.bot = BotState::WaitClearIn(csw);
                }
                BotState::CswPending => {
                    self.bot = BotState::Command;
                    self.scsi = ScsiState::Idle;
                    if self.eject_after_csw {
                        self.eject_after_csw = false;
                        self.eject_ready = true;
                    }
                }
                _ => {}
            }
        }
    }

    fn control_in(&mut self, transfer: ControlIn<B>) {
        let request = *transfer.request();
        if !self.is_interface_request(request.request_type, request.recipient, request.index) {
            return;
        }
        if request.direction == UsbDirection::In
            && request.request == GET_MAX_LUN
            && request.value == 0
            && request.length == 1
        {
            let _ = transfer.accept_with(&[0]);
        } else {
            let _ = transfer.reject();
        }
    }

    fn control_out(&mut self, transfer: ControlOut<B>) {
        let request = *transfer.request();
        let clear_halt = request.direction == UsbDirection::Out
            && request.request_type == RequestType::Standard
            && request.recipient == Recipient::Endpoint
            && request.request == usb_device::control::Request::CLEAR_FEATURE
            && request.value == usb_device::control::Request::FEATURE_ENDPOINT_HALT
            && request.length == 0;
        if let BotState::WaitClearIn(csw) = self.bot
            && clear_halt
            && request.index == u16::from(u8::from(self.bulk_in.address()))
        {
            // Leave the standard request to UsbDevice. It clears the halt and
            // data toggle after this callback; the following class poll then
            // queues the CSW on the recovered endpoint.
            self.bot = BotState::SendCsw(csw);
            return;
        }
        if !self.is_interface_request(request.request_type, request.recipient, request.index) {
            return;
        }
        if request.direction == UsbDirection::Out
            && request.request == BULK_ONLY_RESET
            && request.value == 0
            && request.length == 0
            && transfer.data().is_empty()
        {
            self.begin_bulk_only_reset();
            let _ = transfer.accept();
        } else {
            let _ = transfer.reject();
        }
    }
}

const fn minimum_command_length(opcode: u8) -> usize {
    match opcode >> 5 {
        0 => 6,
        1 | 2 => 10,
        4 => 16,
        5 => 12,
        _ => 1,
    }
}
