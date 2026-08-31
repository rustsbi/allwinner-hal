//! Polled USB devices for the V821 USB0 controller.
//!
//! The controller layout and initialization sequence are taken from the
//! sun300iw1p1 BootROM and Tina RTOS UDC driver.  This module intentionally
//! keeps the raw-MMIO boundary local: there is one owner, interrupts stay
//! disabled, FIFO/status registers are not exposed as ordinary read/write
//! cells, and W1C registers are acknowledged with exact masks.

use allwinner_hal::{
    ccu::v821::{AonRegisterBlock, AppRegisterBlock},
    usb::{
        PhyRegisterBlock as UsbPhyRegisterBlock, RegisterBlock as UsbRegisterBlock,
        register::{
            BusInterruptEnable, EndpointIndex, EndpointZeroControlStatus, FifoAddress, FifoSize,
            FunctionAddress, MaximumPacketSize, ReceiveControlStatus, ReceiveInterruptEnable,
            ReceiveInterruptStatus, TransmitControlStatus, TransmitInterruptEnable,
            TransmitInterruptStatus,
        },
    },
};

const USB0_BASE: usize = 0x4410_0000;
const USB_PHY0_BASE: usize = 0x4410_0400;
const APP_CCU_BASE: usize = 0x4200_1000;
const AON_CCU_BASE: usize = 0x4a01_0000;
const COUNTER_LOW: usize = 0x3000_bff8;

const EP0_MAX_PACKET: usize = 64;
const PROFILE_CDC_ACM: u8 = 0;
const PROFILE_MASS_STORAGE: u8 = 1;
const PROFILE_NETWORK: u8 = 2;
const PROFILE_CDC_MSC: u8 = 3;
// A normal NCM function reset switches alt1 -> alt0 -> alt1 quickly. Only a
// sustained alt0/deconfiguration is treated as the user's safe removal.
const NETWORK_EXIT_DELAY_TICKS: u32 = 40 * 500_000;
const NOTIFY_IN_ENDPOINT: u8 = 1;
const DATA_OUT_ENDPOINT: u8 = 2;
const DATA_IN_ENDPOINT: u8 = 2;
const MSC_COMPOSITE_OUT_ENDPOINT: u8 = 3;
const MSC_COMPOSITE_IN_ENDPOINT: u8 = 3;
const FIFO_SIZE_SINGLE_512: FifoSize = FifoSize::single_512();
const FIFO_SIZE_DOUBLE_512: FifoSize = FifoSize::double_512();
const NOTIFY_TX_FIFO_ADDRESS: FifoAddress = FifoAddress::from_byte_offset(0x0200);
const MSC_COMPOSITE_TX_FIFO_ADDRESS: FifoAddress = FifoAddress::from_byte_offset(0x0400);
const DATA_TX_FIFO_ADDRESS: FifoAddress = FifoAddress::from_byte_offset(0x0600);
const DATA_RX_FIFO_ADDRESS: FifoAddress = FifoAddress::from_byte_offset(0x0a00);
const MSC_COMPOSITE_RX_FIFO_ADDRESS: FifoAddress = FifoAddress::from_byte_offset(0x0e00);

const CDC_SET_LINE_CODING: u8 = 0x20;
const CDC_GET_LINE_CODING: u8 = 0x21;
const CDC_SET_CONTROL_LINE_STATE: u8 = 0x22;
const CDC_SEND_BREAK: u8 = 0x23;
const MSC_BULK_ONLY_RESET: u8 = 0xff;
const MSC_GET_MAX_LUN: u8 = 0xfe;
const NCM_GET_NTB_PARAMETERS: u8 = 0x80;
const NCM_GET_NTB_INPUT_SIZE: u8 = 0x85;
const NCM_SET_NTB_INPUT_SIZE: u8 = 0x86;
const NCM_SET_ETHERNET_PACKET_FILTER: u8 = 0x43;

pub const NETWORK_DEVICE_MAC_ADDRESS: [u8; 6] = [0x02, 0xa0, 0xf1, 0x82, 0x10, 0x01];
// CDC-NCM's iMACAddress is assigned to the host adapter, not the USB peer.
pub const NETWORK_HOST_MAC_ADDRESS: [u8; 6] = [0x02, 0xa0, 0xf1, 0x82, 0x10, 0x02];

const CDC_DEVICE_DESCRIPTOR: [u8; 18] = [
    18, 0x01, 0x00, 0x02, 0xef, 0x02, 0x01, 64, 0x3a, 0x1f, 0x10, 0x82, 0x00, 0x01, 1, 2, 3, 1,
];

const MSC_DEVICE_DESCRIPTOR: [u8; 18] = [
    18, 0x01, 0x00, 0x02, 0, 0, 0, 64, 0x3a, 0x1f, 0x11, 0x82, 0x00, 0x01, 1, 2, 3, 1,
];

const NETWORK_DEVICE_DESCRIPTOR: [u8; 18] = [
    18, 0x01, 0x00, 0x02, 0xef, 0x02, 0x01, 64, 0x3a, 0x1f, 0x12, 0x82, 0x00, 0x01, 1, 2, 3, 1,
];

const CDC_MSC_DEVICE_DESCRIPTOR: [u8; 18] = [
    18, 0x01, 0x00, 0x02, 0xef, 0x02, 0x01, 64, 0x3a, 0x1f, 0x13, 0x82, 0x00, 0x01, 1, 2, 3, 1,
];

// Full-speed CDC-ACM configuration: IAD, control interface, three CDC
// functional descriptors, notification endpoint, and a two-endpoint data
// interface.
const CDC_CONFIGURATION_DESCRIPTOR: [u8; 75] = [
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

// Full-speed USB mass storage, SCSI transparent command set and Bulk-Only
// Transport, using EP2 in both directions.
const MSC_CONFIGURATION_DESCRIPTOR: [u8; 32] = [
    9, 0x02, 32, 0, 1, 1, 0, 0x80, 50, // configuration
    9, 0x04, 0, 0, 2, 0x08, 0x06, 0x50, 0, // mass-storage interface
    7, 0x05, 0x02, 0x02, 64, 0, 0, // bulk OUT
    7, 0x05, 0x82, 0x02, 64, 0, 0, // bulk IN
];

// Full-speed CDC-NCM: one control interface and an alternate data interface.
// The Ethernet descriptor advertises a 1514-byte frame and string 4 as MAC.
const NETWORK_CONFIGURATION_DESCRIPTOR: [u8; 94] = [
    9, 0x02, 94, 0, 2, 1, 0, 0x80, 50, // configuration
    8, 0x0b, 0, 2, 0x02, 0x0d, 0, 0, // interface association
    9, 0x04, 0, 0, 1, 0x02, 0x0d, 0, 0, // NCM control interface
    5, 0x24, 0x00, 0x10, 0x01, // CDC header, version 1.10
    5, 0x24, 0x06, 0, 1, // union: control 0, data 1
    13, 0x24, 0x0f, 4, 0, 0, 0, 0, 0xea, 0x05, 0, 0, 0, // Ethernet
    6, 0x24, 0x1a, 0x00, 0x01, 0x01, // NCM 1.0, packet filter
    7, 0x05, 0x81, 0x03, 16, 0, 16, // notification IN
    9, 0x04, 1, 0, 0, 0x0a, 0, 0x01, 0, // data interface, inactive
    9, 0x04, 1, 1, 2, 0x0a, 0, 0x01, 0, // data interface, active
    7, 0x05, 0x82, 0x02, 64, 0, 0, // bulk IN
    7, 0x05, 0x02, 0x02, 64, 0, 0, // bulk OUT
];

// CDC-NCM 1.0 NTB parameters: NTB16 only, 2048-byte transfers, one datagram.
const NCM_NTB_PARAMETERS: [u8; 28] = [
    28, 0, 1, 0, 0, 8, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 8, 0, 0, 4, 0, 0, 0, 4, 0, 1, 0,
];

// Full-speed CDC-ACM plus mass storage. Interfaces 0 and 1 form the CDC
// function through the IAD; interface 2 is an independent SCSI/BOT function.
const CDC_MSC_CONFIGURATION_DESCRIPTOR: [u8; 98] = [
    9, 0x02, 98, 0, 3, 1, 0, 0x80, 50, // configuration
    8, 0x0b, 0, 2, 0x02, 0x02, 0x01, 0, // CDC interface association
    9, 0x04, 0, 0, 1, 0x02, 0x02, 0x01, 0, // CDC communications interface
    5, 0x24, 0x00, 0x10, 0x01, // CDC header, version 1.10
    5, 0x24, 0x01, 0x00, 1, // call management, data interface 1
    4, 0x24, 0x02, 0x02, // ACM: line coding/control-line requests
    5, 0x24, 0x06, 0, 1, // union: control 0, subordinate 1
    7, 0x05, 0x81, 0x03, 16, 0, 16, // CDC notification IN
    9, 0x04, 1, 0, 2, 0x0a, 0, 0, 0, // CDC data interface
    7, 0x05, 0x02, 0x02, 64, 0, 0, // CDC bulk OUT
    7, 0x05, 0x82, 0x02, 64, 0, 0, // CDC bulk IN
    9, 0x04, 2, 0, 2, 0x08, 0x06, 0x50, 0, // mass-storage interface
    7, 0x05, 0x03, 0x02, 64, 0, 0, // MSC bulk OUT
    7, 0x05, 0x83, 0x02, 64, 0, 0, // MSC bulk IN
];

const STRING_LANGUAGE: [u8; 4] = [4, 0x03, 0x09, 0x04];
const STRING_MANUFACTURER: [u8; 16] = [
    16, 0x03, b'R', 0, b'u', 0, b's', 0, b't', 0, b'S', 0, b'B', 0, b'I', 0,
];
const STRING_PRODUCT_CDC: [u8; 28] = [
    28, 0x03, b'V', 0, b'8', 0, b'2', 0, b'1', 0, b' ', 0, b'U', 0, b'S', 0, b'B', 0, b' ', 0,
    b'U', 0, b'A', 0, b'R', 0, b'T', 0,
];
const STRING_PRODUCT_MSC: [u8; 20] = [
    20, 0x03, b'A', 0, b'v', 0, b'a', 0, b'o', 0, b't', 0, b'a', 0, b' ', 0, b'F', 0, b'1', 0,
];
const STRING_PRODUCT_NETWORK: [u8; 44] = [
    44, 0x03, b'A', 0, b'v', 0, b'a', 0, b'o', 0, b't', 0, b'a', 0, b' ', 0, b'F', 0, b'1', 0,
    b' ', 0, b'U', 0, b'S', 0, b'B', 0, b' ', 0, b'N', 0, b'e', 0, b't', 0, b'w', 0, b'o', 0, b'r',
    0, b'k', 0,
];
const STRING_PRODUCT_CDC_MSC: [u8; 40] = [
    40, 0x03, b'A', 0, b'v', 0, b'a', 0, b'o', 0, b't', 0, b'a', 0, b' ', 0, b'F', 0, b'1', 0,
    b' ', 0, b'C', 0, b'D', 0, b'C', 0, b' ', 0, b'+', 0, b' ', 0, b'M', 0, b'S', 0, b'C', 0,
];
const STRING_SERIAL_CDC: [u8; 18] = [
    18, 0x03, b'V', 0, b'8', 0, b'2', 0, b'1', 0, b'0', 0, b'0', 0, b'0', 0, b'1', 0,
];
const STRING_SERIAL_MSC: [u8; 26] = [
    26, 0x03, b'0', 0, b'8', 0, b'2', 0, b'1', 0, b'F', 0, b'1', 0, b'0', 0, b'0', 0, b'0', 0,
    b'0', 0, b'0', 0, b'1', 0,
];
const STRING_SERIAL_NETWORK: [u8; 26] = [
    26, 0x03, b'0', 0, b'8', 0, b'2', 0, b'1', 0, b'F', 0, b'1', 0, b'0', 0, b'0', 0, b'0', 0,
    b'0', 0, b'0', 0, b'2', 0,
];
const STRING_MAC_NETWORK: [u8; 26] = [
    26, 0x03, b'0', 0, b'2', 0, b'A', 0, b'0', 0, b'F', 0, b'1', 0, b'8', 0, b'2', 0, b'1', 0,
    b'0', 0, b'0', 0, b'2', 0,
];
const STRING_SERIAL_CDC_MSC: [u8; 26] = [
    26, 0x03, b'0', 0, b'8', 0, b'2', 0, b'1', 0, b'F', 0, b'1', 0, b'0', 0, b'0', 0, b'0', 0,
    b'0', 0, b'0', 0, b'3', 0,
];

const _: () =
    assert!(CDC_CONFIGURATION_DESCRIPTOR[2] as usize == CDC_CONFIGURATION_DESCRIPTOR.len());
const _: () =
    assert!(MSC_CONFIGURATION_DESCRIPTOR[2] as usize == MSC_CONFIGURATION_DESCRIPTOR.len());
const _: () =
    assert!(NETWORK_CONFIGURATION_DESCRIPTOR[2] as usize == NETWORK_CONFIGURATION_DESCRIPTOR.len());
const _: () =
    assert!(CDC_MSC_CONFIGURATION_DESCRIPTOR[2] as usize == CDC_MSC_CONFIGURATION_DESCRIPTOR.len());

#[derive(Clone, Copy)]
enum TxSource {
    Device,
    Configuration,
    Language,
    Manufacturer,
    Product,
    Serial,
    Mac,
    NtbParameters,
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
    ReceiveNtbInputSize,
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

/// CDC-ACM USB0 device used by the serial-console example.
pub type UsbCdcAcm = UsbDevice<PROFILE_CDC_ACM>;

/// Bulk-only USB0 transport used by the mass-storage example.
pub(crate) type UsbMassStorageTransport = UsbDevice<PROFILE_MASS_STORAGE>;

/// CDC-NCM USB0 transport used by the IPv6 network example.
pub(crate) type UsbNetworkTransport = UsbDevice<PROFILE_NETWORK>;

/// Shared USB0 transport used by the CDC-ACM plus mass-storage example.
pub(crate) type UsbCdcMscTransport = UsbDevice<PROFILE_CDC_MSC>;

/// Exclusive owner of USB0 while the Boot0 payload is running on the E907.
pub struct UsbDevice<const PROFILE: u8> {
    registers: &'static UsbRegisterBlock,
    phy_registers: &'static UsbPhyRegisterBlock,
    app_ccu: &'static AppRegisterBlock,
    aon_ccu: &'static AonRegisterBlock,
    ep0_state: Ep0State,
    ep0_reply: [u8; 8],
    line_coding: [u8; 7],
    ntb_input_size: [u8; 4],
    configured: bool,
    data_alt_setting: u8,
    class_reset: bool,
    bulk_in_wedged: bool,
    network_link_pending: bool,
    network_function_reset: bool,
    network_exit_pending: bool,
    network_exit_started: u32,
}

impl<const PROFILE: u8> UsbDevice<PROFILE> {
    /// Maps the V821 USB0 controller and PHY after the BootROM handoff.
    ///
    /// # Safety
    ///
    /// The caller must run on the V821 E907 after BootROM has transferred
    /// control from SPI Boot0 or FEL, with interrupts disabled and no other
    /// core or ISR accessing USB0, USB PHY0, or their APP-CCU fields. The
    /// addresses and layouts must match sun300iw1p1/V821 revision P1.
    pub unsafe fn from_v821_mmio() -> Self {
        // SAFETY: all four source-verified blocks are aligned and exclusively
        // owned under the caller's E907/interrupt preconditions.
        let registers = unsafe { &*(USB0_BASE as *const UsbRegisterBlock) };
        let phy_registers = unsafe { &*(USB_PHY0_BASE as *const UsbPhyRegisterBlock) };
        let app_ccu = unsafe { &*(APP_CCU_BASE as *const AppRegisterBlock) };
        let aon_ccu = unsafe { &*(AON_CCU_BASE as *const AonRegisterBlock) };

        Self {
            registers,
            phy_registers,
            app_ccu,
            aon_ccu,
            ep0_state: Ep0State::Idle,
            ep0_reply: [0; 8],
            // 115200 baud, one stop bit, no parity, eight data bits.
            line_coding: [0x00, 0xc2, 0x01, 0x00, 0, 0, 8],
            ntb_input_size: [0; 4],
            configured: false,
            data_alt_setting: 0,
            class_reset: false,
            bulk_in_wedged: false,
            network_link_pending: false,
            network_function_reset: false,
            network_exit_pending: false,
            network_exit_started: 0,
        }
    }

    /// Cold-initializes USB0, then reconnects it as a full-speed USB device.
    pub fn initialize(&mut self) {
        self.initialize_v821_usb0_hardware();

        // SAFETY: this UsbDevice exclusively owns USB0 for the payload's
        // lifetime, so these configuration writes cannot race another writer.
        unsafe {
            self.registers
                .power
                .write(self.registers.power.read().set_soft_connected(false));
        }
        delay_microseconds(250_000);

        // Select the same PIO bus mode used by BootROM FEL.
        // SAFETY: same exclusive USB0 ownership as above.
        unsafe {
            self.registers
                .vendor_control
                .write(self.registers.vendor_control.read().select_pio_bus());
            self.registers
                .interrupt_usb_enable
                .write(BusInterruptEnable::default());
            self.registers
                .interrupt_tx_enable
                .write(TransmitInterruptEnable::default());
            self.registers
                .interrupt_rx_enable
                .write(ReceiveInterruptEnable::default());
        }
        self.acknowledge_all_pending_interrupts();

        // SAFETY: same exclusive USB0 ownership as above.
        unsafe {
            self.registers
                .function_address
                .write(FunctionAddress::default());
        }
        self.ep0_state = Ep0State::Idle;
        self.configured = false;
        self.data_alt_setting = 0;
        self.class_reset = false;
        self.bulk_in_wedged = false;
        self.network_link_pending = false;
        self.network_function_reset = false;
        self.network_exit_pending = false;
        self.network_exit_started = 0;
        self.configure_data_endpoints();

        // Full speed keeps bulk endpoints at the declared 64-byte max-packet
        // size and avoids unverified high-speed PHY behavior.
        let power = self
            .registers
            .power
            .read()
            .set_high_speed_enabled(false)
            .set_iso_update_enabled(false)
            .set_soft_connected(false);
        let bus_interrupts = BusInterruptEnable::default()
            .enable_suspend()
            .enable_resume()
            .enable_reset();
        // SAFETY: same exclusive USB0 ownership as above.
        unsafe {
            self.registers.power.write(power);
            self.registers.interrupt_usb_enable.write(bus_interrupts);
        }

        delay_microseconds(1_000);
        // SAFETY: same exclusive USB0 ownership as above.
        unsafe {
            self.registers.power.write(power.set_soft_connected(true));
        }
    }

    fn initialize_v821_usb0_hardware(&self) {
        // This is BootROM 0x87be's reset/clock sequence. The E907 owns these
        // shared APP-CCU words exclusively here, so each volatile RMW cannot
        // race an ISR, another core, or another driver.
        // SAFETY: `from_v821_mmio` requires this E907 payload to be the sole
        // APP-CCU writer with interrupts disabled.
        unsafe {
            self.app_ccu
                .bus_reset0
                .modify(|value| value.assert_usb_phy());
            self.app_ccu
                .bus_clock_gating0
                .modify(|value| value.mask_usb_otg());
            self.app_ccu
                .bus_reset0
                .modify(|value| value.assert_usb_otg());
            self.app_ccu
                .bus_reset0
                .modify(|value| value.assert_usb_hclk());
        }
        delay_microseconds(20);
        // SAFETY: same exclusive APP-CCU ownership as above.
        unsafe {
            self.app_ccu
                .bus_clock_gating0
                .modify(|value| value.mask_usb_hclk());
        }
        delay_microseconds(20);

        // SAFETY: same exclusive APP-CCU ownership as above.
        unsafe {
            self.app_ccu
                .bus_reset0
                .modify(|value| value.deassert_usb_phy());
        }
        delay_microseconds(50);
        // SAFETY: same exclusive APP-CCU ownership as above.
        unsafe {
            self.app_ccu
                .bus_reset0
                .modify(|value| value.deassert_usb_otg());
        }
        delay_microseconds(100);
        // SAFETY: same exclusive APP-CCU ownership as above.
        unsafe {
            self.app_ccu
                .bus_clock_gating0
                .modify(|value| value.pass_usb_otg());
        }
        delay_microseconds(50);

        // SAFETY: same exclusive APP-CCU ownership as above.
        unsafe {
            self.app_ccu
                .bus_reset0
                .modify(|value| value.deassert_usb_hclk());
        }
        delay_microseconds(20);
        // SAFETY: same exclusive APP-CCU ownership as above.
        unsafe {
            self.app_ccu
                .bus_clock_gating0
                .modify(|value| value.pass_usb_hclk());
        }
        delay_microseconds(20);
        // SAFETY: same exclusive APP-CCU ownership as above.
        unsafe {
            self.app_ccu.usb_clock.modify(|value| value.enable());
        }

        // BootROM records its normalized oscillator choice in bit 31:
        // set means 24 MHz; clear means 40 MHz.
        let serial_byte = if self.aon_ccu.dcxo_status.read().is_24_mhz() {
            0x14_u8
        } else {
            0x0c_u8
        };
        for selector in 11_u8..19 {
            let data_high = serial_byte & (1 << (selector - 11)) != 0;
            let control = &self.phy_registers.phy_control_28nm;

            // Preserve every volatile transaction from BootROM 0x52e2..0x531e.
            // The four writes enable the VC bus, drive its clock low, present
            // the selector/data bit, then create the rising edge that latches it.
            // SAFETY: USB0 and its PHY are exclusively owned by this payload;
            // each typed value preserves the BootROM's four-write sequence.
            unsafe {
                control.write(control.read().enable_vc_bus());
                control.write(control.read().prepare_vc_write());
                control.write(control.read().set_vc_address_and_data(selector, data_high));
                control.write(control.read().raise_vc_clock());
            }
            delay_microseconds(50);
        }

        // BootROM 0x8768 selects USB0's OTG controller path before setup.
        // SAFETY: same exclusive USB0 and PHY ownership as above.
        unsafe {
            self.phy_registers
                .phy_select
                .write(self.phy_registers.phy_select.read().select_otg_controller());
            self.phy_registers
                .phy_control_28nm
                .write(self.phy_registers.phy_control_28nm.read().power_up());
        }
        delay_microseconds(20);

        self.phy_registers
            .interface_status_control
            .modify_control(|value| value.force_id_high());
        self.phy_registers
            .interface_status_control
            .modify_control(|value| {
                value
                    .set_dpdm_pullup_enabled(true)
                    .use_all_vbus_valid_sources()
            });
        if !self.registers.device_control.read().is_vbus_valid() {
            self.phy_registers
                .interface_status_control
                .modify_control(|value| value.force_vbus_valid_high());
        }
        self.phy_registers
            .interface_status_control
            .modify_control(|value| value.set_dpdm_pullup_enabled(false));
    }

    pub fn is_configured(&self) -> bool {
        self.configured
    }

    pub(crate) fn take_class_reset(&mut self) -> bool {
        core::mem::take(&mut self.class_reset)
    }

    pub(crate) fn take_network_link_pending(&mut self) -> bool {
        PROFILE == PROFILE_NETWORK && core::mem::take(&mut self.network_link_pending)
    }

    pub(crate) fn take_network_exit_requested(&mut self) -> bool {
        if PROFILE == PROFILE_NETWORK
            && self.network_exit_pending
            && counter_low().wrapping_sub(self.network_exit_started) >= NETWORK_EXIT_DELAY_TICKS
        {
            self.network_exit_pending = false;
            true
        } else {
            false
        }
    }

    pub(crate) fn take_network_function_reset(&mut self) -> bool {
        PROFILE == PROFILE_NETWORK && core::mem::take(&mut self.network_function_reset)
    }

    pub(crate) fn network_data_active(&self) -> bool {
        PROFILE == PROFILE_NETWORK && self.endpoint_transfers_active(Self::primary_data_endpoint())
    }

    #[inline]
    fn endpoint_transfers_active(&self, endpoint: u8) -> bool {
        self.configured
            && !(Self::is_mass_storage_endpoint(endpoint) && self.class_reset)
            && (PROFILE != PROFILE_NETWORK || self.data_alt_setting == 1)
    }

    const fn primary_data_endpoint() -> u8 {
        if PROFILE == PROFILE_CDC_MSC {
            MSC_COMPOSITE_IN_ENDPOINT
        } else {
            DATA_IN_ENDPOINT
        }
    }

    const fn mass_storage_endpoint() -> u8 {
        if PROFILE == PROFILE_CDC_MSC {
            MSC_COMPOSITE_IN_ENDPOINT
        } else {
            DATA_IN_ENDPOINT
        }
    }

    const fn is_mass_storage_endpoint(endpoint: u8) -> bool {
        (PROFILE == PROFILE_MASS_STORAGE && endpoint == DATA_IN_ENDPOINT)
            || (PROFILE == PROFILE_CDC_MSC && endpoint == MSC_COMPOSITE_IN_ENDPOINT)
    }

    const fn last_interface() -> u16 {
        if PROFILE == PROFILE_MASS_STORAGE {
            0
        } else if PROFILE == PROFILE_CDC_MSC {
            2
        } else {
            1
        }
    }

    pub(crate) fn stall_bulk_in(&mut self, until_class_reset: bool) {
        let endpoint = Self::mass_storage_endpoint();
        if !self.endpoint_transfers_active(endpoint) {
            return;
        }
        self.bulk_in_wedged = until_class_reset;
        self.select_endpoint(endpoint);
        self.registers
            .tx_csr
            .write_transmit(TransmitControlStatus::stall());
        self.select_endpoint(0);
    }

    /// Services bus/EP0 state and returns at most one received bulk packet.
    pub fn poll(&mut self, output: &mut [u8; 64]) -> usize {
        self.poll_endpoint_packet(Self::primary_data_endpoint(), output)
            .unwrap_or(0)
    }

    /// Services bus/EP0 state and distinguishes an OUT ZLP from no packet.
    pub(crate) fn poll_packet(&mut self, output: &mut [u8; 64]) -> Option<usize> {
        self.poll_endpoint_packet(Self::primary_data_endpoint(), output)
    }

    pub(crate) fn poll_cdc(&mut self, output: &mut [u8; 64]) -> usize {
        debug_assert!(PROFILE == PROFILE_CDC_MSC);
        self.poll_endpoint_packet(DATA_OUT_ENDPOINT, output)
            .unwrap_or(0)
    }

    fn poll_endpoint_packet(&mut self, endpoint: u8, output: &mut [u8; 64]) -> Option<usize> {
        if self.service_bus_and_control() {
            return None;
        }
        if !self.endpoint_transfers_active(endpoint) {
            return None;
        }

        self.select_endpoint(endpoint);
        if !self.registers.rx_csr.read().packet_ready() {
            return None;
        }

        let count = self.registers.rx_count.read().bytes().min(output.len());
        for byte in &mut output[..count] {
            *byte = self.registers.fifo[endpoint as usize].read_byte();
        }
        // RXCSR.RXPKTRDY is cleared by writing zero.  No persistent RXCSR
        // configuration bits are needed for this PIO bulk endpoint.
        self.registers.rx_csr.write(ReceiveControlStatus::clear());
        self.registers
            .interrupt_rx
            .acknowledge(ReceiveInterruptStatus::for_endpoint(endpoint));
        Some(count)
    }

    pub(crate) fn notify_network_link_up(&mut self) {
        const SPEED_CHANGE: [u8; 16] = [
            0xa1, 0x2a, 0, 0, 0, 0, 8, 0, 0, 0x1b, 0xb7, 0, 0, 0x1b, 0xb7, 0,
        ];
        const NETWORK_CONNECTION: [u8; 8] = [0xa1, 0, 1, 0, 0, 0, 0, 0];

        if PROFILE == PROFILE_NETWORK {
            self.write_notification(&SPEED_CHANGE);
            self.write_notification(&NETWORK_CONNECTION);
        }
    }

    fn write_notification(&mut self, bytes: &[u8]) {
        while self.configured && self.data_alt_setting == 1 {
            self.select_endpoint(NOTIFY_IN_ENDPOINT);
            if !self.registers.tx_csr.read_transmit().packet_ready() {
                for byte in bytes {
                    self.registers.fifo[NOTIFY_IN_ENDPOINT as usize].write_byte(*byte);
                }
                self.registers
                    .tx_csr
                    .write_transmit(TransmitControlStatus::queue_packet());
                return;
            }
            self.service_bus_and_control();
        }
    }

    /// Writes a byte stream through the bulk-IN endpoint.
    pub fn write(&mut self, bytes: &[u8]) {
        self.write_endpoint(Self::primary_data_endpoint(), bytes);
    }

    pub(crate) fn write_cdc(&mut self, bytes: &[u8]) {
        debug_assert!(PROFILE == PROFILE_CDC_MSC);
        self.write_endpoint(DATA_IN_ENDPOINT, bytes);
    }

    fn write_endpoint(&mut self, endpoint: u8, mut bytes: &[u8]) {
        while self.endpoint_transfers_active(endpoint) && !bytes.is_empty() {
            while self.endpoint_transfers_active(endpoint) {
                self.select_endpoint(endpoint);
                if self.registers.tx_csr.read_transmit().can_accept_packet() {
                    break;
                }
                self.service_bus_and_control();
            }
            if !self.endpoint_transfers_active(endpoint) {
                return;
            }

            let count = bytes.len().min(64);
            self.select_endpoint(endpoint);
            for byte in &bytes[..count] {
                self.registers.fifo[endpoint as usize].write_byte(*byte);
            }
            self.registers
                .tx_csr
                .write_transmit(TransmitControlStatus::queue_packet());
            bytes = &bytes[count..];
        }
    }

    pub(crate) fn write_zero_length_packet(&mut self) {
        self.write_zero_length_packet_to(Self::primary_data_endpoint());
    }

    fn write_zero_length_packet_to(&mut self, endpoint: u8) {
        while self.endpoint_transfers_active(endpoint) {
            self.select_endpoint(endpoint);
            if self.registers.tx_csr.read_transmit().can_accept_packet() {
                self.registers
                    .tx_csr
                    .write_transmit(TransmitControlStatus::queue_packet());
                return;
            }
            self.service_bus_and_control();
        }
    }

    /// Waits for the final bulk-IN acknowledgement, or reports a reset.
    pub fn flush(&mut self) -> bool {
        self.flush_endpoint(Self::primary_data_endpoint())
    }

    pub(crate) fn flush_cdc(&mut self) -> bool {
        debug_assert!(PROFILE == PROFILE_CDC_MSC);
        self.flush_endpoint(DATA_IN_ENDPOINT)
    }

    fn flush_endpoint(&mut self, endpoint: u8) -> bool {
        while self.endpoint_transfers_active(endpoint) {
            self.select_endpoint(endpoint);
            let pending = self.registers.tx_csr.read_transmit().packet_ready();
            self.service_bus_and_control();
            if !pending {
                return true;
            }
        }
        false
    }

    /// Returns true when a bus reset was handled and the caller should stop
    /// processing the current packet.
    fn service_bus_and_control(&mut self) -> bool {
        let usb_status = self.registers.interrupt_usb.status();
        if !usb_status.is_empty() {
            self.registers.interrupt_usb.acknowledge(usb_status);
        }
        if usb_status.reset_pending() {
            self.handle_bus_reset();
            return true;
        }

        let tx_status = self.registers.interrupt_tx.status();
        if tx_status.endpoint_pending(0) {
            self.registers
                .interrupt_tx
                .acknowledge(TransmitInterruptStatus::for_endpoint(0));
            self.handle_endpoint_zero();
        }
        let completed_nonzero = tx_status.without_endpoint(0);
        if !completed_nonzero.is_empty() {
            self.registers.interrupt_tx.acknowledge(completed_nonzero);
        }
        false
    }

    fn handle_bus_reset(&mut self) {
        // SAFETY: UsbDevice exclusively owns USB0 and resets its address.
        unsafe {
            self.registers
                .function_address
                .write(FunctionAddress::default());
        }
        self.ep0_state = Ep0State::Idle;
        self.configured = false;
        self.data_alt_setting = 0;
        self.class_reset = true;
        self.bulk_in_wedged = false;
        self.network_link_pending = false;
        self.network_function_reset = PROFILE == PROFILE_NETWORK;
        self.network_exit_pending = false;
        self.configure_data_endpoints();
        self.select_endpoint(0);
    }

    fn handle_endpoint_zero(&mut self) {
        self.select_endpoint(0);
        let csr0 = self.registers.tx_csr.read_endpoint_zero();

        if csr0.sent_stall() {
            self.registers
                .tx_csr
                .write_endpoint_zero(EndpointZeroControlStatus::clear());
            self.ep0_state = Ep0State::Idle;
            return;
        }
        if csr0.setup_end() {
            self.registers
                .tx_csr
                .write_endpoint_zero(EndpointZeroControlStatus::service_setup_end());
            self.ep0_state = Ep0State::Idle;
        }

        match self.ep0_state {
            Ep0State::Idle if csr0.received_packet_ready() => self.handle_setup_packet(),
            Ep0State::Tx { .. } if !csr0.transmit_packet_ready() => self.continue_control_in(),
            Ep0State::ReceiveLineCoding { received } if csr0.received_packet_ready() => {
                self.receive_line_coding(received)
            }
            Ep0State::ReceiveNtbInputSize if csr0.received_packet_ready() => {
                self.receive_ntb_input_size()
            }
            Ep0State::ApplyAddress(address) if !csr0.transmit_packet_ready() => {
                self.registers.tx_csr.write_endpoint_zero(
                    EndpointZeroControlStatus::service_received_packet_and_setup_end(),
                );
                // SAFETY: USB0 is exclusively owned and address is validated
                // from the host's seven-bit SET_ADDRESS value.
                unsafe {
                    self.registers
                        .function_address
                        .write(FunctionAddress::new(address));
                }
                self.ep0_state = Ep0State::Idle;
            }
            _ => {}
        }
    }

    fn handle_setup_packet(&mut self) {
        // COUNT0 can settle a few cycles after the EP0 interrupt.  Both the
        // V821 BootROM and Tina UDC retry this exact read up to 16 times.
        let mut count = self.registers.rx_count.read().bytes();
        for _ in 0..16 {
            if count == 8 {
                break;
            }
            count = self.registers.rx_count.read().bytes();
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
                        4 if PROFILE == PROFILE_NETWORK => Some(TxSource::Mac),
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
                self.registers.tx_csr.write_endpoint_zero(
                    EndpointZeroControlStatus::service_received_packet_and_complete(),
                );
                self.ep0_state = Ep0State::ApplyAddress((setup.value & 0x7f) as u8);
            }
            (0x00, 0x09) if setup.index == 0 && setup.length == 0 => {
                if setup.value <= 1 {
                    let value = setup.value as u8;
                    if PROFILE == PROFILE_NETWORK {
                        self.network_function_reset = true;
                        if self.configured && value == 0 {
                            self.network_exit_pending = true;
                            self.network_exit_started = counter_low();
                        } else if value == 1 {
                            self.network_exit_pending = false;
                        }
                    }
                    self.acknowledge_control_out();
                    self.configured = value == 1;
                    self.data_alt_setting = 0;
                    self.network_link_pending = false;
                    if self.configured {
                        self.configure_data_endpoints();
                    }
                } else {
                    self.stall_endpoint_zero();
                }
            }
            (0x80, 0x08) if setup.value == 0 && setup.index == 0 && setup.length != 0 => {
                self.ep0_reply[0] = self.configured as u8;
                self.start_control_in(TxSource::Reply, setup.length.min(1) as usize);
            }
            (0x81, 0x0a) if setup.value == 0 && setup.length != 0 => {
                if setup.index > Self::last_interface() {
                    self.stall_endpoint_zero();
                    return;
                }
                self.ep0_reply[0] = if PROFILE == PROFILE_NETWORK && setup.index == 1 {
                    self.data_alt_setting
                } else {
                    0
                };
                self.start_control_in(TxSource::Reply, setup.length.min(1) as usize);
            }
            (0x01, 0x0b) if setup.length == 0 => {
                let valid = if PROFILE == PROFILE_NETWORK && setup.index == 1 && setup.value <= 1 {
                    let alternate = setup.value as u8;
                    if self.data_alt_setting == 1 && alternate == 0 {
                        self.network_exit_pending = true;
                        self.network_exit_started = counter_low();
                    }
                    if alternate == 0 {
                        self.network_function_reset = true;
                    } else {
                        self.network_exit_pending = false;
                    }
                    if self.data_alt_setting != alternate {
                        self.data_alt_setting = alternate;
                        self.configure_data_endpoints();
                    }
                    self.network_link_pending = alternate == 1;
                    true
                } else {
                    setup.index <= Self::last_interface() && setup.value == 0
                };
                if valid {
                    self.acknowledge_control_out();
                } else {
                    self.stall_endpoint_zero();
                }
            }
            (0x80, 0x00) if setup.value == 0 && setup.index == 0 && setup.length == 2 => {
                self.ep0_reply[0] = 0;
                self.ep0_reply[1] = 0;
                self.start_control_in(TxSource::Reply, 2);
            }
            (0x81, 0x00)
                if setup.value == 0
                    && setup.index <= Self::last_interface()
                    && setup.length == 2 =>
            {
                self.ep0_reply[0] = 0;
                self.ep0_reply[1] = 0;
                self.start_control_in(TxSource::Reply, 2);
            }
            (0x82, 0x00) if setup.value == 0 && setup.length == 2 => {
                if let Some(halted) = self.endpoint_halt_status(setup.index) {
                    self.ep0_reply[0] = halted as u8;
                    self.ep0_reply[1] = 0;
                    self.start_control_in(TxSource::Reply, 2);
                } else {
                    self.stall_endpoint_zero();
                }
            }
            (0x02, 0x01) if setup.value == 0 && setup.length == 0 => {
                if setup.index <= u8::MAX as u16 && self.clear_endpoint_halt(setup.index as u8) {
                    self.acknowledge_control_out();
                } else {
                    self.stall_endpoint_zero();
                }
            }
            _ => self.stall_endpoint_zero(),
        }
    }

    fn handle_class_request(&mut self, setup: SetupPacket) {
        let mass_storage_interface = if PROFILE == PROFILE_CDC_MSC { 2 } else { 0 };
        if PROFILE == PROFILE_MASS_STORAGE
            || (PROFILE == PROFILE_CDC_MSC && setup.index == mass_storage_interface)
        {
            match (setup.request_type, setup.request) {
                (0xa1, MSC_GET_MAX_LUN)
                    if setup.value == 0
                        && setup.index == mass_storage_interface
                        && setup.length == 1 =>
                {
                    self.ep0_reply[0] = 0;
                    self.start_control_in(TxSource::Reply, 1);
                }
                (0x21, MSC_BULK_ONLY_RESET)
                    if setup.value == 0
                        && setup.index == mass_storage_interface
                        && setup.length == 0 =>
                {
                    self.class_reset = true;
                    // BOT reset preserves bulk stalls and data toggles.  It
                    // only releases a wedged IN endpoint so the host's
                    // following CLEAR_FEATURE requests can recover both ends.
                    self.bulk_in_wedged = false;
                    self.acknowledge_control_out();
                }
                _ => self.stall_endpoint_zero(),
            }
            return;
        }

        if PROFILE == PROFILE_NETWORK {
            match (setup.request_type, setup.request) {
                (0xa1, NCM_GET_NTB_PARAMETERS)
                    if setup.value == 0 && setup.index == 0 && setup.length != 0 =>
                {
                    self.start_control_in(
                        TxSource::NtbParameters,
                        setup.length.min(NCM_NTB_PARAMETERS.len() as u16) as usize,
                    );
                }
                (0xa1, NCM_GET_NTB_INPUT_SIZE)
                    if setup.value == 0 && setup.index == 0 && setup.length >= 4 =>
                {
                    self.ep0_reply[..4].copy_from_slice(&2048_u32.to_le_bytes());
                    self.start_control_in(TxSource::Reply, 4);
                }
                (0x21, NCM_SET_NTB_INPUT_SIZE)
                    if setup.value == 0 && setup.index == 0 && setup.length == 4 =>
                {
                    self.ntb_input_size.fill(0);
                    self.registers
                        .tx_csr
                        .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet());
                    self.ep0_state = Ep0State::ReceiveNtbInputSize;
                }
                (0x21, NCM_SET_ETHERNET_PACKET_FILTER) if setup.index == 0 && setup.length == 0 => {
                    self.acknowledge_control_out();
                }
                _ => self.stall_endpoint_zero(),
            }
            return;
        }

        match (setup.request_type, setup.request) {
            (0xa1, CDC_GET_LINE_CODING) if setup.index == 0 && setup.length != 0 => {
                self.ep0_reply[..7].copy_from_slice(&self.line_coding);
                self.start_control_in(TxSource::Reply, setup.length.min(7) as usize);
            }
            (0x21, CDC_SET_LINE_CODING) if setup.index == 0 && setup.length == 7 => {
                self.registers
                    .tx_csr
                    .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet());
                self.ep0_state = Ep0State::ReceiveLineCoding { received: 0 };
            }
            (0x21, CDC_SET_CONTROL_LINE_STATE) | (0x21, CDC_SEND_BREAK)
                if setup.index == 0 && setup.length == 0 =>
            {
                self.acknowledge_control_out();
            }
            _ => self.stall_endpoint_zero(),
        }
    }

    fn receive_line_coding(&mut self, received: usize) {
        let count = self.registers.rx_count.read().bytes();
        let accepted = count.min(7_usize.saturating_sub(received));
        for slot in &mut self.line_coding[received..received + accepted] {
            *slot = self.registers.fifo[0].read_byte();
        }
        for _ in accepted..count {
            let _ = self.registers.fifo[0].read_byte();
        }

        let next = received + accepted;
        if next == 7 || count < EP0_MAX_PACKET {
            self.registers.tx_csr.write_endpoint_zero(
                EndpointZeroControlStatus::service_received_packet_and_complete(),
            );
            self.ep0_state = Ep0State::Idle;
        } else {
            self.registers
                .tx_csr
                .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet());
            self.ep0_state = Ep0State::ReceiveLineCoding { received: next };
        }
    }

    fn receive_ntb_input_size(&mut self) {
        let count = self.registers.rx_count.read().bytes();
        let accepted = count.min(self.ntb_input_size.len());
        for slot in &mut self.ntb_input_size[..accepted] {
            *slot = self.registers.fifo[0].read_byte();
        }
        for _ in accepted..count {
            let _ = self.registers.fifo[0].read_byte();
        }

        if count == self.ntb_input_size.len() && u32::from_le_bytes(self.ntb_input_size) == 2048 {
            self.registers.tx_csr.write_endpoint_zero(
                EndpointZeroControlStatus::service_received_packet_and_complete(),
            );
            self.ep0_state = Ep0State::Idle;
        } else {
            self.stall_endpoint_zero();
        }
    }

    fn start_control_in(&mut self, source: TxSource, requested: usize) {
        let available = self.source_len(source);
        let total = available.min(requested);
        // BootROM and Tina first retire the SETUP packet with CSR0=0x40,
        // then fill FIFO0 and prime TX separately.
        self.registers
            .tx_csr
            .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet());
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

        let transfer_complete = next == total && !defer_zlp;
        if transfer_complete {
            self.ep0_state = Ep0State::Idle;
        } else {
            self.ep0_state = Ep0State::Tx {
                source,
                total,
                offset: next,
                needs_zlp: next < total && needs_zlp,
            };
        }
        self.registers.tx_csr.write_endpoint_zero(
            EndpointZeroControlStatus::queue_transmit_packet(transfer_complete),
        );
    }

    fn source_len(&self, source: TxSource) -> usize {
        self.source(source).len()
    }

    fn source_byte(&self, source: TxSource, index: usize) -> u8 {
        self.source(source)[index]
    }

    fn source(&self, source: TxSource) -> &[u8] {
        match source {
            TxSource::Device if PROFILE == PROFILE_NETWORK => &NETWORK_DEVICE_DESCRIPTOR,
            TxSource::Device if PROFILE == PROFILE_CDC_MSC => &CDC_MSC_DEVICE_DESCRIPTOR,
            TxSource::Device if PROFILE == PROFILE_MASS_STORAGE => &MSC_DEVICE_DESCRIPTOR,
            TxSource::Device => &CDC_DEVICE_DESCRIPTOR,
            TxSource::Configuration if PROFILE == PROFILE_NETWORK => {
                &NETWORK_CONFIGURATION_DESCRIPTOR
            }
            TxSource::Configuration if PROFILE == PROFILE_CDC_MSC => {
                &CDC_MSC_CONFIGURATION_DESCRIPTOR
            }
            TxSource::Configuration if PROFILE == PROFILE_MASS_STORAGE => {
                &MSC_CONFIGURATION_DESCRIPTOR
            }
            TxSource::Configuration => &CDC_CONFIGURATION_DESCRIPTOR,
            TxSource::Language => &STRING_LANGUAGE,
            TxSource::Manufacturer => &STRING_MANUFACTURER,
            TxSource::Product if PROFILE == PROFILE_NETWORK => &STRING_PRODUCT_NETWORK,
            TxSource::Product if PROFILE == PROFILE_CDC_MSC => &STRING_PRODUCT_CDC_MSC,
            TxSource::Product if PROFILE == PROFILE_MASS_STORAGE => &STRING_PRODUCT_MSC,
            TxSource::Product => &STRING_PRODUCT_CDC,
            TxSource::Serial if PROFILE == PROFILE_NETWORK => &STRING_SERIAL_NETWORK,
            TxSource::Serial if PROFILE == PROFILE_CDC_MSC => &STRING_SERIAL_CDC_MSC,
            TxSource::Serial if PROFILE == PROFILE_MASS_STORAGE => &STRING_SERIAL_MSC,
            TxSource::Serial => &STRING_SERIAL_CDC,
            TxSource::Mac => &STRING_MAC_NETWORK,
            TxSource::NtbParameters => &NCM_NTB_PARAMETERS,
            TxSource::Reply => &self.ep0_reply,
        }
    }

    fn acknowledge_control_out(&mut self) {
        self.select_endpoint(0);
        self.registers
            .tx_csr
            .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet_and_complete());
        self.ep0_state = Ep0State::Idle;
    }

    fn stall_endpoint_zero(&mut self) {
        self.select_endpoint(0);
        self.registers
            .tx_csr
            .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet_and_stall());
        self.ep0_state = Ep0State::Idle;
    }

    fn endpoint_halt_status(&mut self, endpoint_index: u16) -> Option<bool> {
        let endpoint_address = u8::try_from(endpoint_index).ok()?;
        if !Self::is_valid_endpoint_address(endpoint_address) {
            return None;
        }
        if endpoint_address == 0 || endpoint_address == 0x80 {
            return Some(false);
        }
        // This driver never actively stalls OUT endpoints. Reading RXCSR here
        // would add no information and could couple status handling to
        // undocumented receive-stall semantics.
        if endpoint_address & 0x80 == 0 {
            return Some(false);
        }

        let endpoint = endpoint_address & 0x0f;
        let wedged = endpoint_address == Self::mass_storage_in_address() && self.bulk_in_wedged;
        self.select_endpoint(endpoint);
        let csr = self.registers.tx_csr.read_transmit();
        self.select_endpoint(0);
        Some(wedged || csr.is_stalled())
    }

    const fn is_valid_endpoint_address(endpoint_address: u8) -> bool {
        match endpoint_address {
            0x00 | 0x80 | 0x02 | 0x82 => true,
            0x81 => PROFILE != PROFILE_MASS_STORAGE,
            0x03 | 0x83 => PROFILE == PROFILE_CDC_MSC,
            _ => false,
        }
    }

    const fn mass_storage_in_address() -> u8 {
        if PROFILE == PROFILE_CDC_MSC {
            0x83
        } else if PROFILE == PROFILE_MASS_STORAGE {
            0x82
        } else {
            0
        }
    }

    fn clear_endpoint_halt(&mut self, endpoint_address: u8) -> bool {
        let endpoint = match endpoint_address {
            0x81 if PROFILE != PROFILE_MASS_STORAGE => NOTIFY_IN_ENDPOINT,
            0x82 | 0x02 => DATA_IN_ENDPOINT,
            0x83 | 0x03 if PROFILE == PROFILE_CDC_MSC => MSC_COMPOSITE_IN_ENDPOINT,
            _ => return false,
        };
        if endpoint_address == Self::mass_storage_in_address() && self.bulk_in_wedged {
            return true;
        }
        self.select_endpoint(endpoint);
        if endpoint_address & 0x80 != 0 {
            self.flush_tx_fifo(endpoint == DATA_IN_ENDPOINT);
        } else {
            self.flush_rx_fifo(endpoint == DATA_OUT_ENDPOINT);
        }
        true
    }

    fn flush_tx_fifo(&self, double_buffered: bool) {
        let command = TransmitControlStatus::flush_and_clear_data_toggle();
        self.registers.tx_csr.write_transmit(command);
        if double_buffered {
            self.registers.tx_csr.write_transmit(command);
        }
    }

    fn flush_rx_fifo(&self, double_buffered: bool) {
        let command = ReceiveControlStatus::flush_and_clear_data_toggle();
        self.registers.rx_csr.write(command);
        if double_buffered {
            self.registers.rx_csr.write(command);
        }
    }

    fn configure_data_endpoints(&mut self) {
        self.bulk_in_wedged = false;
        if PROFILE != PROFILE_MASS_STORAGE {
            // EP1 IN notification: 16-byte packets backed by a 512-byte FIFO
            // at byte 0x200. ACM leaves it idle; NCM uses it for link and speed
            // notifications after alternate setting 1 becomes active.
            self.select_endpoint(NOTIFY_IN_ENDPOINT);
            self.registers
                .tx_csr
                .write_transmit(TransmitControlStatus::clear());
            // SAFETY: this driver owns the USB controller and writes a valid
            // packet size to the currently selected endpoint register.
            unsafe {
                self.registers
                    .tx_max_packet
                    .write(MaximumPacketSize::new(16));
            }
            self.flush_tx_fifo(false);
            // SAFETY: the typed values describe an aligned, in-range FIFO
            // allocation for the currently selected endpoint.
            unsafe {
                self.registers.tx_fifo_size.write(FIFO_SIZE_SINGLE_512);
                self.registers.tx_fifo_address.write(NOTIFY_TX_FIFO_ADDRESS);
            }
        }

        // EP2 IN bulk: two 512-byte banks (1024 bytes total) at byte 0x600.
        self.select_endpoint(DATA_IN_ENDPOINT);
        self.registers
            .tx_csr
            .write_transmit(TransmitControlStatus::clear());
        // SAFETY: this driver owns the USB controller and writes a valid
        // packet size to the currently selected endpoint register.
        unsafe {
            self.registers
                .tx_max_packet
                .write(MaximumPacketSize::new(64));
        }
        self.flush_tx_fifo(true);
        // SAFETY: the typed values describe an aligned, in-range FIFO
        // allocation for the currently selected endpoint.
        unsafe {
            self.registers.tx_fifo_size.write(FIFO_SIZE_DOUBLE_512);
            self.registers.tx_fifo_address.write(DATA_TX_FIFO_ADDRESS);
        }

        // EP2 OUT bulk: two 512-byte banks (1024 bytes total) at byte 0xa00.
        self.registers.rx_csr.write(ReceiveControlStatus::clear());
        // SAFETY: this driver owns the USB controller and writes a valid
        // packet size to the currently selected endpoint register.
        unsafe {
            self.registers
                .rx_max_packet
                .write(MaximumPacketSize::new(64));
        }
        self.flush_rx_fifo(true);
        // SAFETY: the typed values describe an aligned, in-range FIFO
        // allocation for the currently selected endpoint.
        unsafe {
            self.registers.rx_fifo_size.write(FIFO_SIZE_DOUBLE_512);
            self.registers.rx_fifo_address.write(DATA_RX_FIFO_ADDRESS);
        }

        if PROFILE == PROFILE_CDC_MSC {
            // EP3 IN uses the 512-byte gap at byte 0x400, between EP1 and
            // EP2's banks. EP3 OUT uses the final bank at byte 0xe00.
            self.select_endpoint(MSC_COMPOSITE_IN_ENDPOINT);
            self.registers
                .tx_csr
                .write_transmit(TransmitControlStatus::clear());
            // SAFETY: this driver owns the USB controller and writes a valid
            // packet size to the currently selected endpoint register.
            unsafe {
                self.registers
                    .tx_max_packet
                    .write(MaximumPacketSize::new(64));
            }
            self.flush_tx_fifo(false);
            // SAFETY: the typed values describe an aligned, in-range FIFO
            // allocation for the currently selected endpoint.
            unsafe {
                self.registers.tx_fifo_size.write(FIFO_SIZE_SINGLE_512);
                self.registers
                    .tx_fifo_address
                    .write(MSC_COMPOSITE_TX_FIFO_ADDRESS);
            }

            self.registers.rx_csr.write(ReceiveControlStatus::clear());
            // SAFETY: this driver owns the USB controller and writes a valid
            // packet size to the currently selected endpoint register.
            unsafe {
                self.registers
                    .rx_max_packet
                    .write(MaximumPacketSize::new(64));
            }
            self.flush_rx_fifo(false);
            // SAFETY: the typed values describe an aligned, in-range FIFO
            // allocation for the currently selected endpoint.
            unsafe {
                self.registers.rx_fifo_size.write(FIFO_SIZE_SINGLE_512);
                self.registers
                    .rx_fifo_address
                    .write(MSC_COMPOSITE_RX_FIFO_ADDRESS);
            }
        }

        let mut transmit_interrupts = TransmitInterruptEnable::default()
            .enable_endpoint(0)
            .enable_endpoint(DATA_IN_ENDPOINT);
        if PROFILE != PROFILE_MASS_STORAGE {
            transmit_interrupts = transmit_interrupts.enable_endpoint(NOTIFY_IN_ENDPOINT);
        }
        if PROFILE == PROFILE_CDC_MSC {
            transmit_interrupts = transmit_interrupts.enable_endpoint(MSC_COMPOSITE_IN_ENDPOINT);
        }
        let mut receive_interrupts =
            ReceiveInterruptEnable::default().enable_endpoint(DATA_OUT_ENDPOINT);
        if PROFILE == PROFILE_CDC_MSC {
            receive_interrupts = receive_interrupts.enable_endpoint(MSC_COMPOSITE_OUT_ENDPOINT);
        }
        // SAFETY: this driver exclusively owns the controller interrupt masks;
        // the typed masks enable only endpoints configured above.
        unsafe {
            self.registers
                .interrupt_tx_enable
                .write(transmit_interrupts);
            self.registers.interrupt_rx_enable.write(receive_interrupts);
        }
        self.select_endpoint(0);
    }

    #[inline(always)]
    fn select_endpoint(&self, endpoint: u8) {
        // SAFETY: this driver owns INDEX, and EndpointIndex validates the
        // endpoint number before the volatile write.
        unsafe { self.registers.index.write(EndpointIndex::new(endpoint)) }
    }

    fn acknowledge_all_pending_interrupts(&self) {
        let tx = self.registers.interrupt_tx.status();
        if !tx.is_empty() {
            self.registers.interrupt_tx.acknowledge(tx);
        }
        let rx = self.registers.interrupt_rx.status();
        if !rx.is_empty() {
            self.registers.interrupt_rx.acknowledge(rx);
        }
        let usb = self.registers.interrupt_usb.status();
        if !usb.is_empty() {
            self.registers.interrupt_usb.acknowledge(usb);
        }
    }
}

fn delay_microseconds(microseconds: u32) {
    // The BootROM uses 40 ticks/us here. A slower counter only lengthens the
    // required reset/disconnect waits, which is harmless.
    let ticks = microseconds.saturating_mul(40);
    let start = counter_low();
    loop {
        let now = counter_low();
        if now.wrapping_sub(start) >= ticks {
            break;
        }
    }
}

fn counter_low() -> u32 {
    // SAFETY: COUNTER_LOW is the documented aligned, read-only low word of the
    // V821 free-running system counter.
    unsafe { core::ptr::read_volatile(COUNTER_LOW as *const u32) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cdc_configuration_descriptor_is_well_formed() {
        assert_configuration_descriptor(&CDC_CONFIGURATION_DESCRIPTOR, 2, &[0x81, 0x02, 0x82]);
    }

    #[test]
    fn mass_storage_configuration_descriptor_is_well_formed() {
        assert_configuration_descriptor(&MSC_CONFIGURATION_DESCRIPTOR, 1, &[0x02, 0x82]);
    }

    #[test]
    fn network_configuration_descriptor_is_well_formed() {
        assert_configuration_descriptor(&NETWORK_CONFIGURATION_DESCRIPTOR, 2, &[0x81, 0x82, 0x02]);
        assert_eq!(&NETWORK_CONFIGURATION_DESCRIPTOR[44..46], &[0xea, 0x05]);
        assert_ne!(NETWORK_DEVICE_MAC_ADDRESS, NETWORK_HOST_MAC_ADDRESS);
        for (index, byte) in b"02A0F1821002".iter().enumerate() {
            assert_eq!(STRING_MAC_NETWORK[2 + index * 2], *byte);
            assert_eq!(STRING_MAC_NETWORK[3 + index * 2], 0);
        }
        assert_eq!(
            u16::from_le_bytes([NCM_NTB_PARAMETERS[0], NCM_NTB_PARAMETERS[1]]),
            28
        );
        assert_eq!(
            u32::from_le_bytes(NCM_NTB_PARAMETERS[4..8].try_into().unwrap()),
            2048
        );
    }

    #[test]
    fn cdc_msc_configuration_descriptor_is_well_formed() {
        assert_configuration_descriptor(
            &CDC_MSC_CONFIGURATION_DESCRIPTOR,
            3,
            &[0x81, 0x02, 0x82, 0x03, 0x83],
        );
        assert_eq!(&CDC_MSC_CONFIGURATION_DESCRIPTOR[2..4], &[98, 0]);
        assert_eq!(
            &CDC_MSC_CONFIGURATION_DESCRIPTOR[84..91],
            &[7, 5, 3, 2, 64, 0, 0]
        );
        assert_eq!(
            &CDC_MSC_CONFIGURATION_DESCRIPTOR[91..98],
            &[7, 5, 0x83, 2, 64, 0, 0]
        );
    }

    #[test]
    fn cdc_msc_fifo_banks_fit_without_overlap() {
        let banks = [
            fifo_range(NOTIFY_TX_FIFO_ADDRESS, FIFO_SIZE_SINGLE_512),
            fifo_range(MSC_COMPOSITE_TX_FIFO_ADDRESS, FIFO_SIZE_SINGLE_512),
            fifo_range(DATA_TX_FIFO_ADDRESS, FIFO_SIZE_DOUBLE_512),
            fifo_range(DATA_RX_FIFO_ADDRESS, FIFO_SIZE_DOUBLE_512),
            fifo_range(MSC_COMPOSITE_RX_FIFO_ADDRESS, FIFO_SIZE_SINGLE_512),
        ];

        for (index, bank) in banks.iter().enumerate() {
            assert!(bank.end <= 0x1000);
            for other in &banks[index + 1..] {
                assert!(bank.end <= other.start || other.end <= bank.start);
            }
        }
    }

    #[test]
    fn get_status_endpoint_routing_matches_each_profile() {
        assert!(UsbDevice::<PROFILE_CDC_ACM>::is_valid_endpoint_address(
            0x81
        ));
        assert!(!UsbDevice::<PROFILE_MASS_STORAGE>::is_valid_endpoint_address(0x81));
        assert!(!UsbDevice::<PROFILE_NETWORK>::is_valid_endpoint_address(
            0x83
        ));
        assert!(UsbDevice::<PROFILE_CDC_MSC>::is_valid_endpoint_address(
            0x83
        ));
        assert!(UsbDevice::<PROFILE_CDC_MSC>::is_valid_endpoint_address(
            0x03
        ));
        assert!(!UsbDevice::<PROFILE_CDC_MSC>::is_valid_endpoint_address(
            0x84
        ));
        assert_eq!(
            UsbDevice::<PROFILE_CDC_MSC>::mass_storage_in_address(),
            0x83
        );
    }

    fn fifo_range(address: FifoAddress, size: FifoSize) -> core::ops::Range<usize> {
        let start = address.byte_offset();
        start..start + size.total_bytes()
    }

    fn assert_configuration_descriptor(
        descriptor: &[u8],
        interfaces: u8,
        expected_endpoints: &[u8],
    ) {
        assert_eq!(descriptor[0], 9);
        assert_eq!(descriptor[1], 2);
        assert_eq!(
            u16::from_le_bytes([descriptor[2], descriptor[3]]) as usize,
            descriptor.len()
        );
        assert_eq!(descriptor[4], interfaces);

        let mut offset = 0;
        let mut endpoints = [0u8; 5];
        let mut endpoint_count = 0;
        while offset < descriptor.len() {
            let length = descriptor[offset] as usize;
            assert!(length >= 2);
            assert!(offset + length <= descriptor.len());
            if descriptor[offset + 1] == 5 {
                endpoints[endpoint_count] = descriptor[offset + 2];
                endpoint_count += 1;
            }
            offset += length;
        }
        assert_eq!(offset, descriptor.len());
        assert_eq!(&endpoints[..endpoint_count], expected_endpoints);
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
