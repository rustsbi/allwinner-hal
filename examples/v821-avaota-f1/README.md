# Avaota F1 (V821) USB UART Boot0

`usb-uart` is a polling, allocation-free USB CDC-ACM console for the V821 E907.
It exposes two commands:

```text
> hello
hello world
> help
Commands:
  help   show this help
  hello  print hello world
>
```

The host's CDC line-coding request is accepted but does not configure a
physical UART. `115200 8N1` is a convenient terminal setting.

## Boot image

The output is an `eGON.BT0` Boot0 image, not an SPI-XIP program. V821 BootROM
reads it from SPI NOR offset `0`, verifies its length and additive checksum,
copies it to SRAM at `0x02000000`, and transfers the E907 to the first
instruction. The same image can also be written to SRAM and executed through
FEL for a non-persistent smoke test.

The example keeps its small RV32 entry, eGON header, linker script, and raw
USB0 clock/PHY initialization locally. The repository's current
`allwinner-rt` V821 paths target unfinished runtimes and are not used here.

## Build `egon.bt0`

From the workspace root:

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
```

`rfel patch` pads the image to 16 KiB, writes the final length at offset
`0x10`, and replaces the stamp at `0x0c` with the eGON checksum.

## Non-persistent FEL smoke test

Keep the board in FEL mode and run the exact patched image from SRAM first:

```powershell
cargo rfel version
cargo rfel write 0x02000000 `
  target\riscv32imafc-unknown-none-elf\release\egon.bt0
cargo rfel exec 0x02000000
```

The FEL device disconnects and the same port should enumerate as
`V821 USB UART` (`1f3a:8210`) using the operating system's CDC-ACM driver.

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

or build, patch, and flash offset `0` in one command through the workspace
runner:

```powershell
cargo run --release `
  --package v821-avaota-f1 `
  --bin usb-uart `
  --target riscv32imafc-unknown-none-elf
```

`cargo run` maps to `cargo rfel run --elf`; it writes persistent flash and does
not reset the board. On boards containing more than one supported flash type,
prefer the explicit `spinor write` form. Read back the written 16 KiB and
compare its hash before releasing FEL and resetting.

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

The USB module resets and initializes USB0 itself, so it no longer depends on
clock or PHY state left behind by FEL.
