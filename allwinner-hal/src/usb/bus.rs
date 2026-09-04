use core::cell::RefCell;

use critical_section::Mutex;
use usb_device::{
    UsbDirection, UsbError,
    bus::{PollResult, UsbBus as UsbBusTrait},
    endpoint::{EndpointAddress, EndpointType},
};

use super::{
    Usb,
    peripheral::acknowledge_pending,
    register::usb::{
        BusInterruptEnable, EndpointIndex, EndpointZeroControlStatus, FifoAddress, FifoSize,
        FunctionAddress, MaximumPacketSize, ReceiveControlStatus, ReceiveInterruptEnable,
        TransmitControlStatus, TransmitInterruptEnable,
    },
};

// V821 validation currently covers the fixed control endpoint plus EP1..EP3.
const ENDPOINT_COUNT: usize = 4;
// Reserve the first 512 bytes of packet RAM for the fixed endpoint-zero FIFO.
const DYNAMIC_FIFO_START: u16 = 0x0200;
// Exclusive end of the controller's 4 KiB packet RAM.
const DYNAMIC_FIFO_END: u16 = 0x1000;
// Each allocated direction uses the board-verified single 512-byte FIFO bank.
const DYNAMIC_FIFO_BYTES: u16 = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EndpointConfig {
    ep_type: EndpointType,
    max_packet_size: u16,
    fifo_address: FifoAddress,
}

#[derive(Debug)]
struct AllocationState {
    in_endpoints: [Option<EndpointConfig>; ENDPOINT_COUNT],
    out_endpoints: [Option<EndpointConfig>; ENDPOINT_COUNT],
    next_fifo: u16,
}

impl AllocationState {
    const fn new() -> Self {
        Self {
            in_endpoints: [None; ENDPOINT_COUNT],
            out_endpoints: [None; ENDPOINT_COUNT],
            next_fifo: DYNAMIC_FIFO_START,
        }
    }

    fn allocate(
        &mut self,
        direction: UsbDirection,
        requested: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> usb_device::Result<EndpointAddress> {
        validate_endpoint_parameters(ep_type, max_packet_size)?;

        let index = if let Some(address) = requested {
            if address.direction() != direction {
                return Err(UsbError::InvalidEndpoint);
            }
            let index = address.index();
            if index >= ENDPOINT_COUNT {
                return Err(UsbError::InvalidEndpoint);
            }
            index
        } else if ep_type == EndpointType::Control {
            0
        } else {
            self.find_automatic(direction, ep_type, max_packet_size)
                .ok_or(UsbError::EndpointOverflow)?
        };

        if (index == 0) != (ep_type == EndpointType::Control) {
            return Err(UsbError::InvalidEndpoint);
        }

        let (target, opposite) = match direction {
            UsbDirection::In => (&self.in_endpoints, &self.out_endpoints),
            UsbDirection::Out => (&self.out_endpoints, &self.in_endpoints),
        };
        if target[index].is_some() {
            return Err(UsbError::InvalidEndpoint);
        }
        if index == 0
            && opposite[index].is_some_and(|config| config.max_packet_size != max_packet_size)
        {
            return Err(UsbError::InvalidEndpoint);
        }

        let (fifo_address, committed_next_fifo) = if index == 0 {
            (FifoAddress::default(), self.next_fifo)
        } else {
            let next = self
                .next_fifo
                .checked_add(DYNAMIC_FIFO_BYTES)
                .ok_or(UsbError::EndpointMemoryOverflow)?;
            if next > DYNAMIC_FIFO_END {
                return Err(UsbError::EndpointMemoryOverflow);
            }
            (FifoAddress::from_byte_offset(self.next_fifo), next)
        };

        let config = EndpointConfig {
            ep_type,
            max_packet_size,
            fifo_address,
        };
        match direction {
            UsbDirection::In => self.in_endpoints[index] = Some(config),
            UsbDirection::Out => self.out_endpoints[index] = Some(config),
        }
        self.next_fifo = committed_next_fifo;

        Ok(EndpointAddress::from_parts(index, direction))
    }

    fn find_automatic(
        &self,
        direction: UsbDirection,
        ep_type: EndpointType,
        max_packet_size: u16,
    ) -> Option<usize> {
        let (target, opposite) = match direction {
            UsbDirection::In => (&self.in_endpoints, &self.out_endpoints),
            UsbDirection::Out => (&self.out_endpoints, &self.in_endpoints),
        };

        // Pair matching IN/OUT endpoints first. This maps CDC's notification
        // endpoint to EP1 and its bulk pair to EP2, matching the board-proven
        // V821 layout while still allowing exact endpoint requests.
        (1..ENDPOINT_COUNT)
            .find(|&index| {
                target[index].is_none()
                    && opposite[index].is_some_and(|config| {
                        config.ep_type == ep_type && config.max_packet_size == max_packet_size
                    })
            })
            .or_else(|| {
                (1..ENDPOINT_COUNT)
                    .find(|&index| target[index].is_none() && opposite[index].is_none())
            })
            .or_else(|| (1..ENDPOINT_COUNT).find(|&index| target[index].is_none()))
    }

    fn endpoint(&self, address: EndpointAddress) -> usb_device::Result<&EndpointConfig> {
        let endpoints = match address.direction() {
            UsbDirection::In => &self.in_endpoints,
            UsbDirection::Out => &self.out_endpoints,
        };
        endpoints
            .get(address.index())
            .and_then(Option::as_ref)
            .ok_or(UsbError::InvalidEndpoint)
    }
}

fn validate_endpoint_parameters(
    ep_type: EndpointType,
    max_packet_size: u16,
) -> usb_device::Result<()> {
    let supported = match ep_type {
        EndpointType::Control | EndpointType::Bulk => {
            matches!(max_packet_size, 8 | 16 | 32 | 64)
        }
        EndpointType::Interrupt => (1..=64).contains(&max_packet_size),
        EndpointType::Isochronous { .. } => false,
    };
    if supported {
        Ok(())
    } else {
        Err(UsbError::Unsupported)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ControlPhase {
    Setup,
    DataIn { requested: u16, transferred: u16 },
    DataOut { remaining: u16 },
    StatusIn,
    StatusOut,
}

struct Inner<'a> {
    usb: Usb<'a>,
    allocation: AllocationState,
    in_pending: [bool; ENDPOINT_COUNT],
    in_complete_pending: u16,
    in_stalled: [bool; ENDPOINT_COUNT],
    out_stalled: [bool; ENDPOINT_COUNT],
    control_phase: ControlPhase,
    setup_pending: bool,
    early_status_out_pending: bool,
    ignore_control_zlp: bool,
}

impl<'a> Inner<'a> {
    fn new(usb: Usb<'a>) -> Self {
        Self {
            usb,
            allocation: AllocationState::new(),
            in_pending: [false; ENDPOINT_COUNT],
            in_complete_pending: 0,
            in_stalled: [false; ENDPOINT_COUNT],
            out_stalled: [false; ENDPOINT_COUNT],
            control_phase: ControlPhase::Setup,
            setup_pending: false,
            early_status_out_pending: false,
            ignore_control_zlp: false,
        }
    }

    fn select_endpoint(&self, endpoint: usize) {
        // SAFETY: all callers validate the index against ENDPOINT_COUNT and
        // hold the bus critical section across INDEX and dependent accesses.
        unsafe {
            self.usb
                .registers()
                .index
                .write(EndpointIndex::new(endpoint as u8));
        }
    }

    fn configure_endpoints(&mut self) {
        let registers = self.usb.registers();
        self.in_pending.fill(false);
        self.in_complete_pending = 0;
        self.in_stalled.fill(false);
        self.out_stalled.fill(false);
        self.control_phase = ControlPhase::Setup;
        self.setup_pending = false;
        self.early_status_out_pending = false;
        self.ignore_control_zlp = false;

        self.select_endpoint(0);
        registers
            .tx_csr
            .write_endpoint_zero(EndpointZeroControlStatus::clear());

        let mut transmit_interrupts = TransmitInterruptEnable::default().enable_endpoint(0);
        let mut receive_interrupts = ReceiveInterruptEnable::default();

        for endpoint in 1..ENDPOINT_COUNT {
            self.select_endpoint(endpoint);
            if let Some(config) = self.allocation.in_endpoints[endpoint] {
                registers
                    .tx_csr
                    .write_transmit(TransmitControlStatus::clear());
                // SAFETY: allocation validates a full-speed packet size and
                // the selected endpoint is held for this complete transaction.
                unsafe {
                    registers
                        .tx_max_packet
                        .write(MaximumPacketSize::new(config.max_packet_size));
                }
                registers
                    .tx_csr
                    .write_transmit(TransmitControlStatus::flush_and_clear_data_toggle());
                // SAFETY: every bump allocation is aligned, single-bank, and
                // bounded by DYNAMIC_FIFO_END.
                unsafe {
                    registers.tx_fifo_size.write(FifoSize::single_512());
                    registers.tx_fifo_address.write(config.fifo_address);
                }
                transmit_interrupts = transmit_interrupts.enable_endpoint(endpoint as u8);
            }

            if let Some(config) = self.allocation.out_endpoints[endpoint] {
                registers.rx_csr.write(ReceiveControlStatus::clear());
                // SAFETY: same validated selected-endpoint invariant as IN.
                unsafe {
                    registers
                        .rx_max_packet
                        .write(MaximumPacketSize::new(config.max_packet_size));
                }
                registers
                    .rx_csr
                    .write(ReceiveControlStatus::flush_and_clear_data_toggle());
                // SAFETY: same bounded FIFO allocation invariant as IN.
                unsafe {
                    registers.rx_fifo_size.write(FifoSize::single_512());
                    registers.rx_fifo_address.write(config.fifo_address);
                }
                receive_interrupts = receive_interrupts.enable_endpoint(endpoint as u8);
            }
        }

        // SAFETY: masks contain only allocated endpoints zero through three.
        unsafe {
            registers.interrupt_tx_enable.write(transmit_interrupts);
            registers.interrupt_rx_enable.write(receive_interrupts);
        }
        self.select_endpoint(0);
    }

    fn reset(&mut self) {
        let registers = self.usb.registers();
        // SAFETY: the bus owns the controller address register.
        unsafe {
            registers.function_address.write(FunctionAddress::default());
        }
        acknowledge_pending(registers);
        self.configure_endpoints();
    }

    fn write(&mut self, address: EndpointAddress, buffer: &[u8]) -> usb_device::Result<usize> {
        let config = *self.allocation.endpoint(address)?;
        if !address.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }
        if buffer.len() > config.max_packet_size as usize {
            return Err(UsbError::BufferOverflow);
        }

        let endpoint = address.index();
        if self.in_pending[endpoint] || self.in_stalled[endpoint] {
            return Err(UsbError::WouldBlock);
        }
        if endpoint == 0 {
            self.write_control(buffer, config.max_packet_size)
        } else {
            self.select_endpoint(endpoint);
            let registers = self.usb.registers();
            if !registers.tx_csr.read_transmit().can_accept_packet() {
                self.select_endpoint(0);
                return Err(UsbError::WouldBlock);
            }
            for &byte in buffer {
                registers.fifo[endpoint].write_byte(byte);
            }
            registers
                .tx_csr
                .write_transmit(TransmitControlStatus::queue_packet());
            self.in_pending[endpoint] = true;
            self.select_endpoint(0);
            Ok(buffer.len())
        }
    }

    fn write_control(&mut self, buffer: &[u8], max_packet_size: u16) -> usb_device::Result<usize> {
        self.select_endpoint(0);
        let registers = self.usb.registers();

        if self.control_phase == ControlPhase::StatusOut
            && self.ignore_control_zlp
            && buffer.is_empty()
        {
            // usb-device 0.3 queues a ZLP after every full final EP0 packet.
            // When wLength itself ended the transfer, DATAEND was already set
            // and the host proceeds directly to status OUT; accept that
            // redundant software ZLP without touching the hardware.
            self.ignore_control_zlp = false;
            return Ok(0);
        }

        if registers
            .tx_csr
            .read_endpoint_zero()
            .transmit_packet_ready()
        {
            return Err(UsbError::WouldBlock);
        }

        match self.control_phase {
            ControlPhase::DataIn {
                requested,
                transferred,
            } => {
                for &byte in buffer {
                    registers.fifo[0].write_byte(byte);
                }
                let next = transferred.saturating_add(buffer.len() as u16);
                let reached_request = next >= requested;
                let data_end = buffer.len() < max_packet_size as usize || reached_request;
                registers.tx_csr.write_endpoint_zero(
                    EndpointZeroControlStatus::queue_transmit_packet(data_end),
                );
                self.in_pending[0] = true;
                if data_end {
                    self.control_phase = ControlPhase::StatusOut;
                    self.ignore_control_zlp = reached_request
                        && buffer.len() == max_packet_size as usize
                        && !buffer.is_empty();
                } else {
                    self.control_phase = ControlPhase::DataIn {
                        requested,
                        transferred: next,
                    };
                }
                Ok(buffer.len())
            }
            ControlPhase::StatusIn if buffer.is_empty() => {
                // MUSB completes a control-OUT transfer by servicing the held
                // final RX packet together with DATAEND. It then generates the
                // status-IN handshake without a FIFO payload.
                registers.tx_csr.write_endpoint_zero(
                    EndpointZeroControlStatus::service_received_packet_and_complete(),
                );
                self.in_pending[0] = true;
                Ok(0)
            }
            _ => Err(UsbError::InvalidState),
        }
    }

    fn read(&mut self, address: EndpointAddress, buffer: &mut [u8]) -> usb_device::Result<usize> {
        let config = *self.allocation.endpoint(address)?;
        if !address.is_out() {
            return Err(UsbError::InvalidEndpoint);
        }

        let endpoint = address.index();
        if endpoint != 0 && self.out_stalled[endpoint] {
            return Err(UsbError::WouldBlock);
        }
        if endpoint == 0 {
            self.read_control(buffer, config.max_packet_size)
        } else {
            self.select_endpoint(endpoint);
            let registers = self.usb.registers();
            if !registers.rx_csr.read().packet_ready() {
                self.select_endpoint(0);
                return Err(UsbError::WouldBlock);
            }
            let count = registers.rx_count.read().bytes();
            if count > buffer.len() || count > config.max_packet_size as usize {
                self.select_endpoint(0);
                return Err(UsbError::BufferOverflow);
            }
            for byte in &mut buffer[..count] {
                *byte = registers.fifo[endpoint].read_byte();
            }
            registers.rx_csr.write(ReceiveControlStatus::clear());
            self.select_endpoint(0);
            Ok(count)
        }
    }

    fn read_control(
        &mut self,
        buffer: &mut [u8],
        max_packet_size: u16,
    ) -> usb_device::Result<usize> {
        self.select_endpoint(0);
        let registers = self.usb.registers();
        if self.early_status_out_pending {
            // MUSB reports an early status-OUT through SETUPEND and consumes
            // the ZLP in hardware. Present the packet expected by usb-device
            // even though RXPKTRDY is no longer set.
            self.early_status_out_pending = false;
            self.control_phase = ControlPhase::Setup;
            self.in_pending[0] = false;
            self.in_complete_pending &= !1;
            self.ignore_control_zlp = false;
            return Ok(0);
        }
        let csr = registers.tx_csr.read_endpoint_zero();
        if !csr.received_packet_ready() {
            return Err(UsbError::WouldBlock);
        }

        if self.setup_pending {
            let mut count = registers.rx_count.read().bytes();
            for _ in 0..16 {
                if count == 8 {
                    break;
                }
                count = registers.rx_count.read().bytes();
            }
            if count != 8 {
                return Err(UsbError::InvalidState);
            }
            if buffer.len() < 8 {
                return Err(UsbError::BufferOverflow);
            }
            for byte in &mut buffer[..8] {
                *byte = registers.fifo[0].read_byte();
            }

            let direction_in = buffer[0] & 0x80 != 0;
            let requested = u16::from_le_bytes([buffer[6], buffer[7]]);
            self.control_phase = if direction_in {
                registers
                    .tx_csr
                    .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet());
                ControlPhase::DataIn {
                    requested,
                    transferred: 0,
                }
            } else if requested == 0 {
                // Hold RXPKTRDY. The following status-IN write retires this
                // SETUP packet and sets DATAEND in one 0x48 transaction.
                ControlPhase::StatusIn
            } else {
                registers
                    .tx_csr
                    .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet());
                ControlPhase::DataOut {
                    remaining: requested,
                }
            };
            self.setup_pending = false;
            self.ignore_control_zlp = false;
            return Ok(8);
        }

        let count = registers.rx_count.read().bytes();
        if count > buffer.len() || count > max_packet_size as usize {
            return Err(UsbError::BufferOverflow);
        }

        match self.control_phase {
            ControlPhase::DataOut { remaining } => {
                for byte in &mut buffer[..count] {
                    *byte = registers.fifo[0].read_byte();
                }
                let remaining = remaining.saturating_sub(count as u16);
                if remaining == 0 {
                    // Keep RXPKTRDY set until usb-device accepts/rejects the
                    // complete request and writes the status-IN response.
                    self.control_phase = ControlPhase::StatusIn;
                } else {
                    registers
                        .tx_csr
                        .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet());
                    self.control_phase = ControlPhase::DataOut { remaining };
                }
                Ok(count)
            }
            ControlPhase::DataIn { .. } | ControlPhase::StatusOut if count == 0 => {
                // A host may end a control-IN data stage before the complete
                // response has been sent by acknowledging the last IN packet
                // with an OUT ZLP. usb-device deliberately reports that ZLP
                // while its control pipe is still in a DataIn state, so the
                // controller phase must accept it here as a normal status
                // stage as well as after the final data packet.
                registers.tx_csr.write_endpoint_zero(
                    EndpointZeroControlStatus::service_received_packet_and_complete(),
                );
                self.in_pending[0] = false;
                self.in_complete_pending &= !1;
                self.control_phase = ControlPhase::Setup;
                self.ignore_control_zlp = false;
                Ok(0)
            }
            _ => Err(UsbError::InvalidState),
        }
    }

    fn set_stalled(&mut self, address: EndpointAddress, stalled: bool) {
        if self.allocation.endpoint(address).is_err() {
            return;
        }
        let endpoint = address.index();
        self.select_endpoint(endpoint);
        let registers = self.usb.registers();

        if endpoint == 0 {
            let csr = registers.tx_csr.read_endpoint_zero();
            if stalled {
                let command = if csr.received_packet_ready() {
                    EndpointZeroControlStatus::service_received_packet_and_stall()
                } else {
                    EndpointZeroControlStatus::stall()
                };
                registers.tx_csr.write_endpoint_zero(command);
            } else {
                registers
                    .tx_csr
                    .write_endpoint_zero(EndpointZeroControlStatus::clear());
            }
        } else if address.is_in() {
            registers.tx_csr.write_transmit(if stalled {
                TransmitControlStatus::stall()
            } else {
                TransmitControlStatus::flush_and_clear_data_toggle()
            });
        } else {
            registers.rx_csr.write(if stalled {
                ReceiveControlStatus::stall()
            } else {
                ReceiveControlStatus::flush_and_clear_data_toggle()
            });
        }

        if endpoint == 0 {
            // EP0 has one physical CSR and one stall handshake even though
            // usb-device represents its two directions as separate addresses.
            self.in_stalled[0] = stalled;
            self.out_stalled[0] = stalled;
            if stalled {
                self.in_pending[0] = false;
                self.in_complete_pending &= !1;
                self.control_phase = ControlPhase::Setup;
                self.setup_pending = false;
                self.early_status_out_pending = false;
                self.ignore_control_zlp = false;
            }
        } else if address.is_in() {
            self.in_stalled[endpoint] = stalled;
            // Stalling or resetting an IN endpoint discards any queued packet
            // and completion from the previous endpoint epoch.
            self.in_pending[endpoint] = false;
            self.in_complete_pending &= !(1 << endpoint);
        } else {
            self.out_stalled[endpoint] = stalled;
        }
        self.select_endpoint(0);
    }

    fn is_stalled(&self, address: EndpointAddress) -> bool {
        if self.allocation.endpoint(address).is_err() {
            return false;
        }
        if address.index() == 0 {
            self.in_stalled[0]
        } else if address.is_in() {
            self.in_stalled[address.index()]
        } else {
            self.out_stalled[address.index()]
        }
    }

    fn poll(&mut self) -> PollResult {
        let registers = self.usb.registers();
        let bus_status = registers.interrupt_usb.status();
        if !bus_status.is_empty() {
            registers.interrupt_usb.acknowledge(bus_status);
        }
        if bus_status.reset_pending() {
            self.in_pending.fill(false);
            self.in_complete_pending = 0;
            self.setup_pending = false;
            self.control_phase = ControlPhase::Setup;
            self.early_status_out_pending = false;
            self.ignore_control_zlp = false;
            return PollResult::Reset;
        }

        let transmit_status = registers.interrupt_tx.status();
        if !transmit_status.is_empty() {
            registers.interrupt_tx.acknowledge(transmit_status);
        }
        let receive_status = registers.interrupt_rx.status();
        if !receive_status.is_empty() {
            registers.interrupt_rx.acknowledge(receive_status);
        }

        for endpoint in 0..ENDPOINT_COUNT {
            if !transmit_status.endpoint_pending(endpoint as u8) || !self.in_pending[endpoint] {
                continue;
            }
            self.select_endpoint(endpoint);
            let packet_pending = if endpoint == 0 {
                registers
                    .tx_csr
                    .read_endpoint_zero()
                    .transmit_packet_ready()
            } else {
                registers.tx_csr.read_transmit().packet_ready()
            };
            if !packet_pending {
                self.in_pending[endpoint] = false;
                self.in_complete_pending |= 1 << endpoint;
                if endpoint == 0 && self.control_phase == ControlPhase::StatusIn {
                    self.control_phase = ControlPhase::Setup;
                }
            }
        }

        self.select_endpoint(0);
        let mut endpoint_zero = registers.tx_csr.read_endpoint_zero();
        if endpoint_zero.sent_stall() {
            self.in_stalled[0] = true;
            self.out_stalled[0] = true;
            self.control_phase = ControlPhase::Setup;
            self.setup_pending = false;
            self.early_status_out_pending = false;
            self.in_pending[0] = false;
            self.in_complete_pending &= !1;
            self.ignore_control_zlp = false;
        }
        if endpoint_zero.setup_end() {
            let ended_control_in = matches!(self.control_phase, ControlPhase::DataIn { .. });
            registers
                .tx_csr
                .write_endpoint_zero(EndpointZeroControlStatus::service_setup_end());
            self.control_phase = ControlPhase::Setup;
            self.setup_pending = false;
            self.early_status_out_pending = false;
            self.in_pending[0] = false;
            self.in_complete_pending &= !1;
            self.ignore_control_zlp = false;

            // SETUPEND also denotes an early status stage. Re-read after its
            // acknowledgement: RXPKTRDY now means a back-to-back SETUP,
            // while no packet means the controller consumed an early OUT ZLP.
            endpoint_zero = registers.tx_csr.read_endpoint_zero();
            if endpoint_zero.received_packet_ready() {
                self.setup_pending = true;
            } else if ended_control_in {
                self.early_status_out_pending = true;
            }
        }

        if endpoint_zero.received_packet_ready()
            && matches!(
                self.control_phase,
                ControlPhase::StatusIn | ControlPhase::StatusOut
            )
        {
            // After DATAEND, MUSB normally consumes the status handshake in
            // hardware. The next eight-byte RX packet is therefore a new
            // SETUP even though SETUPEND is not asserted. COUNT0 can settle a
            // few cycles after RXPKTRDY, matching the BootROM retry sequence.
            let mut count = registers.rx_count.read().bytes();
            for _ in 0..16 {
                if count == 8 {
                    break;
                }
                count = registers.rx_count.read().bytes();
            }
            if count == 8 {
                self.control_phase = ControlPhase::Setup;
                self.in_pending[0] = false;
                self.in_complete_pending &= !1;
                self.ignore_control_zlp = false;
            }
        }
        if endpoint_zero.received_packet_ready() && self.control_phase == ControlPhase::Setup {
            self.setup_pending = true;
            self.early_status_out_pending = false;
        }

        let mut ep_out = 0u16;
        let mut ep_setup = 0u16;
        if endpoint_zero.received_packet_ready() {
            if self.setup_pending {
                ep_setup |= 1;
            } else {
                ep_out |= 1;
            }
        }
        if self.early_status_out_pending && !self.setup_pending {
            ep_out |= 1;
        }
        for endpoint in 1..ENDPOINT_COUNT {
            if self.allocation.out_endpoints[endpoint].is_none() {
                continue;
            }
            self.select_endpoint(endpoint);
            if registers.rx_csr.read().packet_ready() {
                ep_out |= 1 << endpoint;
            }
        }
        self.select_endpoint(0);

        let ep_in_complete = self.in_complete_pending;
        self.in_complete_pending = 0;
        if ep_out != 0 || ep_setup != 0 || ep_in_complete != 0 {
            PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            }
        } else {
            PollResult::None
        }
    }
}

/// `usb-device` adapter for an owned [`Usb`] peripheral.
///
/// Every indexed endpoint transaction executes inside one critical section,
/// including the INDEX write and all dependent CSR/count/FIFO accesses. The
/// first implementation is deliberately polling-only; the USB interrupt must
/// not call into this bus.
pub struct UsbBus<'a> {
    inner: Mutex<RefCell<Inner<'a>>>,
}

impl<'a> UsbBus<'a> {
    /// Create a polling `usb-device` backend and consume the peripheral owner.
    pub fn new(usb: Usb<'a>) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(Inner::new(usb))),
        }
    }

    fn with_inner<R>(&self, f: impl FnOnce(&mut Inner<'a>) -> R) -> R {
        critical_section::with(|critical_section| {
            let mut inner = self.inner.borrow(critical_section).borrow_mut();
            f(&mut inner)
        })
    }
}

impl UsbBusTrait for UsbBus<'_> {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> usb_device::Result<EndpointAddress> {
        self.inner.get_mut().get_mut().allocation.allocate(
            ep_dir,
            ep_addr,
            ep_type,
            max_packet_size,
            interval,
        )
    }

    fn enable(&mut self) {
        let inner = self.inner.get_mut().get_mut();
        inner.configure_endpoints();
        let registers = inner.usb.registers();
        acknowledge_pending(registers);
        // Only reset is advertised in the first polling implementation.
        // SAFETY: the adapter exclusively owns the interrupt masks.
        unsafe {
            registers
                .interrupt_usb_enable
                .write(BusInterruptEnable::default().enable_reset());
            registers
                .power
                .write(registers.power.read().set_soft_connected(true));
        }
    }

    fn reset(&self) {
        self.with_inner(Inner::reset);
    }

    fn set_device_address(&self, address: u8) {
        self.with_inner(|inner| {
            // SAFETY: usb-device supplies a seven-bit address and this bus is
            // the sole owner of FADDR.
            unsafe {
                inner
                    .usb
                    .registers()
                    .function_address
                    .write(FunctionAddress::new(address));
            }
        });
    }

    fn write(&self, endpoint: EndpointAddress, buffer: &[u8]) -> usb_device::Result<usize> {
        self.with_inner(|inner| inner.write(endpoint, buffer))
    }

    fn read(&self, endpoint: EndpointAddress, buffer: &mut [u8]) -> usb_device::Result<usize> {
        self.with_inner(|inner| inner.read(endpoint, buffer))
    }

    fn set_stalled(&self, endpoint: EndpointAddress, stalled: bool) {
        self.with_inner(|inner| inner.set_stalled(endpoint, stalled));
    }

    fn is_stalled(&self, endpoint: EndpointAddress) -> bool {
        self.with_inner(|inner| inner.is_stalled(endpoint))
    }

    fn suspend(&self) {}

    fn resume(&self) {}

    fn poll(&self) -> PollResult {
        self.with_inner(Inner::poll)
    }

    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_zero_directions_share_the_fixed_fifo() {
        let mut allocation = AllocationState::new();
        let out = allocation
            .allocate(
                UsbDirection::Out,
                Some(EndpointAddress::from(0x00)),
                EndpointType::Control,
                64,
                0,
            )
            .unwrap();
        let input = allocation
            .allocate(
                UsbDirection::In,
                Some(EndpointAddress::from(0x80)),
                EndpointType::Control,
                64,
                0,
            )
            .unwrap();
        assert_eq!(out.index(), 0);
        assert_eq!(input.index(), 0);
        assert_eq!(allocation.next_fifo, DYNAMIC_FIFO_START);
    }

    #[test]
    fn automatic_cdc_allocation_matches_the_proven_endpoint_pairing() {
        let mut allocation = AllocationState::new();
        let notification = allocation
            .allocate(UsbDirection::In, None, EndpointType::Interrupt, 8, 255)
            .unwrap();
        let data_out = allocation
            .allocate(UsbDirection::Out, None, EndpointType::Bulk, 64, 0)
            .unwrap();
        let data_in = allocation
            .allocate(UsbDirection::In, None, EndpointType::Bulk, 64, 0)
            .unwrap();

        assert_eq!(u8::from(notification), 0x81);
        assert_eq!(u8::from(data_out), 0x02);
        assert_eq!(u8::from(data_in), 0x82);
        assert_eq!(
            allocation.in_endpoints[1]
                .unwrap()
                .fifo_address
                .byte_offset(),
            0x0200
        );
        assert_eq!(
            allocation.out_endpoints[2]
                .unwrap()
                .fifo_address
                .byte_offset(),
            0x0400
        );
        assert_eq!(
            allocation.in_endpoints[2]
                .unwrap()
                .fifo_address
                .byte_offset(),
            0x0600
        );
    }

    #[test]
    fn composite_bulk_pair_uses_endpoint_three_without_exceeding_four_kib() {
        let mut allocation = AllocationState::new();
        allocation
            .allocate(UsbDirection::In, None, EndpointType::Interrupt, 8, 255)
            .unwrap();
        allocation
            .allocate(UsbDirection::Out, None, EndpointType::Bulk, 64, 0)
            .unwrap();
        allocation
            .allocate(UsbDirection::In, None, EndpointType::Bulk, 64, 0)
            .unwrap();
        let second_out = allocation
            .allocate(UsbDirection::Out, None, EndpointType::Bulk, 64, 0)
            .unwrap();
        let second_in = allocation
            .allocate(UsbDirection::In, None, EndpointType::Bulk, 64, 0)
            .unwrap();

        assert_eq!(u8::from(second_out), 0x03);
        assert_eq!(u8::from(second_in), 0x83);
        assert!(allocation.next_fifo <= DYNAMIC_FIFO_END);
    }

    #[test]
    fn failed_allocation_is_transactional() {
        let mut allocation = AllocationState::new();
        let before = allocation.next_fifo;
        assert_eq!(
            allocation.allocate(
                UsbDirection::In,
                Some(EndpointAddress::from(0x84)),
                EndpointType::Bulk,
                64,
                0,
            ),
            Err(UsbError::InvalidEndpoint)
        );
        assert_eq!(allocation.next_fifo, before);
        assert!(allocation.in_endpoints.iter().all(Option::is_none));
    }

    #[test]
    fn unsupported_transfer_modes_are_rejected() {
        use usb_device::endpoint::{IsochronousSynchronizationType, IsochronousUsageType};

        let mut allocation = AllocationState::new();
        assert_eq!(
            allocation.allocate(
                UsbDirection::In,
                None,
                EndpointType::Isochronous {
                    synchronization: IsochronousSynchronizationType::NoSynchronization,
                    usage: IsochronousUsageType::Data,
                },
                64,
                1,
            ),
            Err(UsbError::Unsupported)
        );
        assert_eq!(
            allocation.allocate(UsbDirection::Out, None, EndpointType::Bulk, 512, 0),
            Err(UsbError::Unsupported)
        );
    }
}
