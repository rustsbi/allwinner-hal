#![no_std]
#![no_main]

use allwinner_hal::usb::{Usb, UsbBus as AllwinnerUsbBus, phy::v2::UsbPhy};
use allwinner_rt::{Clocks, Peripherals, entry};
use embedded_hal::delay::DelayNs;
use f101_yuzuki_neko::console::{Command, Console, InputEvent};
use riscv::delay::McycleDelay;
use usb_device::{
    UsbError,
    bus::{UsbBus, UsbBusAllocator},
    device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main(p: Peripherals, clocks: Clocks) {
    let mut usb0 = p.usb0;
    let mut usb_phy0 = p.usb_phy0;
    let mut ccu = p.ccu;
    let mut sysctl = p.sysctl;
    let mut delay = McycleDelay::new(clocks.mcycle_ticks_second(&ccu).unwrap());
    let oscillator = clocks.enable_usb(&mut usb0, &mut usb_phy0, &mut ccu, &mut sysctl, &mut delay);

    let usb = Usb::new(usb0, &mut delay);
    let mut _usb_phy = UsbPhy::new(usb_phy0, oscillator, &mut delay);
    if !usb.is_vbus_valid() {
        _usb_phy.force_vbus_valid();
    }

    let usb_bus = UsbBusAllocator::new(AllwinnerUsbBus::new(usb));
    let mut serial = SerialPort::new(&usb_bus);
    let strings = [StringDescriptors::default()
        .manufacturer("RustSBI")
        .product("F101 USB UART")
        .serial_number("F101-YUZUKI-NEKO")];
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1f3a, 0xf101))
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
        if usb_device.state() != UsbDeviceState::Configured || !serial.dtr() {
            greeting_visible = false;
            console = Console::new();
            continue;
        }
        if !greeting_visible {
            // Host serial drivers can purge receive buffers while opening the
            // port, just after asserting DTR. Let that operation finish first.
            delay.delay_ms(50);
            usb_device.poll(&mut [&mut serial]);
            if !write_all(
                &mut usb_device,
                &mut serial,
                b"Welcome to Allwinner-HAL f101-yuzuki-neko example!\r\n> ",
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
                            // Keep servicing IN completion and the host driver
                            // before detaching the serial port.
                            for _ in 0..100 {
                                usb_device.poll(&mut [&mut serial]);
                                delay.delay_ms(1);
                            }
                            usb_device.bus().disconnect();
                            delay.delay_ms(250);
                            allwinner_rt::soc::f101::enter_fel();
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
        if usb_device.state() != UsbDeviceState::Configured || !serial.dtr() {
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
    while usb_device.state() == UsbDeviceState::Configured && serial.dtr() {
        match serial.flush() {
            Ok(()) => return,
            Err(UsbError::WouldBlock) => {
                usb_device.poll(&mut [&mut *serial]);
            }
            Err(_) => return,
        }
    }
}
