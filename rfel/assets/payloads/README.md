All binaries except `spi_v861.bin` are built from
`examples/rfel-payload`. Third-party notices are retained in
[LICENSE-XFEL](LICENSE-XFEL).

## V861/V881: shared Rust register helpers

V861/V881 reuses `read32_rv32.bin` and `write32_rv32.bin` compiled from
`examples/rfel-payload` for the RV32 E907. Both are 32 bytes; rebuilding the
`read32` and `write32` binaries for `riscv32imac-unknown-none-elf` in release mode
reproduces the bundled bytes. They enable T-Head instructions, execute `fence.i`,
locate appended little-endian parameters relative to the PC, and return without
using the stack. The parameter/result layout matches xfel, even though its
register payloads have different instruction sequences and are 44 bytes long.

## V861/V881: unmodified xfel SPI payload

`spi_v861.bin` is extracted verbatim from the `chip_spi_init` `payload[]` array in
[xboot/xfel chips/v881.c](https://github.com/xboot/xfel/blob/7ab3769640be6847e9ca9c5f2df8b11d228d316b/chips/v881.c),
commit `7ab3769640be6847e9ca9c5f2df8b11d228d316b`. This SPI payload has not been
reverse-engineered, rewritten in Rust, or rebuilt locally.

**License: MIT.** The upstream [repository license](https://github.com/xboot/xfel/blob/7ab3769640be6847e9ca9c5f2df8b11d228d316b/LICENSE)
and [SPIF source notice](https://github.com/xboot/xfel/blob/7ab3769640be6847e9ca9c5f2df8b11d228d316b/payloads/v881/spi/source/sys-spif.c)
identify Jianjun Jiang and Han Gao as copyright holders. Their notices and the
MIT permission/disclaimer text are included in [LICENSE-XFEL](LICENSE-XFEL).

| File | Upstream function | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `spi_v861.bin` | `chip_spi_init` | 1464 | `931a4151c7db1ecaf81311591a37d1029a56539113a67abe3abde84528e61c39` |

The hardware reports FEL ID `0x00191800` (xfel calls it V881). These helpers
execute on the RV32 E907. Register helpers load at the ROM-reported scratchpad
(`0x0011fc00` on the tested board). SPI loads at `0x00100000`, commands at
`0x00101000` (256 bytes), status at `0x00101ffc`, and the 64 KiB swap buffer at
`0x00102000`. The helper owns a private stack below `0x00101f00` and uses the
SPIF controller at `0x04f00000` with the 24 MHz oscillator.

The upstream scope is single-lane SPI NOR with three-byte addresses, including
SFDP, page program and 4/32/64 KiB erase. DDR, JTAG, SPI NAND, four-byte addresses,
quad I/O and DTR are not implemented. See the
[upstream payload README](https://github.com/xboot/xfel/blob/7ab3769640be6847e9ca9c5f2df8b11d228d316b/payloads/v881/spi/README.md).

To reproduce the import, download the pinned `chips/v881.c` above and run:

```sh
python rfel/assets/payloads/import_v861.py /path/to/v881.c --check
```

Omit `--check` to regenerate `spi_v861.bin`. The importer checks the
source hash and the payload hash before writing; no cross compiler is needed.

## V861 hardware verification (2026-09-13)

FEL identification, SID and SPIF version reads agreed with xfel. SRAM word writes
were read back by both tools and the original word was restored. `rfel flash`
detected a 16 MiB NOR via SFDP, and `rfel spinor read` read all 16 MiB successfully.
The installed xfel could not detect this NOR through its `spinor` command; its
raw FEL commands were used with the imported helper to cross-check 64 KiB at
offsets `0`, `0x12345`, `0xff0000`, and 5 bytes at `0xfffffb`. All matched rfel's
full dump. Flash program/erase and watchdog reset were not exercised.

After switching register access to the shared Rust RV32 helpers, SID and SPIF
version reads and alternating SRAM writes/reads were rechecked against xfel.
The original SRAM word was restored, and SPI detection remained functional.
