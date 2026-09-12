//! The byte-command interpreter shared by xfel's three SPI payloads.
//! Ported from xboot/xfel; see LICENSE-XFEL. Register offsets and operation order
//! were also checked against the payload disassembly.

use allwinner_hal::{
    ccu::{PeriFactorN, SpiClockSource, d1, f101, v821},
    gpio::v2::RegisterBlockV2,
    spi::RegisterBlock,
};
use core::ptr::{read_volatile, write_volatile};

/// Register map and pin/clock setup selected by the binary, not detected at runtime.
#[derive(Clone, Copy)]
pub enum Soc {
    D1,
    F101,
    V821,
}

// These raw accesses cover SPI TCR fields which do not yet have value-type setters.
unsafe fn modify(address: usize, clear: u32, set: u32) {
    // SAFETY: every caller supplies an aligned RW register owned exclusively by FEL.
    unsafe {
        write_volatile(
            address as *mut u32,
            (read_volatile(address as *const u32) & !clear) | set,
        )
    };
}

// Keep the constant SoC visible through run's cross-crate inlining.
#[inline(always)]
unsafe fn init(soc: Soc, spi: &RegisterBlock, base: usize) {
    // SAFETY: run's contract provides the correct chip and exclusive peripheral access.
    unsafe {
        let (gpio_base, first, last, function) = match soc {
            Soc::D1 => (0x0200_0000, 2, 5, 2),
            Soc::F101 => (0x0200_0000, 0, 5, 3),
            Soc::V821 => (0x4200_0000, 8, 11, 3),
        };
        let gpio = &*(gpio_base as *const RegisterBlockV2);
        for pin in first..=last {
            let shift = (pin % 8) * 4;
            gpio.sys_port[2].cfg[pin / 8].modify(|v| (v & !(0xf << shift)) | (function << shift));
        }
        match soc {
            Soc::D1 => {
                let ccu = &*(0x0200_1000 as *const d1::RegisterBlock);
                ccu.spi_bgr.modify(|v| v.deassert_reset::<0>());
                ccu.spi_clk[0].modify(|v| v.unmask_clock());
                ccu.spi_bgr.modify(|v| v.gate_pass::<0>());
                ccu.spi_clk[0].modify(|v| {
                    // Match xfel's two-bit source mask. D1-H manual 3.2.6.65
                    // defines bits 26:24 as one field: if bit 26 was set, this
                    // reproduces xfel's reserved 0b101 selection, not PLL_PERI(1X).
                    let source = v.set_clock_source(SpiClockSource::PllPeri1x);
                    d1::SpiClock::from_bits(source.bits() | (v.bits() & (1 << 26)))
                });
                ccu.spi_clk[0].modify(|v| v.set_factor_n(PeriFactorN::N1));
                ccu.spi_clk[0].modify(|v| v.set_factor_m(5));
            }
            Soc::F101 => {
                let ccu = &*(0x0200_1000 as *const f101::RegisterBlock);
                ccu.spi_bgr.modify(|v| v.deassert_reset());
                ccu.spi_bgr.modify(|v| v.gate_pass());
                ccu.spi_clk.modify(|v| v.unmask_clock());
                ccu.spi_clk.modify(|v| v.select_hosc());
                ccu.spi_clk.modify(|v| v.set_factor_n(PeriFactorN::N1));
                ccu.spi_clk.modify(|v| v.set_factor_m(0));
            }
            Soc::V821 => {
                let ccu = &*(0x4200_1000 as *const v821::AppRegisterBlock);
                ccu.bus_reset1.modify(|v| v.deassert_spi0());
                ccu.bus_clock_gating1.modify(|v| v.pass_spi0());
                ccu.spi_clock.modify(|v| v.unmask_clock());
                ccu.spi_clock
                    .modify(|v| v.set_clock_source(v821::SpiClockSource::Peri307M));
                ccu.spi_clock.modify(|v| v.set_factor_n(PeriFactorN::N1));
                ccu.spi_clock.modify(|v| v.set_factor_m(2));
            }
        }
        write_volatile((base + 0x24) as *mut u32, 0x1000); // SPI_CCR: divide by two.
        spi.gcr.modify(|v| {
            v.software_reset()
                .set_transmit_pause_enable(true)
                .set_master_mode()
                .set_enabled(true)
        });
        // The HAL's is_software_reset_finished currently returns the asserted bit.
        while read_volatile((base + 4) as *const u32) & (1 << 31) != 0 {}
        modify(base + 8, 3, (1 << 6) | (1 << 2));
        spi.fcr.modify(|v| v.reset_fifos());
    }
}

unsafe fn select(base: usize, active: bool) {
    // SAFETY: TCR is the selected chip's exclusively owned RW configuration register.
    unsafe {
        modify(
            base + 8,
            (3 << 4) | (1 << 7),
            if active { 0 } else { 1 << 7 },
        )
    };
}

unsafe fn transfer(spi: &RegisterBlock, mut tx: *const u8, mut rx: *mut u8, mut len: u32) {
    // SAFETY: caller supplies valid buffers, or null for dummy TX/discarded RX.
    // Byte FIFO transactions and the 64-byte burst limit match the original payload.
    unsafe {
        while len != 0 {
            let count = len.min(64);
            spi.mbc.write(count);
            spi.mtc.write(count);
            // All bursts fit in the HAL's single-mode counter field.
            spi.bcc
                .write(core::mem::transmute::<u32, allwinner_hal::spi::BurstControl>(count));
            for _ in 0..count {
                let byte = if tx.is_null() {
                    0xff
                } else {
                    let byte = read_volatile(tx);
                    tx = tx.add(1);
                    byte
                };
                spi.txd.write_u8(byte);
            }
            spi.tcr.modify(|v| v.start_burst_exchange());
            while !spi.tcr.read().burst_finished() {}
            while (spi.fsr.read().receive_fifo_counter() as u32) < count {}
            for _ in 0..count {
                let byte = spi.rxd.read_u8();
                if !rx.is_null() {
                    write_volatile(rx, byte);
                    rx = rx.add(1);
                }
            }
            len -= count;
        }
    }
}

unsafe fn next_byte(cursor: &mut *const u8) -> u8 {
    // SAFETY: the host supplies a valid, terminated command stream in SRAM.
    unsafe {
        let byte = read_volatile(*cursor);
        *cursor = cursor.add(1);
        byte
    }
}

unsafe fn next_word(cursor: &mut *const u8) -> u32 {
    // Descriptors are byte packed: an unaligned u32 MMIO load would be incorrect.
    // SAFETY: the command descriptor contains four readable bytes.
    unsafe {
        u32::from_le_bytes([
            next_byte(cursor),
            next_byte(cursor),
            next_byte(cursor),
            next_byte(cursor),
        ])
    }
}

/// Run SPI commands until END or an unknown opcode.
///
/// # Safety
///
/// Requires the matching SoC, exclusive SPI0/pin/clock access, an ABI-aligned
/// SRAM stack, and a valid terminated command stream with valid SRAM buffers.
/// INIT must precede SPI use.
// Each binary supplies a constant SoC; inline across crates so unused register
// maps and clock-setup branches disappear from its SRAM image.
#[inline(always)]
pub unsafe fn run(soc: Soc, commands: *const u8) {
    let base = match soc {
        Soc::D1 | Soc::F101 => 0x0402_5000,
        Soc::V821 => 0x4402_5000,
    };
    // SAFETY: the caller establishes the map, buffers and exclusive access above.
    unsafe {
        let spi = &*(base as *const RegisterBlock);
        let mut cursor = commands;
        loop {
            match next_byte(&mut cursor) {
                1 => init(soc, spi, base),
                2 => select(base, true),
                3 => select(base, false),
                4 => {
                    let len = next_byte(&mut cursor) as u32;
                    transfer(spi, cursor, core::ptr::null_mut(), len);
                    cursor = cursor.add(len as usize);
                }
                opcode @ (5 | 6) => {
                    let address = next_word(&mut cursor) as usize;
                    let len = next_word(&mut cursor);
                    if opcode == 5 {
                        transfer(spi, address as *const u8, core::ptr::null_mut(), len);
                    } else {
                        transfer(spi, core::ptr::null(), address as *mut u8, len);
                    }
                }
                opcode @ (7 | 8) => {
                    let tx = if opcode == 7 { [0x05, 0] } else { [0x0f, 0xc0] };
                    let mut status = 0u8;
                    loop {
                        // Keep the host-selected CS asserted across status polls,
                        // including the command retransmission used by the originals.
                        transfer(
                            spi,
                            tx.as_ptr(),
                            core::ptr::null_mut(),
                            if opcode == 7 { 1 } else { 2 },
                        );
                        transfer(spi, core::ptr::null(), &mut status, 1);
                        if status & 1 == 0 {
                            break;
                        }
                    }
                }
                _ => return,
            }
        }
    }
}
