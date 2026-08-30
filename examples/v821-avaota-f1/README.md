# Avaota F1 (V821) Examples

This package contains three allocation-free examples for the V821 E907:

- `usb-uart`: USB CDC-ACM over USB0 D+/D- (default).
- `uart-demo`: UART0 through the Avaota Hypercard at `115200 8N1`.
- `usb-storage`: a read-only USB drive named `Avaota F1`, containing one
  `README.txt`; safely ejecting the drive returns to FEL.

The two consoles support `help`, `hello`, and `exit`. `hello` prints
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

The workspace runner converts the ELF into an `eGON.BT0` image and writes it
to the attached SPI flash at offset `0`. Reset the board after flashing. Use
`cargo build` instead when no flash write is intended.

## Connect

- `usb-uart` enumerates as `V821 USB UART` using the operating system's
  CDC-ACM driver.
- For `uart-demo`, open `USB-Enhanced-SERIAL CH343` at `115200 8N1`. Do not
  select `CKLink Serial Port`.
