//! F101-S2/S3 PSRAM initialization; 24 parameter words followed by a size result.
#![no_std]
#![no_main]

use rfel_payload::{ddr, entry};

#[entry(align_stack)]
fn main(parameters: *mut u32) {
    // SAFETY: the FEL host supplies 25 writable words and the run_f101 hardware
    // contract. The appended parameter block need not be naturally aligned.
    unsafe {
        let p = parameters.cast::<ddr::Parameters>().read_unaligned();
        let mut config = ddr::f101::Configuration::new(p);
        let size = ddr::run_f101(&mut config);
        parameters
            .cast::<ddr::Parameters>()
            .write_unaligned(config.parameters);
        parameters.add(24).write_unaligned(size);
    }
}
