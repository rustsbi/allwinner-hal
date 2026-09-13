# rfel

Rust Allwinner FEL command-line tool.

Supported chips:

- D1 / D1s / D1-H / F133
- V821
- F101-S2 / F101-S3
- V861 / V881 (FEL ID `0x00191800`, E907)

On V821, the `0x0000c000..0x00010000` range is unmapped and is not part of the
BootROM image; do not include it in a ROM dump.

On F101, aligned four-byte reads (including `read32`) execute the RV32 read
helper in the FEL scratchpad at `0x20000`. This also executes `fence.i`, so a
`read32` after loading a new SRAM image synchronizes its instruction fetch
before `exec`. Bulk and unaligned reads use the ROM's memory-transfer path.

The V861/V881 SPI payload is imported unchanged from xfel under the **MIT license**;
see [payload provenance and hashes](assets/payloads/README.md) and the
[xfel copyright and license notice](assets/payloads/LICENSE-XFEL).
V861/V881 register access reuses `read32_rv32.bin` and `write32_rv32.bin`. These
and all other bundled payloads are built from the Rust sources in `examples/rfel-payload`.

V861/V881 supports FEL memory transfers, register reads/writes, SID, watchdog
reset, and single-lane SPI NOR through its dedicated SPIF controller. NOR access
uses SRAM only; DDR initialization and C907 startup are not required. The imported
payload supports three-byte flash addresses and transfers up to 64 KiB. SPI NAND,
four-byte addressing, quad I/O, DDR initialization and JTAG setup are not supported.

## Auto flash selection

Use `rfel flash` to automatically detect whether a connected board exposes SPI NAND or SPI NOR flash before performing read or write operations. Examples:

- `rfel flash` prints the detected flash type, name, and capacity.
- `rfel flash read 0x0 0x10000 backup.bin` reads 64 KiB from whichever SPI flash is available.
- `rfel flash write 0x0 image.bin` writes the contents of `image.bin` to the detected flash device.

## Cargo usage

### `rfel run`

```
rfel run --elf <PATH>
		  [--address <ADDR>]    # defaults to 0x0
		  [--temp-dir <DIR>]    # defaults to target/rfel-run
		  [--keep-temps]        # delete temps unless this flag is set
```

This command converts the given ELF into a raw binary, patches it into a FEL-ready image, and flashes it to the detected SPI flash. Intermediate files (`firmware.bin`, `firmware.img`) live under the temp directory and are removed after flashing unless `--keep-temps` is provided.

### Cargo helpers

- The workspace defines a Cargo alias so `cargo rfel <args>` expands to `cargo run --package rfel --release -- <args>`. See `.cargo/config.toml` for details.
- For embedded targets (`target_os = "none"`), `cargo run --target <triple>` automatically invokes `cargo rfel run --elf <built-elf>`, so building and flashing becomes a single command.
