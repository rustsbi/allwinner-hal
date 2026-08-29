# Avaota F1 (V821) USB and UART Consoles

This package contains two polling, allocation-free consoles for the V821 E907:

- `usb-uart`: USB CDC-ACM over USB0 D+/D-.
- `uart-demo`: UART0 over the Avaota Hypercard Type-C SBU pins.

Both expose the same two commands:

```text
> hello
hello world
> help
Commands:
  help   show this help
  hello  print hello world
>
```

For `usb-uart`, the host's CDC line-coding request is accepted but does not
configure a physical UART. `uart-demo` uses `115200 8N1` with no flow control.

## UART0 through Avaota Hypercard

The board routes PL4/UART0-TX to Type-C SBU2 and PL5/UART0-RX to SBU1. Both
pads use mux function 3. The PL bank is 1.8 V; use the Hypercard path rather
than connecting a 3.3 V USB-to-UART adapter directly.

On Windows, open the port named `USB-Enhanced-SERIAL CH343`. The COM number is
assigned dynamically. Do not select `CKLink Serial Port`; that is the debug
probe's own serial interface, not the Hypercard SBU UART bridge.

`uart-demo` selects the board's 40 MHz HOSC as the APB special clock and uses
divisor 22. The resulting 113636 baud is within 1.36% of a terminal configured
for 115200 baud and does not depend on a peripheral PLL left enabled by FEL.

Build an independent UART image so it cannot be confused with `usb-uart`:

```powershell
cargo build --release `
  --package v821-avaota-f1 `
  --bin uart-demo `
  --target riscv32imafc-unknown-none-elf

cargo rfel elf2bin `
  --input target\riscv32imafc-unknown-none-elf\release\uart-demo `
  --output target\riscv32imafc-unknown-none-elf\release\uart-demo.bin

cargo rfel patch `
  --input target\riscv32imafc-unknown-none-elf\release\uart-demo.bin `
  --output target\riscv32imafc-unknown-none-elf\release\uart-demo.bt0
```

For a non-persistent test, keep the board in FEL and run:

```powershell
cargo rfel write 0x02000000 `
  target\riscv32imafc-unknown-none-elf\release\uart-demo.bin
cargo rfel exec 0x02000000
```

Open the Hypercard serial port at `115200 8N1`, then enter `hello` or `help`.

## Boot image

The output is an `eGON.BT0` Boot0 image, not an SPI-XIP program. V821 BootROM
reads it from SPI NOR offset `0`, verifies its length and additive checksum,
copies it to SRAM at `0x02000000`, and transfers the E907 to the first
instruction. The same image can also be written to SRAM and executed through
FEL for a non-persistent smoke test.

`allwinner-rt` supplies the eGON header, E907 entry, ROM return path, stack,
and V821 SRAM layout. The example itself keeps only the raw USB0 clock/PHY and
controller accesses; those still use direct volatile pointer reads and writes.
The linked header is the ROM-defined `0x30`-byte common eGON prefix; `0x60` is
a compatible compact layout used by other loaders, not a V821 ROM requirement.

## Build and flash the CDC-ACM example

Keep the board in FEL mode, then run this from the workspace root:

```powershell
cargo run --release `
  --package v821-avaota-f1 `
  --target riscv32imafc-unknown-none-elf
```

`usb-uart` is the package's default binary. The workspace runner maps
`cargo run` to `cargo rfel run --elf`, which builds the ELF, converts it to a
binary, patches the 16 KiB `eGON.BT0` image, detects the attached SPI flash,
and writes the image at offset `0`.

Despite its Cargo name, this command is not compile-only and it does not load
the image into SRAM: it changes persistent flash and does not reset the board.
Use `cargo build` when no device write is intended. The converted files are
kept as `target\rfel-run\firmware.bin` and `target\rfel-run\firmware.img`;
`firmware.img` is the standalone Boot0 image to archive or read back.

## Optional non-persistent FEL smoke test

`cargo run` cannot replace this safety check because its runner writes flash.
To test only in SRAM before the first persistent write, build and patch an
independent image explicitly:

```powershell
cargo build --release `
  --package v821-avaota-f1 `
  --bin usb-uart `
  --target riscv32imafc-unknown-none-elf

cargo rfel elf2bin `
  --input target\riscv32imafc-unknown-none-elf\release\usb-uart `
  --output target\riscv32imafc-unknown-none-elf\release\usb-uart.bin

cargo rfel patch `
  --input target\riscv32imafc-unknown-none-elf\release\usb-uart.bin `
  --output target\riscv32imafc-unknown-none-elf\release\egon.bt0

cargo rfel version
cargo rfel write 0x02000000 `
  target\riscv32imafc-unknown-none-elf\release\egon.bt0
cargo rfel exec 0x02000000
```

The FEL device disconnects and the same port should enumerate as
`V821 USB UART` (`1f3a:8210`) using the operating system's CDC-ACM driver.
The runtime can return an ordinary payload to the ROM, but this console takes
over USB0 and never returns. Restoring only CPU context after a future exit
would not reconstruct the ROM's FEL USB controller state.

## Persistent SPI NOR boot

Before the first write, detect and back up the NOR while the board is in FEL:

```powershell
cargo rfel spinor
$v821BackupDir = Join-Path '..\v821-backups' (Get-Date -Format 'avaota-f1-yyyyMMdd-HHmmss')
New-Item -ItemType Directory -Path $v821BackupDir

cargo rfel spinor read 0x0 0x80000 `
  "$v821BackupDir\boot-prefix-512k.bin"
cargo rfel spinor read 0x0 0x2000000 `
  "$v821BackupDir\spinor-full-32m.bin"

if ((Get-Item "$v821BackupDir\boot-prefix-512k.bin").Length -ne 0x80000) {
  throw 'incomplete Boot0 backup'
}
if ((Get-Item "$v821BackupDir\spinor-full-32m.bin").Length -ne 0x2000000) {
  throw 'incomplete SPI NOR backup'
}
Get-FileHash "$v821BackupDir\boot-prefix-512k.bin"
Get-FileHash "$v821BackupDir\spinor-full-32m.bin"
```

Keep backups outside `target`, because `cargo clean` removes that directory.
The 512 KiB prefix covers all four BootROM NOR candidates and a conservative
erase boundary. Check the command output as well as file sizes: some `rfel`
device-operation failures are printed without a failing process exit code.

After the SRAM smoke test succeeds, return to FEL and either write the explicit
Boot0 image:

```powershell
$v821Image = 'target\riscv32imafc-unknown-none-elf\release\egon.bt0'
$v821ImageLength = (Get-Item $v821Image).Length

cargo rfel spinor write 0x0 $v821Image
cargo rfel spinor read 0x0 $v821ImageLength `
  "$v821BackupDir\egon-readback.bt0"

Get-FileHash $v821Image
Get-FileHash "$v821BackupDir\egon-readback.bt0"
```

or use the primary `cargo run` command above to build, patch, auto-detect the
flash, and write offset `0` in one step. On boards containing more than one
supported flash type, prefer the explicit `spinor write` form. Read back the
written 16 KiB and compare its hash before releasing FEL and resetting.

If cold boot fails, hold FEL while resetting, then restore and verify the saved
prefix before another reset:

```powershell
cargo rfel spinor write 0x0 `
  "$v821BackupDir\boot-prefix-512k.bin"
cargo rfel spinor read 0x0 0x80000 `
  "$v821BackupDir\boot-prefix-restored.bin"

Get-FileHash "$v821BackupDir\boot-prefix-512k.bin"
Get-FileHash "$v821BackupDir\boot-prefix-restored.bin"
```

## Hardware sources

The implementation follows the V821/sun300iw1p1 sources preserved locally:

- Boot0/SRAM limits, SPI NOR copies, header and checksum:
  `allwinner-notes/v821/BOOTROM-ANALYSIS.md`.
- USB0 clock/reset and PHY sequences: V821 BootROM functions `0x52a8`,
  `0x751e..0x75cc`, and `0x86e4..0x87be` in
  `allwinner-notes/v821/ghidra/all-functions.c`.
- UDC register widths, CSR bits and PIO operations:
  `allwinner-v821/tina-v821.v1.0/rtos/lichee/rtos-hal/hal/source/usb/udc/`.
- UART0 clock, reset, pin mux and 16550 setup:
  `allwinner-v821/tina-v821.v1.0/brandy/brandy-2.0/spl/board/sun300iw1p1/clock.c`,
  `spl/drivers/serial.c`, and the Avaota F1 board schematic/EDA sources.

The USB module resets and initializes USB0 itself, so it no longer depends on
clock or PHY state left behind by FEL.
