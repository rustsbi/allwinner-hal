# Yuzuki Neko (F101 C907)

`usb-uart` and `uart-demo` run in SRAM on the F101 C907 in the BootROM's
RV32 machine mode, using `riscv32imac-unknown-none-elf`. They use the
`allwinner-rt` `f101` feature with default features disabled. PSRAM is not
initialized and the runtime does not switch to RV64.

## Build and load

Run from the workspace root with the board in FEL:

```powershell
cargo build -p f101-yuzuki-neko --release --target riscv32imac-unknown-none-elf --bins
rfel version
```

Find the entry whose chip is **F101** and copy its full selector. For example,
the tested Windows system had F101 at the following path, with another board
on a different port:

```powershell
$f101Device = 'PCIROOT(0)#PCI(1400)#USBROOT(0)#USB(4)#USB(3)'
rfel elf2bin --input target/riscv32imac-unknown-none-elf/release/usb-uart --output target/f101-usb-uart.bin
rfel --device $f101Device write 0x22000 target/f101-usb-uart.bin
rfel --device $f101Device write 0x20000 rfel/assets/payloads/fence_i_f101.bin
rfel --device $f101Device exec 0x20000
rfel --device $f101Device exec 0x22000
```

Use the selector printed on your own machine; on Linux it has the form
`BUS-PORT.PORT`. Pass it to every hardware operation. Execute the supplied
`fence.i` helper from the FEL scratchpad before entering a replaced image.
With `rfel` built from this revision, `rfel --device $f101Device read32 0x22000`
can replace the two scratchpad commands: its RV32 read helper also executes
`fence.i`. Older `rfel` versions used a ROM memory read without that barrier.
These commands load SRAM. The workspace `cargo run` runner programs flash;
use the explicit SRAM commands above for these tests.

To run the physical UART console, replace both `usb-uart` filenames with
`uart-demo` and use the same load address and device selector.

The linker places the image, BSS and 8 KiB stack in `0x22000..0x2c000`.
It excludes the FEL scratchpad at `0x20000`, the live ROM stack/data above the
image region, and SRAM assigned to USB FIFOs. The loadable image is limited
to 32 KiB, including space for `rfel patch`'s 16 KiB padding if used.

## Consoles

- `usb-uart`: full-speed USB CDC ACM, VID:PID `1f3a:f101`, serial descriptor
  `F101-YUZUKI-NEKO`. Open its new serial port with DTR asserted. This is a
  command console. The nominal 115200 baud setting does not change USB speed.
- `uart-demo`: UART1, PB0 TX / PB1 RX, mux function 4, 115200 baud, 8N1.
  Connect a 3.3 V adapter's RX to PB0, TX to PB1, and share ground. It uses
  the BootROM's inherited 24 MHz UART clock.

Both accept `help`, `hello`, and `exit`, with CR, LF, CRLF and backspace editing.
The USB console disconnects before re-entering the ROM FEL path; the physical
UART console flushes its transmitter and returns to the FEL caller.

## Verification

```powershell
cargo test -p allwinner-hal --lib
cargo test -p allwinner-rt --no-default-features --features f101 --lib
cargo test -p f101-yuzuki-neko --lib
python -m pip install pyserial
python examples/f101-yuzuki-neko/test_usb_uart.py --device $f101Device
```

Run the Python check after loading `usb-uart`. It identifies the CDC port by
VID:PID, checks commands and line editing, sends writes around the 64-byte
packet boundary, stresses 6400 commands, reopens the port and verifies that
`exit` restores FEL at the selected physical USB path. A successful run leaves
the board in FEL. `--report target/f101-usb-test.json` saves the results.

| Capability | Scope |
| --- | --- |
| C907 RV32 SRAM startup | BSS/data, 16-byte stack alignment and return to FEL checked on F101-S3 |
| USB CDC | Full-speed PIO; three complete runs, 19200 stress commands, packet boundaries, DTR/reopen and exit to FEL checked on F101-S3 |
| UART1 | Compiled; physical wiring not connected for this validation |
| F101-S2 | Same software path; separate silicon validation pending |
| RV64, PSRAM, DMA, high speed, suspend/resume, remote wake | Not validated by these examples |

## Hardware sources

The implementation uses hardware configuration facts from these sources;
external source code or binary payloads are not included in these examples.

- [F101 boot guide](https://docs.aw-ol.com/docs/soc/f101/sdk-basics/%E5%90%AF%E5%8A%A8_%E4%BD%BF%E7%94%A8%E6%8C%87%E5%8D%97/).
- [Yuzuki Neko board description](https://github.com/YuzukiHD/SyterKit/blob/f95d29d611ce439a2510957b0f312b5b439f53b8/boards/yuzukineko/board.dts): USB and UART1 mapping, clock/reset bits and PB0/PB1 mux.
- [F101 CCU definitions](https://github.com/YuzukiHD/SyterKit/blob/f95d29d611ce439a2510957b0f312b5b439f53b8/include/drivers/clk/sun252iw2/reg.h): CPU and peripheral clock fields.
- [F101 USB SRAM mapping](https://github.com/YuzukiHD/SyterKit/blob/f95d29d611ce439a2510957b0f312b5b439f53b8/drivers/usb/platform/usb_sun252iw2.c): SYSCTRL SRAM bits 25/27.

The connected F101 ROM was also read to verify ROM data initialization and
stack placement. The saved FEL return PC (`0x0000614e`) and trap vector
(`0x00001da0`) confirm the low instruction window, with FEL entry at `0x00000020`.
The SHA-256 of the 64 KiB dump through the data alias at `0x06000000` is
`c53ffb19a8263c96d6f6b067611a8e0d3c23bbc9ba950942b38dfd69fc441544`.
