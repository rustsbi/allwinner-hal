# allwinner-hal

[![crates.io](https://img.shields.io/crates/v/allwinner-hal.svg)](https://crates.io/crates/allwinner-hal)
[![Documentation](https://docs.rs/allwinner-hal/badge.svg)](https://docs.rs/allwinner-hal)
![License](https://img.shields.io/crates/l/allwinner-hal.svg)

全志芯片组件化外设驱动；外设包括DDR控制器。

## 支持列表

组件化外设驱动可根据外设基地址提供驱动支持，这要求外设的设计（寄存器定义和功能等）相同；
大多数情况下，外设IP核相同就能满足以上条件。

按此要求，虽然全志系列芯片大部分外设IP核都是共享的，但本项目只测试了一部分芯片的兼容性。
经过测试能够运行本项目的芯片如下。

| 系列* | CCU | GPIO | SPI | UART |
|:-----|:----|:----|:----|:----|
| D1系列 | ○ | ○ | ○ | ○ |

✓：可运行，功能完整
○：可运行，详细功能仍需完善
×：暂未支持

*全志芯片中有一部分型号的外设基地址布局相同，即使其中的处理器核可能不相同。
本项目称这些芯片为一个系列，并以系列中首个发布的芯片型号命名。

理论上兼容，但尚未经过测试的芯片型号如下：

- D1系列：D1s, F133, V853, R128

## 许可协议

本项目使用MIT和Mulan-PSL v2.0双许可协议。

## 参考资料

以下资料提供了全志芯片驱动的参考设计或社区设计，供贡献者参考。

- RT-Thread驱动：https://gitee.com/rtthread/rt-thread/tree/master/bsp/allwinner/libraries/sunxi-hal/hal/source
- TinyKasKit项目：https://github.com/YuzukiHD/TinyKasKit
