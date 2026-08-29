use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let linker_script = out_dir.join("v821-boot0.ld");

    fs::write(&linker_script, include_bytes!("v821-boot0.ld"))
        .expect("write V821 Boot0 linker script");

    println!("cargo:rerun-if-changed=v821-boot0.ld");
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg-bin=usb-uart=-Tv821-boot0.ld");
    println!("cargo:rustc-link-arg-bin=usb-uart=--gc-sections");
}
