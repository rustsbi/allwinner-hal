use riscv_target_parser::RiscvTarget;
use std::{env, path::PathBuf};

fn main() {
    load_linker_script();
    load_fpu_features();
}

fn load_fpu_features() {
    // Adapted from `riscv-rt` crate.
    // List of all possible RISC-V configurations to check for in allwinner-rt
    const RISCV_CFG: [&str; 3] = ["riscvf", "riscvd", "riscvq"];
    // Required until target_feature risc-v is stable and in-use (rust 1.75)
    for ext in RISCV_CFG.iter() {
        println!("cargo:rustc-check-cfg=cfg({ext})");
    }
    let target = env::var("TARGET").unwrap();
    let cargo_flags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();

    if let Ok(target) = RiscvTarget::build(&target, &cargo_flags) {
        for flag in target.rustc_flags() {
            // Required until target_feature risc-v is stable and in-use
            if RISCV_CFG.contains(&flag.as_str()) {
                println!("cargo:rustc-cfg={}", flag.as_str());
            }
        }
    }
}

fn load_linker_script() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("allwinner-rt.ld");

    std::fs::write(ld, LINKER_ALLWINNER_D1).unwrap();
    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

const LINKER_ALLWINNER_D1: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(head_jump)
MEMORY {
    SRAM : ORIGIN = 0x00020000, LENGTH = 160K
}
SECTIONS {
    .head : {
        KEEP(*(.text.head))
        KEEP(*(.head.egon))
        . = ALIGN(4);
        KEEP(*(.head.meta))
    } > SRAM
    .text : ALIGN(4) {
        *(.text.entry)
        *(.text .text.*)
    } > SRAM
    .rodata : ALIGN(8) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);
        erodata = .;
    } > SRAM
    .data : ALIGN(8) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8);
        edata = .;
    } > SRAM
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(8) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > SRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";
