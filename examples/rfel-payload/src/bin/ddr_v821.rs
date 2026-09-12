//! V821 DDR initialization; 24 parameter words followed by a size result.
#![no_std]
#![no_main]

use rfel_payload::{ddr, entry};

#[entry(align_stack)]
fn main(parameters: *mut u32) {
    // SAFETY: the FEL host supplies 25 writable words and the run_v821 hardware
    // contract. The appended parameter block need not be naturally aligned.
    unsafe {
        let mut config = parameters.cast::<ddr::Parameters>().read_unaligned();
        let size = ddr::run_v821(&mut config);
        parameters.cast::<ddr::Parameters>().write_unaligned(config);
        parameters.add(24).write_unaligned(size);
    }
}
