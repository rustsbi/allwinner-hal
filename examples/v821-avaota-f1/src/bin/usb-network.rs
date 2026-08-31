#![no_std]
#![no_main]

//! CDC-NCM Ethernet device backed by usb-device and the ownership-based
//! Allwinner USB HAL. The class and its IPv6 demo remain local to this example.

use allwinner_hal::usb::{Usb, UsbBus as AllwinnerUsbBus, phy::v821::UsbPhy};
use allwinner_rt::{Clocks, Peripherals, entry};
use riscv::delay::McycleDelay;
use usb_device::{
    Result, UsbError,
    class_prelude::{
        ControlIn, ControlOut, DescriptorWriter, EndpointIn, EndpointOut, InterfaceNumber,
        StringIndex, UsbBus, UsbBusAllocator, UsbClass,
    },
    control::{Recipient, RequestType},
    descriptor::lang_id::LangID,
    device::{StringDescriptors, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};

const IPV6_ADDRESS: [u8; 16] = [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const NETWORK_DEVICE_MAC_ADDRESS: [u8; 6] = [0x02, 0xa0, 0xf1, 0x82, 0x10, 0x01];
const NETWORK_HOST_MAC_ADDRESS: [u8; 6] = [0x02, 0xa0, 0xf1, 0x82, 0x10, 0x02];
const NETWORK_HOST_MAC_STRING: &str = "02A0F1821002";

const ALL_NODES_ADDRESS: [u8; 16] = [0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const ALL_NODES_MAC: [u8; 6] = [0x33, 0x33, 0, 0, 0, 1];
const ALL_ROUTERS_ADDRESS: [u8; 16] = [0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
const ALL_ROUTERS_MAC: [u8; 6] = [0x33, 0x33, 0, 0, 0, 2];
const LINK_LOCAL_ADDRESS: [u8; 16] = [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const MDNS_ADDRESS: [u8; 16] = [0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xfb];
const MDNS_MAC: [u8; 6] = [0x33, 0x33, 0, 0, 0, 0xfb];
const MDNS_NAME: &[u8; 17] = b"\x09avaota-f1\x05local\0";
const MDNS_PORT: u16 = 5353;
const MDNS_TTL: u32 = 120;
const MDNS_AAAA_RECORD_SIZE: usize = MDNS_NAME.len() + 10 + 16;

const USB_PACKET_SIZE: usize = 64;
const MAX_FRAME_SIZE: usize = 1514;
const MAX_NTB_SIZE: usize = 2048;
const NTH16_SIZE: usize = 12;
const NDP16_SIZE: usize = 16;
const ETHERNET_HEADER_SIZE: usize = 14;
// Two padding bytes make the IPv6 header 4-byte aligned as advertised by
// wNdpInPayloadRemainder/wNdpOutPayloadRemainder.
const DATAGRAM_OFFSET: usize = NTH16_SIZE + NDP16_SIZE + 2;
const IPV6_HEADER_SIZE: usize = 40;
const IPV6_PAYLOAD_OFFSET: usize = ETHERNET_HEADER_SIZE + IPV6_HEADER_SIZE;
const ICMPV6_OFFSET: usize = IPV6_PAYLOAD_OFFSET;
const UDP_OFFSET: usize = IPV6_PAYLOAD_OFFSET;
const DNS_OFFSET: usize = UDP_OFFSET + 8;
const NETWORK_EXIT_DELAY_TICKS: u32 = 40 * 500_000;

const CDC_CLASS: u8 = 0x02;
const CDC_NCM_SUBCLASS: u8 = 0x0d;
const CDC_DATA_CLASS: u8 = 0x0a;
const CS_INTERFACE: u8 = 0x24;
const NCM_GET_NTB_PARAMETERS: u8 = 0x80;
const NCM_GET_NTB_INPUT_SIZE: u8 = 0x85;
const NCM_SET_NTB_INPUT_SIZE: u8 = 0x86;
const NCM_SET_ETHERNET_PACKET_FILTER: u8 = 0x43;

const PACKET_TYPE_PROMISCUOUS: u16 = 1 << 0;
const PACKET_TYPE_ALL_MULTICAST: u16 = 1 << 1;
const PACKET_TYPE_DIRECTED: u16 = 1 << 2;
const PACKET_TYPE_BROADCAST: u16 = 1 << 3;
const PACKET_TYPE_MULTICAST: u16 = 1 << 4;
const SUPPORTED_PACKET_FILTER: u16 = PACKET_TYPE_PROMISCUOUS
    | PACKET_TYPE_ALL_MULTICAST
    | PACKET_TYPE_DIRECTED
    | PACKET_TYPE_BROADCAST
    | PACKET_TYPE_MULTICAST;

const NCM_NTB_PARAMETERS: [u8; 28] = [
    28, 0, 1, 0, 0, 8, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 8, 0, 0, 4, 0, 0, 0, 4, 0, 1, 0,
];
const SPEED_CHANGE_NOTIFICATION: [u8; 16] = [
    0xa1, 0x2a, 0, 0, 0, 0, 8, 0, 0, 0x1b, 0xb7, 0, 0, 0x1b, 0xb7, 0,
];
const NETWORK_CONNECTION_NOTIFICATION: [u8; 8] = [0xa1, 0, 1, 0, 0, 0, 0, 0];

#[entry]
fn main(peripherals: Peripherals, clocks: Clocks) {
    let mut usb0 = peripherals.usb0;
    let mut usb_phy0 = peripherals.usb_phy0;
    let mut ccu = peripherals.ccu;
    let aon_ccu = peripherals.aon_ccu;
    let mut delay = McycleDelay::new(clocks.mcycle_ticks_second(&aon_ccu).unwrap());
    let oscillator = clocks.enable_usb(&mut usb0, &mut usb_phy0, &mut ccu, &aon_ccu, &mut delay);
    let usb = Usb::new(usb0, &mut delay);
    let mut _usb_phy = UsbPhy::new(usb_phy0, oscillator, &mut delay);
    if !usb.is_vbus_valid() {
        _usb_phy.force_vbus_valid();
    }

    let usb_bus = UsbBusAllocator::new(AllwinnerUsbBus::new(usb));
    let mut ncm = CdcNcmClass::new(&usb_bus);
    let strings = [StringDescriptors::default()
        .manufacturer("RustSBI")
        .product("Avaota F1 USB Network")
        .serial_number("0821F1000002")];
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1f3a, 0x8212))
        .strings(&strings)
        .unwrap()
        .composite_with_iads()
        .device_release(0x0100)
        .max_packet_size_0(64)
        .unwrap()
        .build();
    let mut network = UsbNetwork::new(IPV6_ADDRESS);

    loop {
        usb_device.poll(&mut [&mut ncm]);
        if network.poll(&mut ncm, usb_device.state() == UsbDeviceState::Configured) {
            return;
        }
    }
}

/// Packet-level CDC-NCM class. Every endpoint operation is non-blocking.
struct CdcNcmClass<'a, B: UsbBus> {
    control_interface: InterfaceNumber,
    data_interface: InterfaceNumber,
    mac_string: StringIndex,
    notification_in: EndpointIn<'a, B>,
    data_out: EndpointOut<'a, B>,
    data_in: EndpointIn<'a, B>,
    alternate_setting: u8,
    notification: u8,
    link_pending: bool,
    function_reset: bool,
    bus_reset: bool,
    packet_filter: u16,
}

impl<'a, B: UsbBus> CdcNcmClass<'a, B> {
    fn new(allocator: &'a UsbBusAllocator<B>) -> Self {
        Self {
            control_interface: allocator.interface(),
            notification_in: allocator.interrupt(16, 16),
            data_interface: allocator.interface(),
            data_out: allocator.bulk(USB_PACKET_SIZE as u16),
            data_in: allocator.bulk(USB_PACKET_SIZE as u16),
            mac_string: allocator.string(),
            alternate_setting: 0,
            notification: 0,
            link_pending: false,
            function_reset: false,
            bus_reset: false,
            packet_filter: 0,
        }
    }

    fn data_active(&self) -> bool {
        self.alternate_setting == 1
    }

    fn take_link_pending(&mut self) -> bool {
        if self.link_pending && self.allows_destination(&ALL_NODES_MAC) {
            self.link_pending = false;
            true
        } else {
            false
        }
    }

    fn take_function_reset(&mut self) -> bool {
        core::mem::take(&mut self.function_reset)
    }

    fn take_bus_reset(&mut self) -> bool {
        core::mem::take(&mut self.bus_reset)
    }

    fn allows_destination(&self, destination: &[u8; 6]) -> bool {
        if self.packet_filter & PACKET_TYPE_PROMISCUOUS != 0 {
            return true;
        }
        if *destination == [u8::MAX; 6] {
            return self.packet_filter & PACKET_TYPE_BROADCAST != 0;
        }
        if destination[0] & 1 != 0 {
            // This fixed demo has no programmable multicast-address table, so
            // MULTICAST admits its IPv6 groups as well as ALL_MULTICAST.
            return self.packet_filter & (PACKET_TYPE_MULTICAST | PACKET_TYPE_ALL_MULTICAST) != 0;
        }
        self.packet_filter & PACKET_TYPE_DIRECTED != 0 && *destination == NETWORK_HOST_MAC_ADDRESS
    }

    fn allows_frame(&self, frame: &[u8]) -> bool {
        frame
            .get(..6)
            .and_then(|destination| destination.try_into().ok())
            .is_some_and(|destination| self.allows_destination(destination))
    }

    fn reset_data_endpoints(&self) {
        // SET_INTERFACE starts a new endpoint epoch. The stall transition
        // cancels a queued IN packet; unstall flushes both FIFOs and resets
        // their data toggles before traffic resumes.
        self.data_in.stall();
        self.data_out.stall();
        self.data_in.unstall();
        self.data_out.unstall();
    }

    fn deconfigure(&mut self) {
        if self.alternate_setting != 0 {
            self.reset_data_endpoints();
            self.alternate_setting = 0;
            self.notification = 0;
            self.link_pending = false;
            self.function_reset = true;
        }
        self.packet_filter = 0;
    }

    fn read_packet(&self, packet: &mut [u8; USB_PACKET_SIZE]) -> Result<usize> {
        if self.data_active() {
            self.data_out.read(packet)
        } else {
            Err(UsbError::WouldBlock)
        }
    }

    fn write_packet(&self, packet: &[u8]) -> Result<usize> {
        if self.data_active() {
            self.data_in.write(packet)
        } else {
            Err(UsbError::WouldBlock)
        }
    }

    fn service_notification(&mut self) {
        if !self.data_active() {
            return;
        }
        let bytes = match self.notification {
            1 => &SPEED_CHANGE_NOTIFICATION[..],
            2 => &NETWORK_CONNECTION_NOTIFICATION[..],
            _ => return,
        };
        match self.notification_in.write(bytes) {
            Ok(_) => self.notification += 1,
            Err(UsbError::WouldBlock) => {}
            Err(_) => self.notification = 0,
        }
        if self.notification > 2 {
            self.notification = 0;
        }
    }

    fn targets_control(&self, ty: RequestType, recipient: Recipient, index: u16) -> bool {
        ty == RequestType::Class
            && recipient == Recipient::Interface
            && index == u8::from(self.control_interface) as u16
    }
}

impl<B: UsbBus> UsbClass<B> for CdcNcmClass<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.iad(
            self.control_interface,
            2,
            CDC_CLASS,
            CDC_NCM_SUBCLASS,
            0,
            None,
        )?;
        writer.interface(self.control_interface, CDC_CLASS, CDC_NCM_SUBCLASS, 0)?;
        writer.write(CS_INTERFACE, &[0x00, 0x10, 0x01])?;
        writer.write(
            CS_INTERFACE,
            &[
                0x06,
                self.control_interface.into(),
                self.data_interface.into(),
            ],
        )?;
        // iMACAddress identifies the host-side adapter, not the device source.
        writer.write(
            CS_INTERFACE,
            &[
                0x0f,
                self.mac_string.into(),
                0,
                0,
                0,
                0,
                0xea,
                0x05,
                0,
                0,
                0,
            ],
        )?;
        writer.write(CS_INTERFACE, &[0x1a, 0x00, 0x01, 0x01])?;
        writer.endpoint(&self.notification_in)?;

        writer.interface_alt(self.data_interface, 0, CDC_DATA_CLASS, 0, 0x01, None)?;
        writer.interface_alt(self.data_interface, 1, CDC_DATA_CLASS, 0, 0x01, None)?;
        writer.endpoint(&self.data_in)?;
        writer.endpoint(&self.data_out)?;
        Ok(())
    }

    fn get_string(&self, index: StringIndex, _lang_id: LangID) -> Option<&str> {
        (index == self.mac_string).then_some(NETWORK_HOST_MAC_STRING)
    }

    fn reset(&mut self) {
        self.alternate_setting = 0;
        self.notification = 0;
        self.link_pending = false;
        self.function_reset = true;
        self.bus_reset = true;
        self.packet_filter = 0;
    }

    fn poll(&mut self) {
        self.service_notification();
    }

    fn endpoint_in_complete(&mut self, address: usb_device::endpoint::EndpointAddress) {
        if address == self.notification_in.address() {
            self.service_notification();
        }
    }

    fn get_alt_setting(&mut self, interface: InterfaceNumber) -> Option<u8> {
        if interface == self.data_interface {
            Some(self.alternate_setting)
        } else if interface == self.control_interface {
            Some(0)
        } else {
            None
        }
    }

    fn set_alt_setting(&mut self, interface: InterfaceNumber, alternate: u8) -> bool {
        if interface == self.control_interface {
            return alternate == 0;
        }
        if interface != self.data_interface || alternate > 1 {
            return false;
        }

        self.reset_data_endpoints();
        self.function_reset = true;
        if alternate == 0 {
            self.notification = 0;
            self.link_pending = false;
        } else {
            self.notification = 1;
            self.link_pending = true;
        }
        self.alternate_setting = alternate;
        true
    }

    fn control_in(&mut self, transfer: ControlIn<B>) {
        let request = transfer.request();
        if !self.targets_control(request.request_type, request.recipient, request.index) {
            return;
        }
        match request.request {
            NCM_GET_NTB_PARAMETERS if request.value == 0 && request.length != 0 => {
                transfer.accept_with(&NCM_NTB_PARAMETERS).ok();
            }
            NCM_GET_NTB_INPUT_SIZE if request.value == 0 && request.length >= 4 => {
                transfer.accept_with(&2048_u32.to_le_bytes()).ok();
            }
            _ => {
                transfer.reject().ok();
            }
        }
    }

    fn control_out(&mut self, transfer: ControlOut<B>) {
        let request = *transfer.request();
        if !self.targets_control(request.request_type, request.recipient, request.index) {
            return;
        }
        match request.request {
            NCM_SET_NTB_INPUT_SIZE
                if request.value == 0
                    && request.length == 4
                    && transfer.data() == 2048_u32.to_le_bytes() =>
            {
                transfer.accept().ok();
            }
            NCM_SET_ETHERNET_PACKET_FILTER
                if request.length == 0 && request.value & !SUPPORTED_PACKET_FILTER == 0 =>
            {
                self.packet_filter = request.value;
                transfer.accept().ok();
            }
            _ => {
                transfer.reject().ok();
            }
        }
    }
}

/// Polling NCM/IPv6 demo state. USB packet I/O never spins waiting for hardware.
struct UsbNetwork {
    receive_ntb: [u8; MAX_NTB_SIZE],
    receive_len: usize,
    dropping_ntb: bool,
    transmit_ntb: [u8; DATAGRAM_OFFSET + MAX_FRAME_SIZE],
    transmit_len: usize,
    transmit_offset: usize,
    transmit_zlp: bool,
    frame: [u8; MAX_FRAME_SIZE],
    ipv6_address: [u8; 16],
    sequence: u16,
    was_active: bool,
    exit_started: Option<u32>,
}

impl UsbNetwork {
    fn new(ipv6_address: [u8; 16]) -> Self {
        Self {
            receive_ntb: [0; MAX_NTB_SIZE],
            receive_len: 0,
            dropping_ntb: false,
            transmit_ntb: [0; DATAGRAM_OFFSET + MAX_FRAME_SIZE],
            transmit_len: 0,
            transmit_offset: 0,
            transmit_zlp: false,
            frame: [0; MAX_FRAME_SIZE],
            ipv6_address,
            sequence: 0,
            was_active: false,
            exit_started: None,
        }
    }

    fn poll<B: UsbBus>(&mut self, ncm: &mut CdcNcmClass<'_, B>, configured: bool) -> bool {
        if !configured {
            ncm.deconfigure();
        }
        if ncm.take_bus_reset() {
            self.was_active = false;
            self.exit_started = None;
        }
        if ncm.take_function_reset() {
            self.reset_transfer_state();
            self.sequence = 0;
        }

        let active = configured && ncm.data_active();
        if active {
            self.exit_started = None;
        } else {
            if self.was_active && self.exit_started.is_none() {
                self.exit_started = Some(counter_low());
            }
            self.was_active = false;
            self.reset_transfer_state();
            return self.exit_started.is_some_and(|start| {
                counter_low().wrapping_sub(start) >= NETWORK_EXIT_DELAY_TICKS
            });
        }
        self.was_active = true;

        if ncm.take_link_pending() && self.transmit_len == 0 {
            let frame_len = write_router_advertisement(&mut self.frame, &self.ipv6_address);
            self.queue_frame(frame_len);
        }

        self.service_transmit(ncm);
        if self.transmit_len != 0 {
            return false;
        }

        let mut packet = [0; USB_PACKET_SIZE];
        let count = match ncm.read_packet(&mut packet) {
            Ok(count) => count,
            Err(UsbError::WouldBlock) => return false,
            Err(_) => {
                self.reset_receive();
                return false;
            }
        };

        if self.dropping_ntb {
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return false;
        }
        if count == 0 || self.receive_len + count > self.receive_ntb.len() {
            self.reset_receive();
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return false;
        }
        self.receive_ntb[self.receive_len..self.receive_len + count]
            .copy_from_slice(&packet[..count]);
        self.receive_len += count;

        if self.receive_len < NTH16_SIZE {
            if count < USB_PACKET_SIZE {
                self.reset_receive();
            }
            return false;
        }

        let block_len = get_u16(&self.receive_ntb, 8) as usize;
        if !(DATAGRAM_OFFSET..=MAX_NTB_SIZE).contains(&block_len) || self.receive_len > block_len {
            self.reset_receive();
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return false;
        }
        if self.receive_len < block_len {
            if count < USB_PACKET_SIZE {
                self.reset_receive();
            }
            return false;
        }

        let frame_len = decode_ntb(&self.receive_ntb[..self.receive_len], &mut self.frame);
        self.reset_receive();
        let Some(frame_len) = frame_len else {
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return false;
        };
        let Some(reply_len) = reply_ipv6(&mut self.frame, frame_len, &self.ipv6_address) else {
            return false;
        };
        if ncm.allows_frame(&self.frame[..reply_len]) {
            self.queue_frame(reply_len);
        }
        self.service_transmit(ncm);
        false
    }

    fn reset_receive(&mut self) {
        self.receive_len = 0;
    }

    fn reset_transfer_state(&mut self) {
        self.receive_len = 0;
        self.dropping_ntb = false;
        self.transmit_len = 0;
        self.transmit_offset = 0;
        self.transmit_zlp = false;
    }

    fn queue_frame(&mut self, frame_len: usize) {
        self.transmit_len = encode_ntb(
            &self.frame[..frame_len],
            &mut self.transmit_ntb,
            self.sequence,
        );
        self.sequence = self.sequence.wrapping_add(1);
        self.transmit_offset = 0;
        self.transmit_zlp = self.transmit_len.is_multiple_of(USB_PACKET_SIZE);
    }

    fn service_transmit<B: UsbBus>(&mut self, ncm: &CdcNcmClass<'_, B>) {
        if self.transmit_len == 0 {
            return;
        }
        if self.transmit_offset < self.transmit_len {
            let end = (self.transmit_offset + USB_PACKET_SIZE).min(self.transmit_len);
            match ncm.write_packet(&self.transmit_ntb[self.transmit_offset..end]) {
                Ok(count) => self.transmit_offset += count,
                Err(UsbError::WouldBlock) => return,
                Err(_) => {
                    self.transmit_len = 0;
                    return;
                }
            }
            if self.transmit_offset < self.transmit_len {
                return;
            }
        }

        if self.transmit_zlp {
            match ncm.write_packet(&[]) {
                Ok(_) => self.transmit_zlp = false,
                Err(UsbError::WouldBlock) => return,
                Err(_) => self.transmit_zlp = false,
            }
        }
        if !self.transmit_zlp {
            self.transmit_len = 0;
            self.transmit_offset = 0;
        }
    }
}

#[inline(always)]
fn counter_low() -> u32 {
    // SAFETY: documented aligned, read-only low word of the V821 counter.
    unsafe { (0x3000_bff8 as *const u32).read_volatile() }
}

fn decode_ntb(input: &[u8], frame: &mut [u8; MAX_FRAME_SIZE]) -> Option<usize> {
    if input.len() < DATAGRAM_OFFSET
        || &input[..4] != b"NCMH"
        || get_u16(input, 4) as usize != NTH16_SIZE
        || get_u16(input, 8) as usize != input.len()
    {
        return None;
    }

    let ndp = get_u16(input, 10) as usize;
    if ndp < NTH16_SIZE || !ndp.is_multiple_of(4) || ndp + NDP16_SIZE > input.len() {
        return None;
    }
    let ndp_len = get_u16(input, ndp + 4) as usize;
    if &input[ndp..ndp + 4] != b"NCM0"
        || ndp_len < NDP16_SIZE
        || !ndp_len.is_multiple_of(4)
        || ndp + ndp_len > input.len()
        || get_u16(input, ndp + 6) != 0
    {
        return None;
    }

    let offset = get_u16(input, ndp + 8) as usize;
    let length = get_u16(input, ndp + 10) as usize;
    let datagram_end = offset.checked_add(length)?;
    let ndp_end = ndp + ndp_len;
    if offset < NTH16_SIZE
        || !(offset + ETHERNET_HEADER_SIZE).is_multiple_of(4)
        || !(ETHERNET_HEADER_SIZE..=MAX_FRAME_SIZE).contains(&length)
        || datagram_end > input.len()
        || (offset < ndp_end && ndp < datagram_end)
        || get_u16(input, ndp + 12) != 0
        || get_u16(input, ndp + 14) != 0
    {
        return None;
    }
    frame[..length].copy_from_slice(&input[offset..datagram_end]);
    Some(length)
}

fn encode_ntb(frame: &[u8], output: &mut [u8], sequence: u16) -> usize {
    let block_len = DATAGRAM_OFFSET + frame.len();
    output[..DATAGRAM_OFFSET].fill(0);
    output[..4].copy_from_slice(b"NCMH");
    put_u16(output, 4, NTH16_SIZE as u16);
    put_u16(output, 6, sequence);
    put_u16(output, 8, block_len as u16);
    put_u16(output, 10, NTH16_SIZE as u16);
    output[NTH16_SIZE..NTH16_SIZE + 4].copy_from_slice(b"NCM0");
    put_u16(output, NTH16_SIZE + 4, NDP16_SIZE as u16);
    put_u16(output, NTH16_SIZE + 8, DATAGRAM_OFFSET as u16);
    put_u16(output, NTH16_SIZE + 10, frame.len() as u16);
    output[DATAGRAM_OFFSET..block_len].copy_from_slice(frame);
    block_len
}

fn reply_ipv6(
    frame: &mut [u8; MAX_FRAME_SIZE],
    received: usize,
    ipv6_address: &[u8; 16],
) -> Option<usize> {
    if received < IPV6_PAYLOAD_OFFSET + 8
        || frame[12..14] != [0x86, 0xdd]
        || frame[14] >> 4 != 6
        || frame[6] & 1 != 0
    {
        return None;
    }
    let destination_mac: [u8; 6] = frame[..6].try_into().ok()?;
    if destination_mac != NETWORK_DEVICE_MAC_ADDRESS && destination_mac[0] & 1 == 0 {
        return None;
    }

    let payload_len = u16::from_be_bytes(frame[18..20].try_into().ok()?) as usize;
    let end = IPV6_PAYLOAD_OFFSET.checked_add(payload_len)?;
    if payload_len < 8 || end > received || IPV6_HEADER_SIZE + payload_len > 1500 {
        return None;
    }

    let source_ip: [u8; 16] = frame[22..38].try_into().ok()?;
    let destination_ip: [u8; 16] = frame[38..54].try_into().ok()?;
    if frame[20] == 17 {
        return reply_mdns(frame, end, source_ip, destination_ip, ipv6_address);
    }
    if frame[20] != 58 {
        return None;
    }
    if icmpv6_checksum(&source_ip, &destination_ip, &frame[ICMPV6_OFFSET..end]) != 0 {
        return None;
    }

    match frame[ICMPV6_OFFSET] {
        128 if frame[ICMPV6_OFFSET + 1] == 0
            && (destination_ip == *ipv6_address || destination_ip == LINK_LOCAL_ADDRESS)
            && destination_mac == NETWORK_DEVICE_MAC_ADDRESS
            && source_ip != [0; 16]
            && source_ip[0] != 0xff =>
        {
            let source_mac: [u8; 6] = frame[6..12].try_into().ok()?;
            let reply_source = destination_ip;
            frame[..6].copy_from_slice(&source_mac);
            frame[6..12].copy_from_slice(&NETWORK_DEVICE_MAC_ADDRESS);
            write_ipv6_header(frame, payload_len, 64, &reply_source, &source_ip);
            frame[ICMPV6_OFFSET] = 129;
            frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].fill(0);
            let checksum = icmpv6_checksum(&reply_source, &source_ip, &frame[ICMPV6_OFFSET..end]);
            frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].copy_from_slice(&checksum.to_be_bytes());
            Some(end)
        }
        133 if frame[ICMPV6_OFFSET + 1] == 0 && frame[21] == 255 => {
            valid_router_solicitation(frame, end, source_ip, destination_ip)
                .then(|| write_router_advertisement(frame, ipv6_address))
        }
        135 if frame[ICMPV6_OFFSET + 1] == 0 && frame[21] == 255 && payload_len >= 24 => {
            reply_neighbor_solicitation(frame, end, source_ip, destination_ip, ipv6_address)
        }
        _ => None,
    }
}

fn reply_mdns(
    frame: &mut [u8; MAX_FRAME_SIZE],
    end: usize,
    source_ip: [u8; 16],
    destination_ip: [u8; 16],
    ipv6_address: &[u8; 16],
) -> Option<usize> {
    if destination_ip != MDNS_ADDRESS
        || frame[..6] != MDNS_MAC
        || source_ip == [0; 16]
        || source_ip[0] == 0xff
        || !is_on_link(&source_ip, ipv6_address)
        || end < DNS_OFFSET + 12
    {
        return None;
    }

    let source_port = read_be_u16(frame, UDP_OFFSET)?;
    let destination_port = read_be_u16(frame, UDP_OFFSET + 2)?;
    let udp_len = read_be_u16(frame, UDP_OFFSET + 4)? as usize;
    if source_port == 0
        || destination_port != MDNS_PORT
        || udp_len != end - UDP_OFFSET
        || udp_len < 20
        || frame[UDP_OFFSET + 6..UDP_OFFSET + 8] == [0, 0]
        || ipv6_checksum(17, &source_ip, &destination_ip, &frame[UDP_OFFSET..end]) != 0
    {
        return None;
    }

    let dns_len = udp_len - 8;
    let (query_id, query_flags, question_count, questions_end, found, known_answer) = {
        let dns = &frame[DNS_OFFSET..DNS_OFFSET + dns_len];
        let query_id = read_be_u16(dns, 0)?;
        let query_flags = read_be_u16(dns, 2)?;
        let question_count = read_be_u16(dns, 4)?;
        let (questions_end, found, known_answer) = parse_mdns_query(dns, ipv6_address)?;
        (
            query_id,
            query_flags,
            question_count,
            questions_end,
            found,
            known_answer,
        )
    };
    if !found || (source_port == MDNS_PORT && known_answer) {
        return None;
    }

    let source_mac: [u8; 6] = frame[6..12].try_into().ok()?;
    let legacy = source_port != MDNS_PORT;
    let answer_offset = if legacy {
        let answer_offset = DNS_OFFSET.checked_add(questions_end)?;
        if answer_offset + MDNS_AAAA_RECORD_SIZE > frame.len() {
            return None;
        }
        write_be_u16(frame, DNS_OFFSET, query_id);
        write_be_u16(frame, DNS_OFFSET + 2, 0x8400 | (query_flags & 0x0100));
        write_be_u16(frame, DNS_OFFSET + 4, question_count);
        write_be_u16(frame, DNS_OFFSET + 6, 1);
        frame[DNS_OFFSET + 8..DNS_OFFSET + 12].fill(0);
        answer_offset
    } else {
        let answer_offset = DNS_OFFSET + 12;
        frame[DNS_OFFSET..answer_offset + MDNS_AAAA_RECORD_SIZE].fill(0);
        write_be_u16(frame, DNS_OFFSET + 2, 0x8400);
        write_be_u16(frame, DNS_OFFSET + 6, 1);
        answer_offset
    };

    let reply_end = write_mdns_aaaa(
        frame,
        answer_offset,
        if legacy { 1 } else { 0x8001 },
        if legacy { 10 } else { MDNS_TTL },
        ipv6_address,
    )?;

    let (destination_mac, reply_source, reply_destination, reply_port) = if legacy {
        let reply_source = if source_ip[0] == 0xfe && source_ip[1] & 0xc0 == 0x80 {
            LINK_LOCAL_ADDRESS
        } else {
            *ipv6_address
        };
        (source_mac, reply_source, source_ip, source_port)
    } else {
        // Multicast is valid for QU too, and avoids competing port-5353 listeners.
        (MDNS_MAC, LINK_LOCAL_ADDRESS, MDNS_ADDRESS, MDNS_PORT)
    };
    frame[..6].copy_from_slice(&destination_mac);
    frame[6..12].copy_from_slice(&NETWORK_DEVICE_MAC_ADDRESS);
    frame[12..14].copy_from_slice(&[0x86, 0xdd]);
    finish_udp_frame(
        frame,
        reply_end - DNS_OFFSET,
        &reply_source,
        &reply_destination,
        MDNS_PORT,
        reply_port,
    )
}

fn parse_mdns_query(dns: &[u8], ipv6_address: &[u8; 16]) -> Option<(usize, bool, bool)> {
    if dns.len() < 12 || read_be_u16(dns, 2)? & 0xfa0f != 0 {
        return None;
    }

    let question_count = read_be_u16(dns, 4)? as usize;
    let answer_count = read_be_u16(dns, 6)? as usize;
    let mut offset = 12;
    let mut found = false;
    for _ in 0..question_count {
        let (name_end, name_matches) = dns_name_matches(dns, offset)?;
        if name_end + 4 > dns.len() {
            return None;
        }
        let question_type = read_be_u16(dns, name_end)?;
        let question_class = read_be_u16(dns, name_end + 2)? & 0x7fff;
        found |=
            name_matches && matches!(question_type, 28 | 255) && matches!(question_class, 1 | 255);
        offset = name_end + 4;
    }
    let questions_end = offset;

    let mut known_answer = false;
    for _ in 0..answer_count {
        let (name_end, name_matches) = dns_name_matches(dns, offset)?;
        if name_end + 10 > dns.len() {
            return None;
        }
        let record_type = read_be_u16(dns, name_end)?;
        let record_class = read_be_u16(dns, name_end + 2)? & 0x7fff;
        let ttl = read_be_u32(dns, name_end + 4)?;
        let data_len = read_be_u16(dns, name_end + 8)? as usize;
        let data = name_end.checked_add(10)?;
        let record_end = data.checked_add(data_len)?;
        if record_end > dns.len() {
            return None;
        }
        known_answer |= name_matches
            && record_type == 28
            && record_class == 1
            && ttl >= MDNS_TTL / 2
            && data_len == ipv6_address.len()
            && dns[data..record_end] == *ipv6_address;
        offset = record_end;
    }
    Some((questions_end, found, known_answer))
}

fn dns_name_matches(packet: &[u8], start: usize) -> Option<(usize, bool)> {
    let mut cursor = start;
    let mut encoded_end = None;
    let mut expected = 0;
    let mut matches = true;
    let mut expanded_len = 0;
    let mut jumps = 0;

    loop {
        let length = *packet.get(cursor)?;
        if length & 0xc0 == 0xc0 {
            let pointer = (usize::from(length & 0x3f) << 8) | usize::from(*packet.get(cursor + 1)?);
            if pointer >= cursor || jumps == 16 {
                return None;
            }
            encoded_end.get_or_insert(cursor + 2);
            cursor = pointer;
            jumps += 1;
            continue;
        }
        if length & 0xc0 != 0 {
            return None;
        }

        cursor += 1;
        expanded_len += 1;
        if expanded_len > 255 {
            return None;
        }
        if length == 0 {
            matches &= MDNS_NAME.get(expected) == Some(&0);
            return Some((encoded_end.unwrap_or(cursor), matches));
        }

        let length = usize::from(length);
        let label_end = cursor.checked_add(length)?;
        if label_end > packet.len() || expanded_len + length > 255 {
            return None;
        }
        expanded_len += length;
        if matches {
            let expected_len = usize::from(*MDNS_NAME.get(expected)?);
            if expected_len != length {
                matches = false;
            } else {
                for index in 0..length {
                    matches &= packet[cursor + index].to_ascii_lowercase()
                        == MDNS_NAME[expected + 1 + index];
                }
                expected += length + 1;
            }
        }
        cursor = label_end;
    }
}

fn write_mdns_aaaa(
    frame: &mut [u8; MAX_FRAME_SIZE],
    mut offset: usize,
    class: u16,
    ttl: u32,
    ipv6_address: &[u8; 16],
) -> Option<usize> {
    let end = offset.checked_add(MDNS_AAAA_RECORD_SIZE)?;
    if end > frame.len() {
        return None;
    }
    frame[offset..offset + MDNS_NAME.len()].copy_from_slice(MDNS_NAME);
    offset += MDNS_NAME.len();
    write_be_u16(frame, offset, 28);
    write_be_u16(frame, offset + 2, class);
    write_be_u32(frame, offset + 4, ttl);
    write_be_u16(frame, offset + 8, ipv6_address.len() as u16);
    frame[offset + 10..end].copy_from_slice(ipv6_address);
    Some(end)
}

fn finish_udp_frame(
    frame: &mut [u8; MAX_FRAME_SIZE],
    payload_len: usize,
    source: &[u8; 16],
    destination: &[u8; 16],
    source_port: u16,
    destination_port: u16,
) -> Option<usize> {
    let udp_len = 8_usize.checked_add(payload_len)?;
    let end = UDP_OFFSET.checked_add(udp_len)?;
    if end > frame.len() || udp_len > u16::MAX as usize {
        return None;
    }
    write_ipv6_header(frame, udp_len, 255, source, destination);
    frame[20] = 17;
    frame[UDP_OFFSET..DNS_OFFSET].fill(0);
    write_be_u16(frame, UDP_OFFSET, source_port);
    write_be_u16(frame, UDP_OFFSET + 2, destination_port);
    write_be_u16(frame, UDP_OFFSET + 4, udp_len as u16);
    let checksum = ipv6_checksum(17, source, destination, &frame[UDP_OFFSET..end]);
    write_be_u16(
        frame,
        UDP_OFFSET + 6,
        if checksum == 0 { u16::MAX } else { checksum },
    );
    Some(end)
}

fn is_on_link(address: &[u8; 16], ipv6_address: &[u8; 16]) -> bool {
    (address[0] == 0xfe && address[1] & 0xc0 == 0x80) || address[..8] == ipv6_address[..8]
}

fn reply_neighbor_solicitation(
    frame: &mut [u8; MAX_FRAME_SIZE],
    end: usize,
    source_ip: [u8; 16],
    destination_ip: [u8; 16],
    ipv6_address: &[u8; 16],
) -> Option<usize> {
    let target: [u8; 16] = frame[ICMPV6_OFFSET + 8..ICMPV6_OFFSET + 24]
        .try_into()
        .ok()?;
    let assigned_address = if target == *ipv6_address || target == LINK_LOCAL_ADDRESS {
        target
    } else {
        return None;
    };
    let destination_mac: [u8; 6] = frame[..6].try_into().ok()?;
    let duplicate_address_detection = source_ip == [0; 16];
    let solicited_node_address = solicited_node_address(&assigned_address);
    let solicited_node_mac = solicited_node_mac(&assigned_address);
    if source_ip[0] == 0xff
        || (destination_ip == assigned_address && destination_mac != NETWORK_DEVICE_MAC_ADDRESS)
        || (destination_ip == solicited_node_address && destination_mac != solicited_node_mac)
        || (duplicate_address_detection && destination_ip != solicited_node_address)
        || (!duplicate_address_detection
            && destination_ip != assigned_address
            && destination_ip != solicited_node_address)
    {
        return None;
    }

    let mut has_source_link_address = false;
    let mut offset = ICMPV6_OFFSET + 24;
    while offset < end {
        if offset + 2 > end || frame[offset + 1] == 0 {
            return None;
        }
        let option_len = usize::from(frame[offset + 1]) * 8;
        if offset + option_len > end {
            return None;
        }
        has_source_link_address |= frame[offset] == 1;
        offset += option_len;
    }

    let source_mac: [u8; 6] = frame[6..12].try_into().ok()?;
    if duplicate_address_detection && has_source_link_address {
        return None;
    }
    let (destination_mac, reply_destination, flags) = if duplicate_address_detection {
        (ALL_NODES_MAC, ALL_NODES_ADDRESS, 0xa0)
    } else {
        (source_mac, source_ip, 0xe0)
    };

    frame[..6].copy_from_slice(&destination_mac);
    frame[6..12].copy_from_slice(&NETWORK_DEVICE_MAC_ADDRESS);
    frame[12..14].copy_from_slice(&[0x86, 0xdd]);
    write_ipv6_header(frame, 32, 255, &assigned_address, &reply_destination);
    frame[ICMPV6_OFFSET..ICMPV6_OFFSET + 32].fill(0);
    frame[ICMPV6_OFFSET] = 136;
    frame[ICMPV6_OFFSET + 4] = flags;
    frame[ICMPV6_OFFSET + 8..ICMPV6_OFFSET + 24].copy_from_slice(&assigned_address);
    frame[ICMPV6_OFFSET + 24] = 2;
    frame[ICMPV6_OFFSET + 25] = 1;
    frame[ICMPV6_OFFSET + 26..ICMPV6_OFFSET + 32].copy_from_slice(&NETWORK_DEVICE_MAC_ADDRESS);
    let reply_end = ICMPV6_OFFSET + 32;
    let checksum = icmpv6_checksum(
        &assigned_address,
        &reply_destination,
        &frame[ICMPV6_OFFSET..reply_end],
    );
    frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].copy_from_slice(&checksum.to_be_bytes());
    Some(reply_end)
}

fn valid_router_solicitation(
    frame: &[u8; MAX_FRAME_SIZE],
    end: usize,
    source_ip: [u8; 16],
    destination_ip: [u8; 16],
) -> bool {
    if end < ICMPV6_OFFSET + 8
        || source_ip[0] == 0xff
        || destination_ip != ALL_ROUTERS_ADDRESS
        || frame[..6] != ALL_ROUTERS_MAC
    {
        return false;
    }

    let unspecified_source = source_ip == [0; 16];
    let mut offset = ICMPV6_OFFSET + 8;
    while offset < end {
        if offset + 2 > end || frame[offset + 1] == 0 {
            return false;
        }
        let option_len = usize::from(frame[offset + 1]) * 8;
        if offset + option_len > end || (unspecified_source && frame[offset] == 1) {
            return false;
        }
        offset += option_len;
    }
    true
}

fn write_router_advertisement(frame: &mut [u8; MAX_FRAME_SIZE], ipv6_address: &[u8; 16]) -> usize {
    const PAYLOAD_LEN: usize = 64;
    const SLLA: usize = ICMPV6_OFFSET + 16;
    const MTU: usize = SLLA + 8;
    const PREFIX: usize = MTU + 8;

    let end = ICMPV6_OFFSET + PAYLOAD_LEN;
    frame[..end].fill(0);
    frame[..6].copy_from_slice(&ALL_NODES_MAC);
    frame[6..12].copy_from_slice(&NETWORK_DEVICE_MAC_ADDRESS);
    frame[12..14].copy_from_slice(&[0x86, 0xdd]);
    write_ipv6_header(
        frame,
        PAYLOAD_LEN,
        255,
        &LINK_LOCAL_ADDRESS,
        &ALL_NODES_ADDRESS,
    );

    frame[ICMPV6_OFFSET] = 134;
    frame[ICMPV6_OFFSET + 4] = 64;
    // Do not install this non-forwarding USB peer as the host's default route.

    frame[SLLA] = 1;
    frame[SLLA + 1] = 1;
    frame[SLLA + 2..SLLA + 8].copy_from_slice(&NETWORK_DEVICE_MAC_ADDRESS);

    frame[MTU] = 5;
    frame[MTU + 1] = 1;
    frame[MTU + 4..MTU + 8].copy_from_slice(&1500_u32.to_be_bytes());

    frame[PREFIX] = 3;
    frame[PREFIX + 1] = 4;
    frame[PREFIX + 2] = 64;
    frame[PREFIX + 3] = 0xc0;
    // Infinite lifetimes avoid a periodic RA timer in this polling demo.
    frame[PREFIX + 4..PREFIX + 12].fill(0xff);
    frame[PREFIX + 16..PREFIX + 24].copy_from_slice(&ipv6_address[..8]);

    let checksum = icmpv6_checksum(
        &LINK_LOCAL_ADDRESS,
        &ALL_NODES_ADDRESS,
        &frame[ICMPV6_OFFSET..end],
    );
    frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].copy_from_slice(&checksum.to_be_bytes());
    end
}

fn solicited_node_address(ipv6_address: &[u8; 16]) -> [u8; 16] {
    [
        0xff,
        0x02,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        0xff,
        ipv6_address[13],
        ipv6_address[14],
        ipv6_address[15],
    ]
}

fn solicited_node_mac(ipv6_address: &[u8; 16]) -> [u8; 6] {
    [
        0x33,
        0x33,
        0xff,
        ipv6_address[13],
        ipv6_address[14],
        ipv6_address[15],
    ]
}

fn write_ipv6_header(
    frame: &mut [u8],
    payload_len: usize,
    hop_limit: u8,
    source: &[u8; 16],
    destination: &[u8; 16],
) {
    frame[14..54].fill(0);
    frame[14] = 0x60;
    frame[18..20].copy_from_slice(&(payload_len as u16).to_be_bytes());
    frame[20] = 58;
    frame[21] = hop_limit;
    frame[22..38].copy_from_slice(source);
    frame[38..54].copy_from_slice(destination);
}

fn icmpv6_checksum(source: &[u8; 16], destination: &[u8; 16], payload: &[u8]) -> u16 {
    ipv6_checksum(58, source, destination, payload)
}

fn ipv6_checksum(
    next_header: u8,
    source: &[u8; 16],
    destination: &[u8; 16],
    payload: &[u8],
) -> u16 {
    let mut sum = checksum_words(0, source);
    sum = checksum_words(sum, destination);
    sum += payload.len() as u32;
    sum += u32::from(next_header);
    sum = checksum_words(sum, payload);
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

fn checksum_words(mut sum: u32, bytes: &[u8]) -> u32 {
    let (words, remainder) = bytes.as_chunks::<2>();
    for word in words {
        sum += u32::from(u16::from_be_bytes(*word));
    }
    if let [last] = remainder {
        sum += u32::from(*last) << 8;
    }
    sum
}

fn get_u16(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([input[offset], input[offset + 1]])
}

fn read_be_u16(input: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    Some(u16::from_be_bytes(input.get(offset..end)?.try_into().ok()?))
}

fn read_be_u32(input: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    Some(u32::from_be_bytes(input.get(offset..end)?.try_into().ok()?))
}

fn write_be_u16(output: &mut [u8], offset: usize, value: u16) {
    output[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
}

fn write_be_u32(output: &mut [u8], offset: usize, value: u32) {
    output[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}

fn put_u16(output: &mut [u8], offset: usize, value: u16) {
    output[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}
