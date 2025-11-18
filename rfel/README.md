# rfel

Rust Allwinner FEL command-line tool.

## Reference

XFEL project: https://github.com/xboot/xfel

## Auto flash selection

Use `rfel flash` to automatically detect whether a connected board exposes SPI NAND or SPI NOR flash before performing read or write operations. Examples:

- `rfel flash` prints the detected flash type, name, and capacity.
- `rfel flash read 0x0 0x10000 backup.bin` reads 64 KiB from whichever SPI flash is available.
- `rfel flash write 0x0 image.bin` writes the contents of `image.bin` to the detected flash device.
