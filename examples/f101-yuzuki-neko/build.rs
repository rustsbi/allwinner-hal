fn main() {
    println!("cargo:rustc-link-arg-bin=usb-uart=-Tallwinner-rt.ld");
    println!("cargo:rustc-link-arg-bin=uart-demo=-Tallwinner-rt.ld");
}
