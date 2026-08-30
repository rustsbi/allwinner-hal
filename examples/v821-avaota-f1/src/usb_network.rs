//! Minimal CDC-NCM Ethernet device with IPv6 SLAAC and mDNS.

use crate::usb::{NETWORK_DEVICE_MAC_ADDRESS, UsbNetworkTransport};

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

/// Polling USB-NCM endpoint that answers IPv6 neighbor discovery, echo, and mDNS.
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IPV6_ADDRESS: [u8; 16] =
        [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    const TEST_HOST_LINK_LOCAL: [u8; 16] = [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];

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
    fn replies_to_mdns_aaaa_question() {
        let (mut frame, request_end) = mdns_question(28, 1, MDNS_PORT, 0);
        assert_eq!(request_end, 95);
        assert_eq!(read_be_u16(&frame, UDP_OFFSET + 6), Some(0x2d77));
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            Some(117)
        );
        assert_eq!(&frame[..6], &MDNS_MAC);
        assert_eq!(&frame[6..12], &NETWORK_DEVICE_MAC_ADDRESS);
        assert_eq!(&frame[22..38], &LINK_LOCAL_ADDRESS);
        assert_eq!(&frame[38..54], &MDNS_ADDRESS);
        assert_eq!(frame[20], 17);
        assert_eq!(frame[21], 255);
        assert_eq!(read_be_u16(&frame, UDP_OFFSET), Some(MDNS_PORT));
        assert_eq!(read_be_u16(&frame, UDP_OFFSET + 2), Some(MDNS_PORT));
        assert_eq!(read_be_u16(&frame, UDP_OFFSET + 4), Some(63));
        assert_eq!(read_be_u16(&frame, UDP_OFFSET + 6), Some(0x669d));
        assert_eq!(
            ipv6_checksum(
                17,
                &LINK_LOCAL_ADDRESS,
                &MDNS_ADDRESS,
                &frame[UDP_OFFSET..117]
            ),
            0
        );

        assert_eq!(
            &frame[DNS_OFFSET..DNS_OFFSET + 12],
            &[0, 0, 0x84, 0, 0, 0, 0, 1, 0, 0, 0, 0]
        );
        let record = DNS_OFFSET + 12;
        assert_eq!(&frame[record..record + MDNS_NAME.len()], MDNS_NAME);
        let fields = record + MDNS_NAME.len();
        assert_eq!(read_be_u16(&frame, fields), Some(28));
        assert_eq!(read_be_u16(&frame, fields + 2), Some(0x8001));
        assert_eq!(read_be_u32(&frame, fields + 4), Some(120));
        assert_eq!(read_be_u16(&frame, fields + 8), Some(16));
        assert_eq!(&frame[fields + 10..117], &TEST_IPV6_ADDRESS);

        let mut ntb = [0; MAX_NTB_SIZE];
        let ntb_len = encode_ntb(&frame[..117], &mut ntb, 0);
        assert_eq!(ntb_len, 147);
    }

    #[test]
    fn accepts_case_insensitive_compressed_multi_question() {
        let mut frame = [0; MAX_FRAME_SIZE];
        write_be_u16(&mut frame, DNS_OFFSET + 4, 2);
        let mut offset = DNS_OFFSET + 12;
        frame[offset..offset + MDNS_NAME.len()].copy_from_slice(b"\x09AVAOTA-F1\x05LOCAL\0");
        offset += MDNS_NAME.len();
        write_be_u16(&mut frame, offset, 1);
        write_be_u16(&mut frame, offset + 2, 1);
        offset += 4;
        frame[offset..offset + 2].copy_from_slice(&[0xc0, 0x0c]);
        write_be_u16(&mut frame, offset + 2, 28);
        write_be_u16(&mut frame, offset + 4, 0x8001);
        offset += 6;
        let end = finish_mdns_query(&mut frame, offset - DNS_OFFSET, MDNS_PORT);

        assert_eq!(reply_ipv6(&mut frame, end, &TEST_IPV6_ADDRESS), Some(117));
        assert_eq!(&frame[..6], &MDNS_MAC);
        assert_eq!(&frame[38..54], &MDNS_ADDRESS);
    }

    #[test]
    fn replies_to_legacy_mdns_query_by_unicast() {
        let source_port = 49_152;
        let query_id = 0x4a71;
        let (mut frame, request_end) = mdns_question(28, 1, source_port, query_id);
        let question: [u8; 21] = frame[DNS_OFFSET + 12..request_end].try_into().unwrap();

        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            Some(138)
        );
        assert_eq!(&frame[..6], &crate::usb::NETWORK_HOST_MAC_ADDRESS);
        assert_eq!(&frame[22..38], &LINK_LOCAL_ADDRESS);
        assert_eq!(&frame[38..54], &TEST_HOST_LINK_LOCAL);
        assert_eq!(read_be_u16(&frame, UDP_OFFSET), Some(MDNS_PORT));
        assert_eq!(read_be_u16(&frame, UDP_OFFSET + 2), Some(source_port));
        assert_eq!(read_be_u16(&frame, DNS_OFFSET), Some(query_id));
        assert_eq!(read_be_u16(&frame, DNS_OFFSET + 2), Some(0x8400));
        assert_eq!(read_be_u16(&frame, DNS_OFFSET + 4), Some(1));
        assert_eq!(read_be_u16(&frame, DNS_OFFSET + 6), Some(1));
        assert_eq!(&frame[DNS_OFFSET + 12..DNS_OFFSET + 33], &question[..]);

        let fields = DNS_OFFSET + 33 + MDNS_NAME.len();
        assert_eq!(read_be_u16(&frame, fields + 2), Some(1));
        assert_eq!(read_be_u32(&frame, fields + 4), Some(10));
        assert_eq!(
            ipv6_checksum(
                17,
                &LINK_LOCAL_ADDRESS,
                &TEST_HOST_LINK_LOCAL,
                &frame[UDP_OFFSET..138]
            ),
            0
        );
    }

    #[test]
    fn suppresses_fresh_known_mdns_answer() {
        let (mut frame, _) = mdns_question(28, 1, MDNS_PORT, 0);
        write_be_u16(&mut frame, DNS_OFFSET + 6, 1);
        let record = DNS_OFFSET + 33;
        frame[record..record + 2].copy_from_slice(&[0xc0, 0x0c]);
        write_be_u16(&mut frame, record + 2, 28);
        write_be_u16(&mut frame, record + 4, 1);
        write_be_u32(&mut frame, record + 6, MDNS_TTL / 2);
        write_be_u16(&mut frame, record + 10, 16);
        frame[record + 12..record + 28].copy_from_slice(&TEST_IPV6_ADDRESS);
        let request_end = finish_mdns_query(&mut frame, 61, MDNS_PORT);
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            None
        );

        write_be_u32(&mut frame, record + 6, MDNS_TTL / 2 - 1);
        let request_end = finish_mdns_query(&mut frame, 61, MDNS_PORT);
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            Some(117)
        );
    }

    #[test]
    fn ignores_a_only_and_malformed_mdns_queries() {
        let (mut frame, request_end) = mdns_question(1, 1, MDNS_PORT, 0);
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            None
        );

        let (mut frame, request_end) = mdns_question(255, 255, MDNS_PORT, 0);
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            Some(117)
        );

        let (mut frame, request_end) = mdns_question(28, 1, MDNS_PORT, 0);
        frame[DNS_OFFSET + 13] ^= 1;
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            None
        );

        let (mut frame, _) = mdns_question(28, 1, MDNS_PORT, 0);
        frame[DNS_OFFSET + 12..DNS_OFFSET + 14].copy_from_slice(&[0xc0, 0x0c]);
        let request_end = finish_mdns_query(&mut frame, 18, MDNS_PORT);
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            None
        );

        let (mut frame, request_end) = mdns_question(28, 1, MDNS_PORT, 0);
        frame[UDP_OFFSET + 6..UDP_OFFSET + 8].fill(0);
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            None
        );

        let (mut frame, _) = mdns_question(28, 1, MDNS_PORT, 0);
        write_be_u16(&mut frame, DNS_OFFSET + 2, 1);
        let request_end = finish_mdns_query(&mut frame, 33, MDNS_PORT);
        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            None
        );
    }

    #[test]
    fn accepts_windows_mdns_hop_limit() {
        let (mut frame, request_end) = mdns_question(28, 1, MDNS_PORT, 0);
        frame[21] = 1;

        assert_eq!(
            reply_ipv6(&mut frame, request_end, &TEST_IPV6_ADDRESS),
            Some(117)
        );
        assert_eq!(frame[21], 255);
    }

    #[test]
    fn rejects_truncated_and_reserved_mdns_names() {
        let (frame, request_end) = mdns_question(28, 1, MDNS_PORT, 0);
        let dns = &frame[DNS_OFFSET..request_end];
        for length in 0..dns.len() {
            assert_eq!(parse_mdns_query(&dns[..length], &TEST_IPV6_ADDRESS), None);
        }

        for reserved in [0x40, 0x80] {
            let mut dns = dns.to_owned();
            dns[12] = reserved;
            assert_eq!(parse_mdns_query(&dns, &TEST_IPV6_ADDRESS), None);
        }
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

    fn mdns_question(
        question_type: u16,
        question_class: u16,
        source_port: u16,
        query_id: u16,
    ) -> ([u8; MAX_FRAME_SIZE], usize) {
        let mut frame = [0; MAX_FRAME_SIZE];
        write_be_u16(&mut frame, DNS_OFFSET, query_id);
        write_be_u16(&mut frame, DNS_OFFSET + 4, 1);
        frame[DNS_OFFSET + 12..DNS_OFFSET + 12 + MDNS_NAME.len()].copy_from_slice(MDNS_NAME);
        let fields = DNS_OFFSET + 12 + MDNS_NAME.len();
        write_be_u16(&mut frame, fields, question_type);
        write_be_u16(&mut frame, fields + 2, question_class);
        let end = finish_mdns_query(&mut frame, 12 + MDNS_NAME.len() + 4, source_port);
        (frame, end)
    }

    fn finish_mdns_query(
        frame: &mut [u8; MAX_FRAME_SIZE],
        dns_len: usize,
        source_port: u16,
    ) -> usize {
        frame[..6].copy_from_slice(&MDNS_MAC);
        frame[6..12].copy_from_slice(&crate::usb::NETWORK_HOST_MAC_ADDRESS);
        frame[12..14].copy_from_slice(&[0x86, 0xdd]);
        finish_udp_frame(
            frame,
            dns_len,
            &TEST_HOST_LINK_LOCAL,
            &MDNS_ADDRESS,
            source_port,
            MDNS_PORT,
        )
        .unwrap()
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
