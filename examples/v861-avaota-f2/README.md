# Avaota F2 (V861 E907)

Port of the Avaota F1 `usb-uart` console. It runs on the V861's RV32 E907
in SRAM; it does not initialize DDR or start the C907 cores.
The target is `riscv32imac-unknown-none-elf` (no floating-point instructions).

## Build and load into SRAM

Run from the workspace root with the board in FEL:

```powershell
rustup target add riscv32imac-unknown-none-elf
cargo build -p v861-avaota-f2 --release --target riscv32imac-unknown-none-elf --bins
rfel elf2bin --input target/riscv32imac-unknown-none-elf/release/usb-uart --output target/f2-usb-uart.bin
rfel write 0x00100000 target/f2-usb-uart.bin
rfel read32 0x00100000
rfel exec 0x00100000
```

Use an `rfel` build with V861 support. The `read32` step executes the existing
RV32 register payload, including `fence.i`, before entering newly uploaded
instructions. These commands only load SRAM. The workspace's `cargo run` runner
instead programs SPI flash, so use the explicit commands above for SRAM tests.

The linker uses `0x00100000..0x00112000`, including an 8 KiB stack, and keeps
the upper SRAM available to the live BootROM. The loadable image is limited
to 64 KiB to accommodate `rfel`'s 16 KiB image padding.

## Consoles

- `usb-uart`: a USB CDC ACM console, VID:PID `1f3a:8610`, serial number
  `V861-AVAOTA-F2`. Windows assigned COM20 during testing. Select this new
  port rather than the CH340 port. This is a command console, not a UART bridge.

The console accepts `help`, `hello`, and `exit`, with CR, LF, CRLF, and backspace editing.
The USB console resets into FEL on `exit` so the host re-enumerates the ROM's
USB device.

## Verification

The USB binary builds for RV32. On Avaota F2, the USB console enumerated as
COM20 and answered `help` and `hello` correctly. `exit` reset the board and
re-enumerated FEL (`1f3a:efe8`); `rfel version` recognized V861 again.
The SRAM-only load commands above were tested without programming flash.
The physical UART example is not included. Runtime UART0 support follows
the schematic (PH9 TX / PH10 RX, mux 5); communication has not been verified.

## References and license

- [Avaota F2 schematic](https://github.com/AvaotaSBC/AvaotaF2/blob/8fa78b726c29e41ee21287ebf3539c204cfd78e9/Hardware/01_SCH/SCH_Schematic1_2026-03-19.pdf): UART0 PH9/PH10 and Type-C wiring.
- [SyterKit F2 SRAM configuration](https://github.com/YuzukiHD/SyterKit/blob/f95d29d611ce439a2510957b0f312b5b439f53b8/boards/avaota-f2/configs/sram_defconfig): E907 and SRAM entry address.
- [V861 CCU register definitions](https://github.com/YuzukiHD/SyterKit/blob/f95d29d611ce439a2510957b0f312b5b439f53b8/include/drivers/clk/sun252iw1/reg.h): APB UART and E907 clock fields, USB OTG gate 8/reset 24, PHY reset 30.

The Rust examples reuse this repository's F1 console and USB implementation
under the workspace license (`MulanPSL-2.0 OR MIT`). No external binary payload
or GPL source code is included in this example; the external sources above
were consulted for hardware configuration facts. The separately imported
`rfel` SPI payload retains its license documentation in
[`rfel/assets/payloads/README.md`](../../rfel/assets/payloads/README.md).
