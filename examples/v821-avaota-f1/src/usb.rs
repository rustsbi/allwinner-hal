//! Polled USB CDC-ACM device for the V821 USB0 controller.
//!
//! The controller layout and initialization sequence are taken from the
//! sun300iw1p1 BootROM and Tina RTOS UDC driver.  This module intentionally
//! keeps the raw-MMIO boundary local: there is one owner, interrupts stay
//! disabled, FIFO/status registers are not exposed as ordinary read/write
//! cells, and W1C registers are acknowledged with exact masks.

use core::{cell::UnsafeCell, mem::offset_of};

const USB0_BASE: usize = 0x4410_0000;
const USB0_PHY_BASE: usize = USB0_BASE + 0x400;
const APP_CCU_BASE: usize = 0x4200_1000;
const HOSC_CONTROL_BASE: usize = 0x4a01_0400;
const COUNTER_LOW: usize = 0x3000_bff8;

const USB_24M_GATE: u32 = 1 << 3;
const USB_HCLK_GATE_RESET: u32 = 1 << 19;
const USB_OTG_GATE_RESET: u32 = 1 << 20;
const USB_PHY_RESET: u32 = 1 << 23;
const HOSC_IS_24_MHZ: u32 = 1 << 31;

const USB_POWER_HS_ENABLE: u8 = 0x20;
const USB_POWER_SOFT_CONNECT: u8 = 0x40;
const USB_POWER_ISO_UPDATE: u8 = 0x80;

const USB_BUS_RESET: u8 = 0x04;
const USB_CSR0_RX_PACKET_READY: u16 = 0x0001;
const USB_CSR0_TX_PACKET_READY: u16 = 0x0002;
const USB_CSR0_SENT_STALL: u16 = 0x0004;
const USB_CSR0_DATA_END: u16 = 0x0008;
const USB_CSR0_SETUP_END: u16 = 0x0010;
const USB_CSR0_SEND_STALL: u16 = 0x0020;
const USB_CSR0_SERVICE_RX_PACKET_READY: u16 = 0x0040;
const USB_CSR0_SERVICE_SETUP_END: u16 = 0x0080;

const USB_TXCSR_TX_PACKET_READY: u16 = 0x0001;
const USB_TXCSR_FLUSH_FIFO: u16 = 0x0008;
const USB_TXCSR_CLEAR_DATA_TOGGLE: u16 = 0x0040;
const USB_TXCSR_MODE: u16 = 0x2000;
const USB_RXCSR_RX_PACKET_READY: u16 = 0x0001;
const USB_RXCSR_FLUSH_FIFO: u16 = 0x0010;
const USB_RXCSR_CLEAR_DATA_TOGGLE: u16 = 0x0080;

const EP0_MAX_PACKET: usize = 64;
const NOTIFY_IN_ENDPOINT: u8 = 1;
const DATA_OUT_ENDPOINT: u8 = 2;
const DATA_IN_ENDPOINT: u8 = 2;

const CDC_SET_LINE_CODING: u8 = 0x20;
const CDC_GET_LINE_CODING: u8 = 0x21;
const CDC_SET_CONTROL_LINE_STATE: u8 = 0x22;
const CDC_SEND_BREAK: u8 = 0x23;

const DEVICE_DESCRIPTOR: [u8; 18] = [
    18, 0x01, 0x00, 0x02, 0xef, 0x02, 0x01, 64, 0x3a, 0x1f, 0x10, 0x82, 0x00, 0x01, 1, 2, 3, 1,
];

// Full-speed CDC-ACM configuration: IAD, control interface, three CDC
// functional descriptors, notification endpoint, and a two-endpoint data
// interface.
const CONFIGURATION_DESCRIPTOR: [u8; 75] = [
    9, 0x02, 75, 0, 2, 1, 0, 0x80, 50, // configuration
    8, 0x0b, 0, 2, 0x02, 0x02, 0x01, 0, // interface association
    9, 0x04, 0, 0, 1, 0x02, 0x02, 0x01, 0, // communications interface
    5, 0x24, 0x00, 0x10, 0x01, // CDC header, version 1.10
    5, 0x24, 0x01, 0x00, 1, // call management, data interface 1
    4, 0x24, 0x02, 0x02, // ACM: line coding/control-line requests
    5, 0x24, 0x06, 0, 1, // union: control 0, subordinate 1
    7, 0x05, 0x81, 0x03, 16, 0, 16, // notification IN
    9, 0x04, 1, 0, 2, 0x0a, 0, 0, 0, // data interface
    7, 0x05, 0x02, 0x02, 64, 0, 0, // bulk OUT
    7, 0x05, 0x82, 0x02, 64, 0, 0, // bulk IN
];

const STRING_LANGUAGE: [u8; 4] = [4, 0x03, 0x09, 0x04];
const STRING_MANUFACTURER: [u8; 16] = [
    16, 0x03, b'R', 0, b'u', 0, b's', 0, b't', 0, b'S', 0, b'B', 0, b'I', 0,
];
const STRING_PRODUCT: [u8; 28] = [
    28, 0x03, b'V', 0, b'8', 0, b'2', 0, b'1', 0, b' ', 0, b'U', 0, b'S', 0, b'B', 0, b' ', 0,
    b'U', 0, b'A', 0, b'R', 0, b'T', 0,
];
const STRING_SERIAL: [u8; 18] = [
    18, 0x03, b'V', 0, b'8', 0, b'2', 0, b'1', 0, b'0', 0, b'0', 0, b'0', 0, b'1', 0,
];

const _: () = assert!(CONFIGURATION_DESCRIPTOR[2] as usize == CONFIGURATION_DESCRIPTOR.len());

#[repr(transparent)]
struct ReadOnly<T: Copy>(UnsafeCell<T>);

impl<T: Copy> ReadOnly<T> {
    #[inline(always)]
    fn read(&self) -> T {
        // SAFETY: `self` points at a source-verified, aligned MMIO register.
        unsafe { core::ptr::read_volatile(self.0.get()) }
    }
}

#[repr(transparent)]
struct ReadWrite<T: Copy>(UnsafeCell<T>);

impl<T: Copy> ReadWrite<T> {
    #[inline(always)]
    fn read(&self) -> T {
        // SAFETY: `self` points at a source-verified, aligned MMIO register.
        unsafe { core::ptr::read_volatile(self.0.get()) }
    }

    #[inline(always)]
    fn write(&self, value: T) {
        // SAFETY: `self` points at a source-verified, aligned MMIO register.
        unsafe { core::ptr::write_volatile(self.0.get(), value) }
    }
}

#[repr(transparent)]
struct WriteOneToClear<T: Copy>(UnsafeCell<T>);

impl<T: Copy> WriteOneToClear<T> {
    #[inline(always)]
    fn status(&self) -> T {
        // SAFETY: status reads are non-destructive for these controller IRQ
        // registers according to the Tina UDC register definitions.
        unsafe { core::ptr::read_volatile(self.0.get()) }
    }

    #[inline(always)]
    fn acknowledge(&self, exact_mask: T) {
        // SAFETY: W1C registers must receive the exact observed mask; callers
        // never use read-modify-write through this type.
        unsafe { core::ptr::write_volatile(self.0.get(), exact_mask) }
    }
}

#[repr(transparent)]
struct Fifo(UnsafeCell<u32>);

impl Fifo {
    #[inline(always)]
    fn read_byte(&self) -> u8 {
        // SAFETY: the controller FIFO accepts byte-wide PIO accesses.
        unsafe { core::ptr::read_volatile(self.0.get().cast::<u8>()) }
    }

    #[inline(always)]
    fn write_byte(&self, value: u8) {
        // SAFETY: the controller FIFO accepts byte-wide PIO accesses.
        unsafe { core::ptr::write_volatile(self.0.get().cast::<u8>(), value) }
    }
}

/// V821 USB0 device-controller register layout used by Boot0 and FEL.
#[repr(C)]
struct UsbRegisters {
    fifo: [Fifo; 4],
    _reserved_010: [u8; 0x30],
    power: ReadWrite<u8>,
    devctl: ReadWrite<u8>,
    index: ReadWrite<u8>,
    vend0: ReadWrite<u8>,
    interrupt_tx: WriteOneToClear<u16>,
    interrupt_rx: WriteOneToClear<u16>,
    interrupt_tx_enable: ReadWrite<u16>,
    interrupt_rx_enable: ReadWrite<u16>,
    interrupt_usb: WriteOneToClear<u8>,
    _reserved_04d: [u8; 3],
    interrupt_usb_enable: ReadWrite<u8>,
    _reserved_051: [u8; 3],
    frame: ReadOnly<u32>,
    _reserved_058: [u8; 0x24],
    test_mode: ReadWrite<u32>,
    tx_max_packet: ReadWrite<u16>,
    tx_csr: ReadWrite<u16>,
    rx_max_packet: ReadWrite<u16>,
    rx_csr: ReadWrite<u16>,
    rx_count: ReadOnly<u16>,
    _reserved_08a: u16,
    tx_type: ReadWrite<u8>,
    tx_interval: ReadWrite<u8>,
    rx_type: ReadWrite<u8>,
    rx_interval: ReadWrite<u8>,
    tx_fifo_size: ReadWrite<u8>,
    _reserved_091: u8,
    tx_fifo_address: ReadWrite<u16>,
    rx_fifo_size: ReadWrite<u8>,
    _reserved_095: u8,
    rx_fifo_address: ReadWrite<u16>,
    function_address: ReadWrite<u8>,
}

#[repr(C)]
struct AppCcuRegisters {
    _reserved_000: [u8; 0x7c],
    usb_24m: ReadWrite<u32>,
    gate0: ReadWrite<u32>,
    _reserved_084: [u8; 0x0c],
    reset0: ReadWrite<u32>,
}

#[repr(C)]
struct HoscControlRegisters {
    _reserved_000: [u8; 4],
    selected_frequency: ReadOnly<u32>,
}

#[repr(C)]
struct UsbPhyRegisters {
    iscr: ReadWrite<u32>,
    _reserved_004: [u8; 0x0c],
    clock_serial: ReadWrite<u32>,
    _reserved_014: [u8; 0x0c],
    control: ReadWrite<u32>,
}

const _: () = {
    assert!(offset_of!(UsbRegisters, power) == 0x40);
    assert!(offset_of!(UsbRegisters, interrupt_tx) == 0x44);
    assert!(offset_of!(UsbRegisters, interrupt_usb) == 0x4c);
    assert!(offset_of!(UsbRegisters, interrupt_usb_enable) == 0x50);
    assert!(offset_of!(UsbRegisters, frame) == 0x54);
    assert!(offset_of!(UsbRegisters, tx_max_packet) == 0x80);
    assert!(offset_of!(UsbRegisters, rx_count) == 0x88);
    assert!(offset_of!(UsbRegisters, tx_fifo_size) == 0x90);
    assert!(offset_of!(UsbRegisters, rx_fifo_size) == 0x94);
    assert!(offset_of!(UsbRegisters, function_address) == 0x98);
    assert!(offset_of!(AppCcuRegisters, usb_24m) == 0x7c);
    assert!(offset_of!(AppCcuRegisters, gate0) == 0x80);
    assert!(offset_of!(AppCcuRegisters, reset0) == 0x90);
    assert!(offset_of!(HoscControlRegisters, selected_frequency) == 0x04);
    assert!(offset_of!(UsbPhyRegisters, clock_serial) == 0x10);
    assert!(offset_of!(UsbPhyRegisters, control) == 0x20);
};

#[derive(Clone, Copy)]
enum TxSource {
    Device,
    Configuration,
    Language,
    Manufacturer,
    Product,
    Serial,
    Reply,
}

#[derive(Clone, Copy)]
enum Ep0State {
    Idle,
    Tx {
        source: TxSource,
        total: usize,
        offset: usize,
        needs_zlp: bool,
    },
    ReceiveLineCoding {
        received: usize,
    },
    ApplyAddress(u8),
}

#[derive(Clone, Copy)]
struct SetupPacket {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
}

impl SetupPacket {
    fn from_bytes(bytes: [u8; 8]) -> Self {
        Self {
            request_type: bytes[0],
            request: bytes[1],
            value: u16::from_le_bytes([bytes[2], bytes[3]]),
            index: u16::from_le_bytes([bytes[4], bytes[5]]),
            length: u16::from_le_bytes([bytes[6], bytes[7]]),
        }
    }
}

/// Exclusive owner of USB0 while the Boot0 payload is running on the E907.
pub struct UsbCdcAcm {
    registers: &'static UsbRegisters,
    app_ccu: &'static AppCcuRegisters,
    hosc: &'static HoscControlRegisters,
    phy: &'static UsbPhyRegisters,
    ep0_state: Ep0State,
    ep0_reply: [u8; 8],
    line_coding: [u8; 7],
    configured: bool,
}

impl UsbCdcAcm {
    /// Maps V821 USB0 and takes exclusive ownership after the BootROM handoff.
    ///
    /// # Safety
    ///
    /// The caller must run on the V821 E907 after BootROM has transferred
    /// control from SPI Boot0 or FEL, with interrupts disabled and no other
    /// core or ISR accessing USB0 or its APP-CCU fields. The address and layout
    /// must match sun300iw1p1/V821 revision P1.
    pub unsafe fn from_v821_mmio() -> Self {
        // SAFETY: all four source-verified blocks are aligned and exclusively
        // owned under the caller's E907/interrupt preconditions.
        let registers = unsafe { &*(USB0_BASE as *const UsbRegisters) };
        let app_ccu = unsafe { &*(APP_CCU_BASE as *const AppCcuRegisters) };
        let hosc = unsafe { &*(HOSC_CONTROL_BASE as *const HoscControlRegisters) };
        let phy = unsafe { &*(USB0_PHY_BASE as *const UsbPhyRegisters) };

        Self {
            registers,
            app_ccu,
            hosc,
            phy,
            ep0_state: Ep0State::Idle,
            ep0_reply: [0; 8],
            // 115200 baud, one stop bit, no parity, eight data bits.
            line_coding: [0x00, 0xc2, 0x01, 0x00, 0, 0, 8],
            configured: false,
        }
    }

    /// Cold-initializes USB0, then reconnects it as a full-speed CDC device.
    pub fn initialize(&mut self) {
        self.initialize_v821_usb0_hardware();

        self.registers
            .power
            .write(self.registers.power.read() & !USB_POWER_SOFT_CONNECT);
        delay_microseconds(250_000);

        // Select the same PIO bus mode used by BootROM FEL.
        self.registers.vend0.write(self.registers.vend0.read() & !1);
        self.registers.interrupt_usb_enable.write(0);
        self.registers.interrupt_tx_enable.write(0);
        self.registers.interrupt_rx_enable.write(0);
        self.acknowledge_all_pending_interrupts();

        self.registers.function_address.write(0);
        self.ep0_state = Ep0State::Idle;
        self.configured = false;
        self.configure_data_endpoints();

        // Full speed keeps the CDC bulk descriptors at the required 64-byte
        // max-packet size and avoids unverified high-speed PHY behavior.
        let power = self.registers.power.read()
            & !(USB_POWER_HS_ENABLE | USB_POWER_ISO_UPDATE | USB_POWER_SOFT_CONNECT);
        self.registers.power.write(power);
        self.registers.interrupt_usb_enable.write(0x07);

        delay_microseconds(1_000);
        self.registers.power.write(power | USB_POWER_SOFT_CONNECT);
    }

    fn initialize_v821_usb0_hardware(&self) {
        // This is BootROM 0x87be's reset/clock sequence. The E907 owns these
        // shared APP-CCU words exclusively here, so each volatile RMW cannot
        // race an ISR, another core, or another driver.
        self.app_ccu
            .reset0
            .write(self.app_ccu.reset0.read() & !USB_PHY_RESET);
        self.app_ccu
            .gate0
            .write(self.app_ccu.gate0.read() & !USB_OTG_GATE_RESET);
        self.app_ccu
            .reset0
            .write(self.app_ccu.reset0.read() & !USB_OTG_GATE_RESET);

        self.app_ccu
            .reset0
            .write(self.app_ccu.reset0.read() & !USB_HCLK_GATE_RESET);
        delay_microseconds(20);
        self.app_ccu
            .gate0
            .write(self.app_ccu.gate0.read() & !USB_HCLK_GATE_RESET);
        delay_microseconds(20);

        self.app_ccu
            .reset0
            .write(self.app_ccu.reset0.read() | USB_PHY_RESET);
        delay_microseconds(50);
        self.app_ccu
            .reset0
            .write(self.app_ccu.reset0.read() | USB_OTG_GATE_RESET);
        delay_microseconds(100);
        self.app_ccu
            .gate0
            .write(self.app_ccu.gate0.read() | USB_OTG_GATE_RESET);
        delay_microseconds(50);

        self.app_ccu
            .reset0
            .write(self.app_ccu.reset0.read() | USB_HCLK_GATE_RESET);
        delay_microseconds(20);
        self.app_ccu
            .gate0
            .write(self.app_ccu.gate0.read() | USB_HCLK_GATE_RESET);
        delay_microseconds(20);
        self.app_ccu
            .usb_24m
            .write(self.app_ccu.usb_24m.read() | USB_24M_GATE);

        // BootROM records its normalized oscillator choice in bit 31:
        // set means 24 MHz; clear means 40 MHz.
        let serial_byte = if self.hosc.selected_frequency.read() & HOSC_IS_24_MHZ != 0 {
            0x14_u32
        } else {
            0x0c_u32
        };
        for selector in 11_u32..19 {
            let data_bit = (serial_byte >> (selector - 11)) & 1;
            let value = (self.phy.clock_serial.read() & 0xffff_007e)
                | (selector << 8)
                | (data_bit << 7)
                | 3;
            self.phy.clock_serial.write(value);
            delay_microseconds(50);
        }

        // BootROM 0x8768 selects USB0's PIO path before PHY setup.
        self.phy.control.write(self.phy.control.read() | 1);
        self.phy
            .clock_serial
            .write(self.phy.clock_serial.read() & !(1 << 3));
        delay_microseconds(20);

        self.phy.iscr.write(self.phy.iscr.read() | 0x0000_c000);
        self.phy.iscr.write(self.phy.iscr.read() | 0x0001_0c00);
        if self.registers.devctl.read() & 0x18 != 0x18 {
            self.phy.iscr.write(self.phy.iscr.read() | 0x3000);
        }
        self.phy.iscr.write(self.phy.iscr.read() & !0x0001_0000);
    }

    pub fn is_configured(&self) -> bool {
        self.configured
    }

    /// Services bus/EP0 state and returns at most one received bulk packet.
    pub fn poll(&mut self, output: &mut [u8; 64]) -> usize {
        if self.service_bus_and_control() {
            return 0;
        }
        if !self.configured {
            return 0;
        }

        self.select_endpoint(DATA_OUT_ENDPOINT);
        if self.registers.rx_csr.read() & USB_RXCSR_RX_PACKET_READY == 0 {
            return 0;
        }

        let count = (self.registers.rx_count.read() as usize).min(output.len());
        for byte in &mut output[..count] {
            *byte = self.registers.fifo[DATA_OUT_ENDPOINT as usize].read_byte();
        }
        // RXCSR.RXPKTRDY is cleared by writing zero.  No persistent RXCSR
        // configuration bits are needed for this PIO bulk endpoint.
        self.registers.rx_csr.write(0);
        self.registers
            .interrupt_rx
            .acknowledge(1_u16 << DATA_OUT_ENDPOINT);
        count
    }

    /// Writes a byte stream through the CDC bulk-IN endpoint.
    pub fn write(&mut self, mut bytes: &[u8]) {
        while self.configured && !bytes.is_empty() {
            while self.configured {
                self.select_endpoint(DATA_IN_ENDPOINT);
                if self.registers.tx_csr.read() & USB_TXCSR_TX_PACKET_READY == 0 {
                    break;
                }
                self.service_bus_and_control();
            }
            if !self.configured {
                return;
            }

            let count = bytes.len().min(64);
            self.select_endpoint(DATA_IN_ENDPOINT);
            for byte in &bytes[..count] {
                self.registers.fifo[DATA_IN_ENDPOINT as usize].write_byte(*byte);
            }
            self.registers
                .tx_csr
                .write(USB_TXCSR_MODE | USB_TXCSR_TX_PACKET_READY);
            bytes = &bytes[count..];
        }
    }

    /// Returns true when a bus reset was handled and the caller should stop
    /// processing the current packet.
    fn service_bus_and_control(&mut self) -> bool {
        let usb_status = self.registers.interrupt_usb.status();
        if usb_status != 0 {
            self.registers.interrupt_usb.acknowledge(usb_status);
        }
        if usb_status & USB_BUS_RESET != 0 {
            self.handle_bus_reset();
            return true;
        }

        let tx_status = self.registers.interrupt_tx.status();
        if tx_status & 1 != 0 {
            self.registers.interrupt_tx.acknowledge(1);
            self.handle_endpoint_zero();
        }
        let completed_nonzero = tx_status & !1;
        if completed_nonzero != 0 {
            self.registers.interrupt_tx.acknowledge(completed_nonzero);
        }
        false
    }

    fn handle_bus_reset(&mut self) {
        self.registers.function_address.write(0);
        self.ep0_state = Ep0State::Idle;
        self.configured = false;
        self.configure_data_endpoints();
        self.select_endpoint(0);
    }

    fn handle_endpoint_zero(&mut self) {
        self.select_endpoint(0);
        let csr0 = self.registers.tx_csr.read();

        if csr0 & USB_CSR0_SENT_STALL != 0 {
            self.registers.tx_csr.write(0);
            self.ep0_state = Ep0State::Idle;
            return;
        }
        if csr0 & USB_CSR0_SETUP_END != 0 {
            self.registers.tx_csr.write(USB_CSR0_SERVICE_SETUP_END);
            self.ep0_state = Ep0State::Idle;
        }

        match self.ep0_state {
            Ep0State::Idle if csr0 & USB_CSR0_RX_PACKET_READY != 0 => self.handle_setup_packet(),
            Ep0State::Tx { .. } if csr0 & USB_CSR0_TX_PACKET_READY == 0 => {
                self.continue_control_in()
            }
            Ep0State::ReceiveLineCoding { received } if csr0 & USB_CSR0_RX_PACKET_READY != 0 => {
                self.receive_line_coding(received)
            }
            Ep0State::ApplyAddress(address) if csr0 & USB_CSR0_TX_PACKET_READY == 0 => {
                self.registers
                    .tx_csr
                    .write(USB_CSR0_SERVICE_RX_PACKET_READY | USB_CSR0_SERVICE_SETUP_END);
                self.registers.function_address.write(address);
                self.ep0_state = Ep0State::Idle;
            }
            _ => {}
        }
    }

    fn handle_setup_packet(&mut self) {
        // COUNT0 can settle a few cycles after the EP0 interrupt.  Both the
        // V821 BootROM and Tina UDC retry this exact read up to 16 times.
        let mut count = self.registers.rx_count.read() as usize;
        for _ in 0..16 {
            if count == 8 {
                break;
            }
            count = self.registers.rx_count.read() as usize;
        }
        if count != 8 {
            for _ in 0..count.min(EP0_MAX_PACKET) {
                let _ = self.registers.fifo[0].read_byte();
            }
            self.stall_endpoint_zero();
            return;
        }

        let mut bytes = [0u8; 8];
        for byte in &mut bytes {
            *byte = self.registers.fifo[0].read_byte();
        }
        let setup = SetupPacket::from_bytes(bytes);

        if setup.request_type & 0x60 == 0 {
            self.handle_standard_request(setup);
        } else if setup.request_type & 0x60 == 0x20 {
            self.handle_class_request(setup);
        } else {
            self.stall_endpoint_zero();
        }
    }

    fn handle_standard_request(&mut self, setup: SetupPacket) {
        match (setup.request_type, setup.request) {
            (0x80, 0x06) => {
                let source = match (setup.value >> 8) as u8 {
                    1 => Some(TxSource::Device),
                    2 => Some(TxSource::Configuration),
                    3 => match setup.value as u8 {
                        0 => Some(TxSource::Language),
                        1 => Some(TxSource::Manufacturer),
                        2 => Some(TxSource::Product),
                        3 => Some(TxSource::Serial),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(source) = source {
                    self.start_control_in(source, setup.length as usize);
                } else {
                    self.stall_endpoint_zero();
                }
            }
            (0x00, 0x05) if setup.index == 0 && setup.length == 0 => {
                // Tina and MUSB finish a zero-data OUT request with 0x48.
                // FEL's private 0x4a sequence leaves TXPKTRDY stuck here.
                self.registers
                    .tx_csr
                    .write(USB_CSR0_SERVICE_RX_PACKET_READY | USB_CSR0_DATA_END);
                self.ep0_state = Ep0State::ApplyAddress((setup.value & 0x7f) as u8);
            }
            (0x00, 0x09) if setup.length == 0 => {
                let value = setup.value as u8;
                if value <= 1 {
                    self.acknowledge_control_out();
                    self.configured = value == 1;
                    if self.configured {
                        self.configure_data_endpoints();
                    }
                } else {
                    self.stall_endpoint_zero();
                }
            }
            (0x80, 0x08) => {
                self.ep0_reply[0] = self.configured as u8;
                self.start_control_in(TxSource::Reply, setup.length.min(1) as usize);
            }
            (0x81, 0x0a) => {
                self.ep0_reply[0] = 0;
                self.start_control_in(TxSource::Reply, setup.length.min(1) as usize);
            }
            (0x01, 0x0b) if setup.length == 0 && setup.value == 0 => {
                self.acknowledge_control_out();
            }
            (request_type, 0x00) if request_type & 0x80 != 0 => {
                self.ep0_reply[0] = 0;
                self.ep0_reply[1] = 0;
                self.start_control_in(TxSource::Reply, setup.length.min(2) as usize);
            }
            (0x02, 0x01) if setup.value == 0 && setup.length == 0 => {
                self.clear_endpoint_halt(setup.index as u8);
                self.acknowledge_control_out();
            }
            _ => self.stall_endpoint_zero(),
        }
    }

    fn handle_class_request(&mut self, setup: SetupPacket) {
        match (setup.request_type, setup.request) {
            (0xa1, CDC_GET_LINE_CODING) if setup.length != 0 => {
                self.ep0_reply[..7].copy_from_slice(&self.line_coding);
                self.start_control_in(TxSource::Reply, setup.length.min(7) as usize);
            }
            (0x21, CDC_SET_LINE_CODING) if setup.length == 7 => {
                self.registers
                    .tx_csr
                    .write(USB_CSR0_SERVICE_RX_PACKET_READY);
                self.ep0_state = Ep0State::ReceiveLineCoding { received: 0 };
            }
            (0x21, CDC_SET_CONTROL_LINE_STATE) | (0x21, CDC_SEND_BREAK) if setup.length == 0 => {
                self.acknowledge_control_out();
            }
            _ => self.stall_endpoint_zero(),
        }
    }

    fn receive_line_coding(&mut self, received: usize) {
        let count = self.registers.rx_count.read() as usize;
        let accepted = count.min(7_usize.saturating_sub(received));
        for slot in &mut self.line_coding[received..received + accepted] {
            *slot = self.registers.fifo[0].read_byte();
        }
        for _ in accepted..count {
            let _ = self.registers.fifo[0].read_byte();
        }

        let next = received + accepted;
        if next == 7 || count < EP0_MAX_PACKET {
            self.registers
                .tx_csr
                .write(USB_CSR0_SERVICE_RX_PACKET_READY | USB_CSR0_DATA_END);
            self.ep0_state = Ep0State::Idle;
        } else {
            self.registers
                .tx_csr
                .write(USB_CSR0_SERVICE_RX_PACKET_READY);
            self.ep0_state = Ep0State::ReceiveLineCoding { received: next };
        }
    }

    fn start_control_in(&mut self, source: TxSource, requested: usize) {
        let available = self.source_len(source);
        let total = available.min(requested);
        // BootROM and Tina first retire the SETUP packet with CSR0=0x40,
        // then fill FIFO0 and prime TX separately.
        self.registers
            .tx_csr
            .write(USB_CSR0_SERVICE_RX_PACKET_READY);
        self.ep0_state = Ep0State::Tx {
            source,
            total,
            offset: 0,
            needs_zlp: total < requested && total.is_multiple_of(EP0_MAX_PACKET),
        };
        self.continue_control_in();
    }

    fn continue_control_in(&mut self) {
        let Ep0State::Tx {
            source,
            total,
            offset,
            needs_zlp,
        } = self.ep0_state
        else {
            return;
        };

        let count = (total - offset).min(EP0_MAX_PACKET);
        for index in 0..count {
            self.registers.fifo[0].write_byte(self.source_byte(source, offset + index));
        }
        let next = offset + count;
        let defer_zlp = next == total && needs_zlp && count != 0;

        let mut csr0 = USB_CSR0_TX_PACKET_READY;
        if next == total && !defer_zlp {
            csr0 |= USB_CSR0_DATA_END;
            self.ep0_state = Ep0State::Idle;
        } else {
            self.ep0_state = Ep0State::Tx {
                source,
                total,
                offset: next,
                needs_zlp: next < total && needs_zlp,
            };
        }
        self.registers.tx_csr.write(csr0);
    }

    fn source_len(&self, source: TxSource) -> usize {
        match source {
            TxSource::Device => DEVICE_DESCRIPTOR.len(),
            TxSource::Configuration => CONFIGURATION_DESCRIPTOR.len(),
            TxSource::Language => STRING_LANGUAGE.len(),
            TxSource::Manufacturer => STRING_MANUFACTURER.len(),
            TxSource::Product => STRING_PRODUCT.len(),
            TxSource::Serial => STRING_SERIAL.len(),
            TxSource::Reply => self.ep0_reply.len(),
        }
    }

    fn source_byte(&self, source: TxSource, index: usize) -> u8 {
        match source {
            TxSource::Device => DEVICE_DESCRIPTOR[index],
            TxSource::Configuration => CONFIGURATION_DESCRIPTOR[index],
            TxSource::Language => STRING_LANGUAGE[index],
            TxSource::Manufacturer => STRING_MANUFACTURER[index],
            TxSource::Product => STRING_PRODUCT[index],
            TxSource::Serial => STRING_SERIAL[index],
            TxSource::Reply => self.ep0_reply[index],
        }
    }

    fn acknowledge_control_out(&mut self) {
        self.select_endpoint(0);
        self.registers
            .tx_csr
            .write(USB_CSR0_SERVICE_RX_PACKET_READY | USB_CSR0_DATA_END);
        self.ep0_state = Ep0State::Idle;
    }

    fn stall_endpoint_zero(&mut self) {
        self.select_endpoint(0);
        self.registers
            .tx_csr
            .write(USB_CSR0_SERVICE_RX_PACKET_READY | USB_CSR0_SEND_STALL);
        self.ep0_state = Ep0State::Idle;
    }

    fn clear_endpoint_halt(&mut self, endpoint_address: u8) {
        let endpoint = match endpoint_address {
            0x81 => NOTIFY_IN_ENDPOINT,
            0x82 | 0x02 => DATA_IN_ENDPOINT,
            _ => return,
        };
        self.select_endpoint(endpoint);
        if endpoint_address & 0x80 != 0 {
            self.registers
                .tx_csr
                .write(USB_TXCSR_MODE | USB_TXCSR_CLEAR_DATA_TOGGLE | USB_TXCSR_FLUSH_FIFO);
        } else {
            self.registers
                .rx_csr
                .write(USB_RXCSR_CLEAR_DATA_TOGGLE | USB_RXCSR_FLUSH_FIFO);
        }
    }

    fn configure_data_endpoints(&mut self) {
        // EP1 IN notification: 16-byte packets backed by a 512-byte FIFO at
        // byte 0x200.  The endpoint normally NAKs because this tiny console
        // has no asynchronous serial-state notifications to report.
        self.select_endpoint(NOTIFY_IN_ENDPOINT);
        self.registers.tx_csr.write(0);
        self.registers.tx_max_packet.write(16);
        self.registers
            .tx_csr
            .write(USB_TXCSR_MODE | USB_TXCSR_CLEAR_DATA_TOGGLE | USB_TXCSR_FLUSH_FIFO);
        self.registers.tx_fifo_size.write(0x06);
        self.registers.tx_fifo_address.write(0x0040);

        // EP2 IN bulk: two 512-byte banks (1024 bytes total) at byte 0x600.
        self.select_endpoint(DATA_IN_ENDPOINT);
        self.registers.tx_csr.write(0);
        self.registers.tx_max_packet.write(64);
        self.registers
            .tx_csr
            .write(USB_TXCSR_MODE | USB_TXCSR_CLEAR_DATA_TOGGLE | USB_TXCSR_FLUSH_FIFO);
        self.registers.tx_fifo_size.write(0x16);
        self.registers.tx_fifo_address.write(0x00c0);

        // EP2 OUT bulk: two 512-byte banks (1024 bytes total) at byte 0xa00.
        self.registers.rx_csr.write(0);
        self.registers.rx_max_packet.write(64);
        self.registers
            .rx_csr
            .write(USB_RXCSR_CLEAR_DATA_TOGGLE | USB_RXCSR_FLUSH_FIFO);
        self.registers.rx_fifo_size.write(0x16);
        self.registers.rx_fifo_address.write(0x0140);

        self.registers
            .interrupt_tx_enable
            .write((1 << 0) | (1 << DATA_IN_ENDPOINT) | (1 << NOTIFY_IN_ENDPOINT));
        self.registers
            .interrupt_rx_enable
            .write(1 << DATA_OUT_ENDPOINT);
        self.select_endpoint(0);
    }

    #[inline(always)]
    fn select_endpoint(&self, endpoint: u8) {
        self.registers.index.write(endpoint);
    }

    fn acknowledge_all_pending_interrupts(&self) {
        let tx = self.registers.interrupt_tx.status();
        if tx != 0 {
            self.registers.interrupt_tx.acknowledge(tx);
        }
        let rx = self.registers.interrupt_rx.status();
        if rx != 0 {
            self.registers.interrupt_rx.acknowledge(rx);
        }
        let usb = self.registers.interrupt_usb.status();
        if usb != 0 {
            self.registers.interrupt_usb.acknowledge(usb);
        }
    }
}

fn delay_microseconds(microseconds: u32) {
    // The BootROM uses 40 ticks/us here. A slower counter only lengthens the
    // required reset/disconnect waits, which is harmless.
    let ticks = microseconds.saturating_mul(40);
    // SAFETY: COUNTER_LOW is the documented aligned, read-only low word of the
    // V821 free-running system counter.
    let start = unsafe { core::ptr::read_volatile(COUNTER_LOW as *const u32) };
    loop {
        // SAFETY: same read-only counter mapping as above.
        let now = unsafe { core::ptr::read_volatile(COUNTER_LOW as *const u32) };
        if now.wrapping_sub(start) >= ticks {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_offsets_match_the_v821_udc_layout() {
        assert_eq!(offset_of!(UsbRegisters, power), 0x40);
        assert_eq!(offset_of!(UsbRegisters, interrupt_usb), 0x4c);
        assert_eq!(offset_of!(UsbRegisters, interrupt_usb_enable), 0x50);
        assert_eq!(offset_of!(UsbRegisters, frame), 0x54);
        assert_eq!(offset_of!(UsbRegisters, tx_csr), 0x82);
        assert_eq!(offset_of!(UsbRegisters, rx_count), 0x88);
        assert_eq!(offset_of!(UsbRegisters, tx_fifo_size), 0x90);
        assert_eq!(offset_of!(UsbRegisters, rx_fifo_size), 0x94);
        assert_eq!(offset_of!(UsbRegisters, function_address), 0x98);
        assert_eq!(offset_of!(AppCcuRegisters, usb_24m), 0x7c);
        assert_eq!(offset_of!(AppCcuRegisters, gate0), 0x80);
        assert_eq!(offset_of!(AppCcuRegisters, reset0), 0x90);
        assert_eq!(offset_of!(HoscControlRegisters, selected_frequency), 0x04);
        assert_eq!(offset_of!(UsbPhyRegisters, clock_serial), 0x10);
        assert_eq!(offset_of!(UsbPhyRegisters, control), 0x20);
    }

    #[test]
    fn configuration_descriptor_is_well_formed() {
        assert_eq!(CONFIGURATION_DESCRIPTOR[0], 9);
        assert_eq!(CONFIGURATION_DESCRIPTOR[1], 2);
        assert_eq!(
            u16::from_le_bytes([CONFIGURATION_DESCRIPTOR[2], CONFIGURATION_DESCRIPTOR[3]]) as usize,
            CONFIGURATION_DESCRIPTOR.len()
        );
        assert_eq!(CONFIGURATION_DESCRIPTOR[4], 2);

        let mut offset = 0;
        let mut endpoints = [0u8; 3];
        let mut endpoint_count = 0;
        while offset < CONFIGURATION_DESCRIPTOR.len() {
            let length = CONFIGURATION_DESCRIPTOR[offset] as usize;
            assert!(length >= 2);
            assert!(offset + length <= CONFIGURATION_DESCRIPTOR.len());
            if CONFIGURATION_DESCRIPTOR[offset + 1] == 5 {
                endpoints[endpoint_count] = CONFIGURATION_DESCRIPTOR[offset + 2];
                endpoint_count += 1;
            }
            offset += length;
        }
        assert_eq!(offset, CONFIGURATION_DESCRIPTOR.len());
        assert_eq!(endpoints, [0x81, 0x02, 0x82]);
    }

    #[test]
    fn setup_packet_fields_are_little_endian() {
        let setup = SetupPacket::from_bytes([0x80, 6, 0, 1, 2, 0, 18, 0]);
        assert_eq!(setup.request_type, 0x80);
        assert_eq!(setup.request, 6);
        assert_eq!(setup.value, 0x0100);
        assert_eq!(setup.index, 2);
        assert_eq!(setup.length, 18);
    }
}
