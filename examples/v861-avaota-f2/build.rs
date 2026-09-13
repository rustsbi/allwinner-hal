fn main() {
    println!("cargo:rustc-link-arg-bin=usb-uart=-Tallwinner-rt.ld");
}
