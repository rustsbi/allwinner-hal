# rfel

Rust Allwinner FEL command-line tool.

## Reference

XFEL project: https://github.com/xboot/xfel

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
