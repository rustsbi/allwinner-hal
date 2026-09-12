//! D1/F133 DDR initialization; 24 parameter words followed by a size result.
#![no_std]
#![no_main]

use rfel_payload::{ddr, entry};

#[entry(align_stack)]
fn main(parameters: *mut u32) {
    // SAFETY: the FEL host appends 25 writable words, supplies board-correct
    // parameters, and grants the hardware access required by run_d1. Appended
    // data can be unaligned because payload instruction sizes vary.
    unsafe {
        let mut config = parameters.cast::<ddr::Parameters>().read_unaligned();
        let size = ddr::run_d1(&mut config);
        parameters.cast::<ddr::Parameters>().write_unaligned(config);
        parameters.add(24).write_unaligned(size);
    }
}
