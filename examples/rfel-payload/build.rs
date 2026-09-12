use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("none")
        || !matches!(
            env::var("CARGO_CFG_TARGET_ARCH").as_deref(),
            Ok("riscv32" | "riscv64")
        )
    {
        return;
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // All payloads are position independent and locate appended parameters
    // relative to the PC. Their load address is chosen by the FEL host.
    let script = r#"
OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS {
    . = 0;
    .text : { KEEP(*(.text.payload)) *(.text .text.*) }
    .rodata : { *(.rodata .rodata.* .srodata .srodata.*) }
    .data : { *(.data .data.* .sdata .sdata.*) }
    __payload_end = .;
    .bss (NOLOAD) : { *(.bss .bss.* .sbss .sbss.* COMMON) }
    /DISCARD/ : { *(.eh_frame .eh_frame_hdr) }
}
/* Architecture-gated entries may be absent in non-payload builds. */
ASSERT(DEFINED(_start) ? _start == 0 : 1, "FEL entry moved");
ASSERT(SIZEOF(.bss) == 0, "FEL helpers must not depend on uninitialized BSS");
"#;
    let path = out.join("link.x");
    fs::write(&path, script).unwrap();
    println!("cargo:rustc-link-arg-bins=-T{}", path.display());
    println!("cargo:rustc-link-arg-bins=--no-relax");
}
