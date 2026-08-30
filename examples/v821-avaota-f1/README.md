# Avaota F1 (V821) Examples

This package contains five allocation-free examples for the V821 E907:

- `usb-uart`: USB CDC-ACM over USB0 D+/D- (default).
- `uart-demo`: UART0 through the Avaota Hypercard at `115200 8N1`.
- `usb-storage`: a read-only USB drive named `Avaota F1`, containing one
  `README.txt`; safely ejecting the drive returns to FEL.
- `usb-network`: a CDC-NCM Ethernet device with SLAAC at `2001:db8::1`;
  leaving its data interface disabled during safe removal returns to FEL.
- `usb-composite`: one USB device exposing both the CDC-ACM console and a
  read-only mass-storage volume.

The three consoles support `help`, `hello`, and `exit`. `hello` prints
`hello world`, while `exit` returns to FEL.

## Flash

Put the board in FEL mode, then run from the workspace root.

USB CDC-ACM:

```powershell
cargo run --release -p v821-avaota-f1 --bin usb-uart --target riscv32imafc-unknown-none-elf
```

UART0:

```powershell
cargo run --release -p v821-avaota-f1 --bin uart-demo --target riscv32imafc-unknown-none-elf
```

USB mass storage:

```powershell
cargo run --release -p v821-avaota-f1 --bin usb-storage --target riscv32imafc-unknown-none-elf
```

USB network:

```powershell
cargo run --release -p v821-avaota-f1 --bin usb-network --target riscv32imafc-unknown-none-elf
```

USB CDC-ACM plus mass storage:

```powershell
cargo run --release -p v821-avaota-f1 --bin usb-composite --target riscv32imafc-unknown-none-elf
```

The workspace runner converts the ELF into an `eGON.BT0` image and writes it
to the attached SPI flash at offset `0`. Reset the board after flashing. Use
`cargo build` instead when no flash write is intended.

## Connect

- `usb-uart` enumerates as `V821 USB UART` using the operating system's
  CDC-ACM driver.
- `usb-network` uses CDC-NCM and advertises `2001:db8::/64` through SLAAC.
  After the host configures its address, run `ping.exe -6 2001:db8::1`.
- `usb-composite` enumerates one CDC-ACM serial port and one read-only FAT12
  drive named `V821 CDCMSC` at the same time. Safely ejecting the drive does
  not exit the payload; enter `exit` in the serial console to return to FEL.
  Transfers are serviced cooperatively, so CDC and MSC are not real-time
  independent.
- For `uart-demo`, open `USB-Enhanced-SERIAL CH343` at `115200 8N1`. Do not
  select `CKLink Serial Port`.
