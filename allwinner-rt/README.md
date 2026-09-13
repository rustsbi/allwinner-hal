# Allwinner-RT 运行环境

[![crates.io](https://img.shields.io/crates/v/allwinner-rt.svg)](https://crates.io/crates/allwinner-rt)
[![Documentation](https://docs.rs/allwinner-rt/badge.svg)](https://docs.rs/allwinner-rt)
![License](https://img.shields.io/crates/l/allwinner-rt.svg)

全志芯片的ROM运行环境，目前包括一个入口函数宏。

F101 使用 `default-features = false, features = ["f101", "panic-halt"]`，
目标为 `riscv32imac-unknown-none-elf`，对应 BootROM 的 RV32 machine-mode 入口。
runtime 提供 GPIO、CCU、SYSCTRL、UART1、USB0 和独立 USB PHY 的所有权，
并保留 FEL 使用的 SRAM。当前入口不切换到 RV64，也不初始化 PSRAM。

构建、选择 FEL 设备和加载 SRAM 的步骤见
[`f101-yuzuki-neko`](../examples/f101-yuzuki-neko/README.md)。
