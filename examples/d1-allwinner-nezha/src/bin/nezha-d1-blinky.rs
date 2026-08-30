#![no_std]
#![no_main]

use allwinner_hal::{ccu::*, prelude::*};
use allwinner_rt::{Clocks, Peripherals, entry};

#[inline]
fn sleep(n: u32) {
    for _ in 0..n * 100_000 {
        unsafe { core::arch::asm!("nop") }
    }
}

#[entry]
fn main(p: Peripherals, _c: Clocks) {
    unsafe {
        LEDC::unmask_gate_only(&p.ccu);
        LEDC::enable_in(&p.ccu);
        p.ccu.ledc_clk.modify(|x| x.enable_clock_gating());
        p.gpio.pc0.into_function::<4>();
    }

    let ledc = p.ledc;
    unsafe {
        // Soft reset LDEC
        ledc.ledc_control.modify(|x| x.clear_soft_reset());
        // Wait reset done
        while ledc.ledc_control.read().soft_reset() {
            core::hint::spin_loop();
        }
    }

    const STEPS: [u8; 3] = [1, 2, 3];
    let mut color = [0, 0, 0];

    let wrap_color = |input: u32, flag: bool| {
        if flag { 255 - input } else { input }
    };
    let color_to_u32 = |color: &[u32; 3]| {
        let green = wrap_color(color[0] & 0xff, ((color[0] & 0x100) >> 8) == 0);
        let red = wrap_color(color[1] & 0xff, ((color[1] & 0x100) >> 8) == 0);
        let blue = wrap_color(color[2] & 0xff, ((color[2] & 0x100) >> 8) == 0);
        (green << 16) | (red << 8) | blue
    };

    loop {
        unsafe {
            // Soft reset LDEC
            ledc.ledc_control.modify(|x| x.clear_soft_reset());
            // Wait reset done
            while ledc.ledc_control.read().soft_reset() {
                core::hint::spin_loop();
            }

            ledc.ledc_data_reg.write(color_to_u32(&color));

            // Send on data
            ledc.ledc_control
                .modify(|x| x.set_total_data_length(1).enable());

            while ledc.ledc_control.read().is_enabled() {
                core::hint::spin_loop();
            }

            sleep(10);
            for i in 0..3 {
                color[i] += STEPS[i] as u32;
            }
        }
    }
}
