use rfel_payload::entry;

#[entry]
fn arbitrary_rust_name(parameters: *mut u32) {
    // SAFETY: the test passes a pointer to its initialized local word.
    unsafe {
        let value = parameters.read_volatile();
        parameters.write_volatile(value.wrapping_add(1));
    }
}

unsafe extern "C" {
    fn __rfel_payload__main(parameters: *mut u32);
}

#[test]
fn exports_the_entry_symbol_and_preserves_the_rust_name() {
    let rust_entry: unsafe extern "C" fn(*mut u32) = arbitrary_rust_name;
    let mut value = 41;
    // SAFETY: both names refer to the generated entry and receive a valid word.
    unsafe {
        __rfel_payload__main(&raw mut value);
        assert_eq!(value, 42);
        rust_entry(&raw mut value);
    }
    assert_eq!(value, 43);
}
