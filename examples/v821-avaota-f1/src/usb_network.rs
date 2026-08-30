//! Minimal CDC-NCM Ethernet device with IPv6 SLAAC.

use crate::usb::{NETWORK_DEVICE_MAC_ADDRESS, UsbNetworkTransport};

const ALL_NODES_ADDRESS: [u8; 16] = [0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const ALL_NODES_MAC: [u8; 6] = [0x33, 0x33, 0, 0, 0, 1];
const ALL_ROUTERS_ADDRESS: [u8; 16] = [0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
const ALL_ROUTERS_MAC: [u8; 6] = [0x33, 0x33, 0, 0, 0, 2];
const LINK_LOCAL_ADDRESS: [u8; 16] = [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

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
const ICMPV6_OFFSET: usize = ETHERNET_HEADER_SIZE + IPV6_HEADER_SIZE;

/// Polling USB-NCM endpoint that answers IPv6 neighbor discovery and echo.
pub struct UsbNetwork {
    transport: UsbNetworkTransport,
    receive_ntb: [u8; MAX_NTB_SIZE],
    receive_len: usize,
    dropping_ntb: bool,
    transmit_ntb: [u8; DATAGRAM_OFFSET + MAX_FRAME_SIZE],
    frame: [u8; MAX_FRAME_SIZE],
    ipv6_address: [u8; 16],
    sequence: u16,
}

impl UsbNetwork {
    /// Maps V821 USB0 after BootROM hands control to the E907 payload.
    ///
    /// # Safety
    ///
    /// USB0, APP-CCU, and AON-CCU must be exclusively owned by this E907
    /// payload with interrupts disabled.
    pub unsafe fn from_v821_mmio(ipv6_address: [u8; 16]) -> Self {
        Self {
            // SAFETY: forwarded from this function's ownership contract.
            transport: unsafe { UsbNetworkTransport::from_v821_mmio() },
            receive_ntb: [0; MAX_NTB_SIZE],
            receive_len: 0,
            dropping_ntb: false,
            transmit_ntb: [0; DATAGRAM_OFFSET + MAX_FRAME_SIZE],
            frame: [0; MAX_FRAME_SIZE],
            ipv6_address,
            sequence: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.receive_len = 0;
        self.dropping_ntb = false;
        self.sequence = 0;
        self.transport.initialize();
    }

    /// Services USB traffic and reports a host-requested safe removal.
    pub fn poll(&mut self) -> bool {
        self.poll_once();
        self.transport.take_network_exit_requested()
    }

    fn poll_once(&mut self) {
        if self.transport.take_network_function_reset() {
            self.receive_len = 0;
            self.dropping_ntb = false;
            self.sequence = 0;
        }
        if self.transport.take_network_link_pending() {
            self.transport.notify_network_link_up();
            let frame_len = write_router_advertisement(&mut self.frame, &self.ipv6_address);
            self.send_frame(frame_len);
        }

        let mut packet = [0; USB_PACKET_SIZE];
        let received = self.transport.poll_packet(&mut packet);
        if !self.transport.network_data_active() {
            self.receive_len = 0;
            return;
        }
        let Some(count) = received else {
            return;
        };
        if self.dropping_ntb {
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return;
        }
        if count == 0 || self.receive_len + count > self.receive_ntb.len() {
            self.receive_len = 0;
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return;
        }

        self.receive_ntb[self.receive_len..self.receive_len + count]
            .copy_from_slice(&packet[..count]);
        self.receive_len += count;

        if self.receive_len < NTH16_SIZE {
            if count < USB_PACKET_SIZE {
                self.receive_len = 0;
            }
            return;
        }

        let block_len = get_u16(&self.receive_ntb, 8) as usize;
        if !(DATAGRAM_OFFSET..=MAX_NTB_SIZE).contains(&block_len) || self.receive_len > block_len {
            self.receive_len = 0;
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return;
        }
        if self.receive_len < block_len {
            if count < USB_PACKET_SIZE {
                self.receive_len = 0;
            }
            return;
        }

        let frame_len = decode_ntb(&self.receive_ntb[..self.receive_len], &mut self.frame);
        self.receive_len = 0;
        let Some(frame_len) = frame_len else {
            self.dropping_ntb = count == USB_PACKET_SIZE;
            return;
        };
        let Some(reply_len) = reply_ipv6(&mut self.frame, frame_len, &self.ipv6_address) else {
            return;
        };

        self.send_frame(reply_len);
    }

    fn send_frame(&mut self, frame_len: usize) {
        let ntb_len = encode_ntb(
            &self.frame[..frame_len],
            &mut self.transmit_ntb,
            self.sequence,
        );
        self.sequence = self.sequence.wrapping_add(1);
        self.transport.write(&self.transmit_ntb[..ntb_len]);
        if ntb_len.is_multiple_of(USB_PACKET_SIZE) {
            self.transport.write_zero_length_packet();
        }
    }
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
    if received < ICMPV6_OFFSET + 8
        || frame[12..14] != [0x86, 0xdd]
        || frame[14] >> 4 != 6
        || frame[20] != 58
        || frame[6] & 1 != 0
    {
        return None;
    }
    let destination_mac: [u8; 6] = frame[..6].try_into().ok()?;
    if destination_mac != NETWORK_DEVICE_MAC_ADDRESS && destination_mac[0] & 1 == 0 {
        return None;
    }

    let payload_len = u16::from_be_bytes(frame[18..20].try_into().ok()?) as usize;
    let end = ICMPV6_OFFSET.checked_add(payload_len)?;
    if payload_len < 8 || end > received || IPV6_HEADER_SIZE + payload_len > 1500 {
        return None;
    }

    let source_ip: [u8; 16] = frame[22..38].try_into().ok()?;
    let destination_ip: [u8; 16] = frame[38..54].try_into().ok()?;
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
    let mut sum = checksum_words(0, source);
    sum = checksum_words(sum, destination);
    sum += payload.len() as u32;
    sum += 58;
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

fn put_u16(output: &mut [u8], offset: usize, value: u16) {
    output[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IPV6_ADDRESS: [u8; 16] =
        [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

    #[test]
    fn ntb16_round_trip() {
        let mut input = [0; MAX_FRAME_SIZE];
        for (index, byte) in input[..86].iter_mut().enumerate() {
            *byte = index as u8;
        }
        let mut ntb = [0; MAX_NTB_SIZE];
        let length = encode_ntb(&input[..86], &mut ntb, 7);
        let mut output = [0; MAX_FRAME_SIZE];
        assert_eq!(decode_ntb(&ntb[..length], &mut output), Some(86));
        assert_eq!(&output[..86], &input[..86]);
        assert_eq!(get_u16(&ntb, 6), 7);
        assert_eq!(get_u16(&ntb, NTH16_SIZE + 8) as usize, DATAGRAM_OFFSET);
        assert!((DATAGRAM_OFFSET + ETHERNET_HEADER_SIZE).is_multiple_of(4));
    }

    #[test]
    fn rejects_datagram_overlapping_ndp() {
        let frame = [0xa5; ETHERNET_HEADER_SIZE];
        let mut ntb = [0; MAX_NTB_SIZE];
        let length = encode_ntb(&frame, &mut ntb, 0);
        put_u16(&mut ntb, NTH16_SIZE + 8, 14);
        put_u16(&mut ntb, NTH16_SIZE + 10, ETHERNET_HEADER_SIZE as u16);
        let mut output = [0; MAX_FRAME_SIZE];
        assert_eq!(decode_ntb(&ntb[..length], &mut output), None);
    }

    #[test]
    fn replies_to_exact_packet_echo_request() {
        let mut frame = [0; MAX_FRAME_SIZE];
        let request: [u8; 64] = [
            0x02, 0xa0, 0xf1, 0x82, 0x10, 0x01, 0x02, 0xa0, 0xf1, 0x82, 0x10, 0x02, 0x86, 0xdd,
            0x60, 0, 0, 0, 0, 0x0a, 0x3a, 0x40, 0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2, 0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0x80, 0, 0xb0,
            0xae, 0x12, 0x34, 0, 1, 0x61, 0x62,
        ];
        frame[..request.len()].copy_from_slice(&request);
        assert_eq!(
            reply_ipv6(&mut frame, request.len(), &TEST_IPV6_ADDRESS),
            Some(64)
        );
        assert_eq!(frame[54], 129);
        assert_eq!(&frame[56..58], &[0xaf, 0xae]);
        assert_eq!(&frame[..6], &request[6..12]);
        assert_eq!(&frame[6..12], &NETWORK_DEVICE_MAC_ADDRESS);
    }

    #[test]
    fn replies_to_assigned_address_solicitations() {
        let host_mac = crate::usb::NETWORK_HOST_MAC_ADDRESS;
        let host_ip = [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
        for target in [TEST_IPV6_ADDRESS, LINK_LOCAL_ADDRESS] {
            let solicited_node_address = solicited_node_address(&target);
            let solicited_node_mac = solicited_node_mac(&target);
            let mut frame = [0; MAX_FRAME_SIZE];
            frame[..6].copy_from_slice(&solicited_node_mac);
            frame[6..12].copy_from_slice(&host_mac);
            frame[12..14].copy_from_slice(&[0x86, 0xdd]);
            write_ipv6_header(&mut frame, 32, 255, &host_ip, &solicited_node_address);
            frame[ICMPV6_OFFSET] = 135;
            frame[ICMPV6_OFFSET + 8..ICMPV6_OFFSET + 24].copy_from_slice(&target);
            frame[ICMPV6_OFFSET + 24] = 1;
            frame[ICMPV6_OFFSET + 25] = 1;
            frame[ICMPV6_OFFSET + 26..ICMPV6_OFFSET + 32].copy_from_slice(&host_mac);
            let checksum = icmpv6_checksum(
                &host_ip,
                &solicited_node_address,
                &frame[ICMPV6_OFFSET..ICMPV6_OFFSET + 32],
            );
            frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].copy_from_slice(&checksum.to_be_bytes());

            assert_eq!(reply_ipv6(&mut frame, 86, &TEST_IPV6_ADDRESS), Some(86));
            assert_eq!(frame[ICMPV6_OFFSET], 136);
            assert_eq!(frame[ICMPV6_OFFSET + 4], 0xe0);
            assert_eq!(&frame[22..38], &target);
            assert_eq!(
                icmpv6_checksum(&target, &host_ip, &frame[ICMPV6_OFFSET..ICMPV6_OFFSET + 32]),
                0
            );
        }
    }

    #[test]
    fn router_solicitation_returns_slaac_advertisement() {
        let mut frame = router_solicitation([0; 16], false);
        assert_eq!(&frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4], &[0x7b, 0xb8]);
        assert_eq!(reply_ipv6(&mut frame, 62, &TEST_IPV6_ADDRESS), Some(118));
        assert_eq!(&frame[..6], &ALL_NODES_MAC);
        assert_eq!(&frame[6..12], &NETWORK_DEVICE_MAC_ADDRESS);
        assert_eq!(&frame[22..38], &LINK_LOCAL_ADDRESS);
        assert_eq!(&frame[38..54], &ALL_NODES_ADDRESS);
        assert_eq!(frame[21], 255);
        assert_eq!(frame[ICMPV6_OFFSET], 134);
        assert_eq!(frame[ICMPV6_OFFSET + 4], 64);
        assert_eq!(&frame[ICMPV6_OFFSET + 5..ICMPV6_OFFSET + 16], &[0; 11]);

        let slla = ICMPV6_OFFSET + 16;
        assert_eq!(&frame[slla..slla + 2], &[1, 1]);
        assert_eq!(&frame[slla + 2..slla + 8], &NETWORK_DEVICE_MAC_ADDRESS);
        let mtu = slla + 8;
        assert_eq!(&frame[mtu..mtu + 4], &[5, 1, 0, 0]);
        assert_eq!(&frame[mtu + 4..mtu + 8], &1500_u32.to_be_bytes());
        let prefix = mtu + 8;
        assert_eq!(&frame[prefix..prefix + 4], &[3, 4, 64, 0xc0]);
        assert_eq!(&frame[prefix + 4..prefix + 12], &[0xff; 8]);
        assert_eq!(&frame[prefix + 12..prefix + 16], &[0; 4]);
        assert_eq!(&frame[prefix + 16..prefix + 24], &TEST_IPV6_ADDRESS[..8]);
        assert_eq!(&frame[prefix + 24..prefix + 32], &[0; 8]);
        assert_eq!(&frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4], &[0xba, 0x7f]);
        assert_eq!(
            icmpv6_checksum(
                &LINK_LOCAL_ADDRESS,
                &ALL_NODES_ADDRESS,
                &frame[ICMPV6_OFFSET..118]
            ),
            0
        );

        let mut ntb = [0; MAX_NTB_SIZE];
        let ntb_len = encode_ntb(&frame[..118], &mut ntb, 0);
        let mut decoded = [0; MAX_FRAME_SIZE];
        assert_eq!(ntb_len, 148);
        assert_eq!(decode_ntb(&ntb[..ntb_len], &mut decoded), Some(118));
        assert_eq!(&decoded[..118], &frame[..118]);
    }

    #[test]
    fn accepts_link_local_router_solicitation_with_slla() {
        let host_ip = [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
        let mut frame = router_solicitation(host_ip, true);
        assert_eq!(reply_ipv6(&mut frame, 70, &TEST_IPV6_ADDRESS), Some(118));
    }

    #[test]
    fn rejects_unspecified_router_solicitation_with_slla() {
        let mut frame = router_solicitation([0; 16], true);
        assert_eq!(reply_ipv6(&mut frame, 70, &TEST_IPV6_ADDRESS), None);
    }

    #[test]
    fn rejects_malformed_router_solicitations() {
        let host_ip = [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];

        let mut frame = router_solicitation(host_ip, true);
        frame[21] = 254;
        assert_eq!(reply_ipv6(&mut frame, 70, &TEST_IPV6_ADDRESS), None);

        let mut frame = router_solicitation(host_ip, true);
        frame[0] ^= 1;
        assert_eq!(reply_ipv6(&mut frame, 70, &TEST_IPV6_ADDRESS), None);

        let mut frame = router_solicitation(host_ip, true);
        frame[ICMPV6_OFFSET + 9] = 0;
        frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].fill(0);
        let checksum = icmpv6_checksum(
            &host_ip,
            &ALL_ROUTERS_ADDRESS,
            &frame[ICMPV6_OFFSET..ICMPV6_OFFSET + 16],
        );
        frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].copy_from_slice(&checksum.to_be_bytes());
        assert_eq!(reply_ipv6(&mut frame, 70, &TEST_IPV6_ADDRESS), None);
    }

    #[test]
    fn ignores_host_duplicate_address_detection() {
        let host_address = [
            0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0x4d, 0x3a, 0x91, 0x62, 0x77, 0x18, 0xaa, 0x04,
        ];
        let destination_ip = solicited_node_address(&host_address);
        let mut frame = [0; MAX_FRAME_SIZE];
        frame[..6].copy_from_slice(&solicited_node_mac(&host_address));
        frame[6..12].copy_from_slice(&crate::usb::NETWORK_HOST_MAC_ADDRESS);
        frame[12..14].copy_from_slice(&[0x86, 0xdd]);
        write_ipv6_header(&mut frame, 24, 255, &[0; 16], &destination_ip);
        frame[ICMPV6_OFFSET] = 135;
        frame[ICMPV6_OFFSET + 8..ICMPV6_OFFSET + 24].copy_from_slice(&host_address);
        let checksum = icmpv6_checksum(
            &[0; 16],
            &destination_ip,
            &frame[ICMPV6_OFFSET..ICMPV6_OFFSET + 24],
        );
        frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].copy_from_slice(&checksum.to_be_bytes());
        assert_eq!(reply_ipv6(&mut frame, 78, &TEST_IPV6_ADDRESS), None);
    }

    #[test]
    fn rejects_all_short_frames() {
        let mut frame = [0; MAX_FRAME_SIZE];
        for length in 0..ICMPV6_OFFSET + 8 {
            assert_eq!(reply_ipv6(&mut frame, length, &TEST_IPV6_ADDRESS), None);
        }
    }

    #[test]
    fn rejects_truncated_icmpv6_echo_header() {
        let source_ip = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x52, 0x09];
        let mut frame = [0; MAX_FRAME_SIZE];
        frame[..6].copy_from_slice(&NETWORK_DEVICE_MAC_ADDRESS);
        frame[6..12].copy_from_slice(&crate::usb::NETWORK_HOST_MAC_ADDRESS);
        frame[12..14].copy_from_slice(&[0x86, 0xdd]);
        write_ipv6_header(&mut frame, 2, 64, &source_ip, &TEST_IPV6_ADDRESS);
        frame[ICMPV6_OFFSET..ICMPV6_OFFSET + 2].copy_from_slice(&[128, 0]);
        assert_eq!(
            icmpv6_checksum(
                &source_ip,
                &TEST_IPV6_ADDRESS,
                &frame[ICMPV6_OFFSET..ICMPV6_OFFSET + 2]
            ),
            0
        );
        assert_eq!(reply_ipv6(&mut frame, 62, &TEST_IPV6_ADDRESS), None);
    }

    fn router_solicitation(source_ip: [u8; 16], source_link_address: bool) -> [u8; MAX_FRAME_SIZE] {
        let mut frame = [0; MAX_FRAME_SIZE];
        let payload_len = if source_link_address { 16 } else { 8 };
        frame[..6].copy_from_slice(&ALL_ROUTERS_MAC);
        frame[6..12].copy_from_slice(&crate::usb::NETWORK_HOST_MAC_ADDRESS);
        frame[12..14].copy_from_slice(&[0x86, 0xdd]);
        write_ipv6_header(
            &mut frame,
            payload_len,
            255,
            &source_ip,
            &ALL_ROUTERS_ADDRESS,
        );
        frame[ICMPV6_OFFSET] = 133;
        if source_link_address {
            frame[ICMPV6_OFFSET + 8] = 1;
            frame[ICMPV6_OFFSET + 9] = 1;
            frame[ICMPV6_OFFSET + 10..ICMPV6_OFFSET + 16]
                .copy_from_slice(&crate::usb::NETWORK_HOST_MAC_ADDRESS);
        }
        let checksum = icmpv6_checksum(
            &source_ip,
            &ALL_ROUTERS_ADDRESS,
            &frame[ICMPV6_OFFSET..ICMPV6_OFFSET + payload_len],
        );
        frame[ICMPV6_OFFSET + 2..ICMPV6_OFFSET + 4].copy_from_slice(&checksum.to_be_bytes());
        frame
    }
}
