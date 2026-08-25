# Allwinner-HAL

Allwinner-HAL provides Rust hardware support for Allwinner SoCs. The repository
contains the following projects:

| Project | Description | Package | Documentation |
|:--------|:------------|:--------|:--------------|
| [`allwinner-hal`](./allwinner-hal/) | `no_std` peripheral drivers and hardware abstractions | [![crates.io](https://img.shields.io/crates/v/allwinner-hal.svg)](https://crates.io/crates/allwinner-hal) | [![Documentation](https://docs.rs/allwinner-hal/badge.svg)](https://docs.rs/allwinner-hal) |
| [`allwinner-rt`](./allwinner-rt/) | Bare-metal startup and runtime support | [![crates.io](https://img.shields.io/crates/v/allwinner-rt.svg)](https://crates.io/crates/allwinner-rt) | [![Documentation](https://docs.rs/allwinner-rt/badge.svg)](https://docs.rs/allwinner-rt) |
| [`rfel`](./rfel/) | Host-side FEL utility for memory access, code execution, DDR initialization, and SPI flash operations | [![crates.io](https://img.shields.io/crates/v/rfel.svg)](https://crates.io/crates/rfel) | [![Documentation](https://docs.rs/rfel/badge.svg)](https://docs.rs/rfel) |

## Hardware support

| Chip | `allwinner-hal` | `allwinner-rt` | `rfel` |
|:-----|:----------------|:---------------|:-------|
| D1 / D1-H | Supported | Supported | Supported |
| D1s / F133 | Compatible, not tested | Supported | Supported |
| V821 | Not yet supported | CPU runtime supported | Supported |

## Using `rfel`

The workspace provides a Cargo alias for running the FEL utility:

```console
cargo rfel version
```

See the [`rfel` README](./rfel/) for memory, SPI flash, and firmware loading
commands.

## License

This repository is dual-licensed under the MIT License and the Mulan PSL v2.

## References

- [RT-Thread sunxi-hal](https://gitee.com/rtthread/rt-thread/tree/master/bsp/allwinner/libraries/sunxi-hal/hal/source)
- [TinyKasKit](https://github.com/YuzukiHD/TinyKasKit)
- [xfel](https://github.com/xboot/xfel)
