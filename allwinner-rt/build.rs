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
    let script = if env::var_os("CARGO_FEATURE_V821_MCU").is_some() {
        LINKER_ALLWINNER_V821_MCU
    } else {
        LINKER_ALLWINNER_D1
    };

    std::fs::write(ld, script).unwrap();
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

const LINKER_ALLWINNER_V821_MCU: &[u8] = br#"
OUTPUT_ARCH(riscv)
ENTRY(head_jump)
MEMORY {
    /* Keep the ROM's live FEL context at 0x02019c00 and above intact. */
    SRAM : ORIGIN = 0x02000000, LENGTH = 0x19c00
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
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > SRAM
    .data : ALIGN(4) {
        sdata = .;
        __global_pointer$ = . + 0x800;
        *(.data .data.*)
        *(.sdata .sdata.*)
        *(.got .got.*)
        . = ALIGN(4);
        edata = .;
        __image_end = .;
    } > SRAM
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        . = ALIGN(4);
        ebss = .;
    } > SRAM
    ASSERT(ADDR(.head) == ORIGIN(SRAM), "V821 eGON header is not at SRAM base")
    ASSERT(SIZEOF(.head) == 0x30, "V821 common eGON header must be 0x30 bytes")
    /* rfel pads to 16 KiB; 0x18000 is the largest safe packed image. */
    ASSERT(__image_end - ORIGIN(SRAM) <= 0x18000, "V821 payload exceeds BootROM image limit")
    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
    }
}"#;
