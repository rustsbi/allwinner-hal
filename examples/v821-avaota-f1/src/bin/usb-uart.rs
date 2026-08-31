#![no_std]
#![no_main]

use allwinner_hal::usb::{Usb, UsbBus as AllwinnerUsbBus, phy::v821::UsbPhy};
use allwinner_rt::{Clocks, Peripherals, entry};
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use usb_device::{
    UsbError,
    bus::{UsbBus, UsbBusAllocator},
    device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use v821_avaota_f1::console::{Command, Console, InputEvent};

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
    let mut serial = SerialPort::new(&usb_bus);
    let strings = [StringDescriptors::default()
        .manufacturer("RustSBI")
        .product("V821 USB UART")
        .serial_number("V821-AVAOTA-F1")];
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1f3a, 0x8210))
        .strings(&strings)
        .unwrap()
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64)
        .unwrap()
        .build();

    let mut console = Console::<32>::new();
    let mut received = [0u8; 64];
    let mut greeting_visible = false;

    loop {
        let active = usb_device.poll(&mut [&mut serial]);
        if usb_device.state() != UsbDeviceState::Configured {
            greeting_visible = false;
            continue;
        }
        if !greeting_visible {
            if !write_all(
                &mut usb_device,
                &mut serial,
                b"Welcome to Allwinner-HAL v821-avaota-f1 example!\r\n> ",
            ) {
                continue;
            }
            greeting_visible = true;
        }
        if !active {
            continue;
        }

        let count = match serial.read(&mut received) {
            Ok(count) => count,
            Err(UsbError::WouldBlock) => 0,
            Err(_) => continue,
        };
        for &byte in &received[..count] {
            match console.push(byte) {
                InputEvent::None => {}
                InputEvent::Echo(byte) => {
                    if !write_all(&mut usb_device, &mut serial, &[byte]) {
                        break;
                    }
                }
                InputEvent::Erase => {
                    if !write_all(&mut usb_device, &mut serial, b"\x08 \x08") {
                        break;
                    }
                }
                InputEvent::Bell => {
                    if !write_all(&mut usb_device, &mut serial, b"\x07") {
                        break;
                    }
                }
                InputEvent::Command(command) => {
                    if !write_all(&mut usb_device, &mut serial, b"\r\n") {
                        break;
                    }
                    let response = match command {
                        Command::Empty => b"" as &[u8],
                        Command::Hello => b"hello world\r\n",
                        Command::Help => {
                            b"Commands:\r\n  help   show this help\r\n  hello  print hello world\r\n  exit   return to FEL\r\n"
                        }
                        Command::Exit => {
                            let _ = write_all(&mut usb_device, &mut serial, b"Bye!\r\n");
                            flush(&mut usb_device, &mut serial);
                            delay.delay_ms(10);
                            return;
                        }
                        Command::Unknown => b"unknown command; try help\r\n",
                    };
                    if !write_all(&mut usb_device, &mut serial, response)
                        || !write_all(&mut usb_device, &mut serial, b"> ")
                    {
                        break;
                    }
                }
            }
        }
    }
}

fn write_all<B: UsbBus>(
    usb_device: &mut UsbDevice<'_, B>,
    serial: &mut SerialPort<'_, B>,
    mut bytes: &[u8],
) -> bool {
    while !bytes.is_empty() {
        if usb_device.state() != UsbDeviceState::Configured {
            return false;
        }
        match serial.write(bytes) {
            Ok(count) => bytes = &bytes[count..],
            Err(UsbError::WouldBlock) => {}
            Err(_) => return false,
        }
        usb_device.poll(&mut [&mut *serial]);
    }
    true
}

fn flush<B: UsbBus>(usb_device: &mut UsbDevice<'_, B>, serial: &mut SerialPort<'_, B>) {
    while usb_device.state() == UsbDeviceState::Configured {
        match serial.flush() {
            Ok(()) => return,
            Err(UsbError::WouldBlock) => {
                usb_device.poll(&mut [&mut *serial]);
            }
            Err(_) => return,
        }
    }
}
