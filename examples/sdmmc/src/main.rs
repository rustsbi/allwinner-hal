#![no_std]
#![no_main]

use core::arch::asm;

use allwinner_hal::{
    ccu::{PeriFactorN, SmhcClockSource},
    smhc::{BusWidthBits, TransferDirection},
    uart::{Config, Serial},
};
use allwinner_rt::{entry, Clocks, Peripherals};
use embedded_io::Write;
use embedded_sdmmc::sdcard::proto;
use embedded_sdmmc::Block;
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) {
    let tx = p.gpio.pb8.into_function::<6>();
    let rx = p.gpio.pb9.into_function::<6>();
    let mut serial = Serial::new(p.uart0, (tx, rx), Config::default(), &c, &p.ccu);

    writeln!(serial, "Hello World!").ok();

    writeln!(serial, "initialize sdmmc pins...").ok();
    let _sdmmc_pins = {
        let sdc0_d1 = p.gpio.pf0.into_function::<2>();
        let sdc0_d0 = p.gpio.pf1.into_function::<2>();
        let sdc0_clk = p.gpio.pf2.into_function::<2>();
        let sdc0_cmd = p.gpio.pf3.into_function::<2>();
        let sdc0_d3 = p.gpio.pf4.into_function::<2>();
        let sdc0_d2 = p.gpio.pf5.into_function::<2>();
        (sdc0_d1, sdc0_d0, sdc0_clk, sdc0_cmd, sdc0_d3, sdc0_d2)
    };

    writeln!(serial, "initialize smhc...").ok();
    const SMHC_IDX: usize = 0;
    let smhc = &p.smhc0;
    let divider = 2;
    let (factor_n, factor_m) = calc_clock_factor(20_000_000.Hz(), c.psi);
    unsafe {
        smhc.clock_control.modify(|val| val.disable_card_clock());

        p.ccu.smhc_bgr.modify(|val| val.assert_reset::<SMHC_IDX>());
        p.ccu.smhc_bgr.modify(|val| val.gate_mask::<SMHC_IDX>());
        p.ccu.smhc_clk[SMHC_IDX].modify(|val| {
            val.set_clock_source(SmhcClockSource::PllPeri1x)
                .set_factor_n(factor_n)
                .set_factor_m(factor_m)
                .enable_clock_gating()
        });
        p.ccu
            .smhc_bgr
            .modify(|val| val.deassert_reset::<SMHC_IDX>());
        p.ccu.smhc_bgr.modify(|val| val.gate_pass::<SMHC_IDX>());

        smhc.global_control.modify(|val| val.set_software_reset());
        while !smhc.global_control.read().is_software_reset_cleared() {}
        smhc.global_control.modify(|val| val.set_fifo_reset());
        while !smhc.global_control.read().is_fifo_reset_cleared() {}
        smhc.global_control.modify(|val| val.disable_interrupt());

        smhc.command.modify(|val| {
            val.enable_wait_for_complete()
                .enable_change_clock()
                .set_command_start()
        });
        while !smhc.command.read().is_command_start_cleared() {}

        smhc.clock_control
            .modify(|val| val.set_card_clock_divider(divider - 1));
        smhc.sample_delay_control.modify(|val| {
            val.set_sample_delay_software(0)
                .enable_sample_delay_software()
        });
        smhc.clock_control.modify(|val| val.enable_card_clock());

        smhc.command.modify(|val| {
            val.enable_wait_for_complete()
                .enable_change_clock()
                .set_command_start()
        });
        while !smhc.command.read().is_command_start_cleared() {}

        smhc.bus_width
            .modify(|val| val.set_bus_width(BusWidthBits::OneBit));
        smhc.block_size
            .modify(|val| val.set_block_size(Block::LEN as u16));
    }

    writeln!(serial, "initializing SD card...").ok();
    // CMD0(reset) -> CMD8(check voltage and sdcard version)
    // -> CMD55+ACMD41(init and read OCR) -> CMD2(read CID)
    /// Host supports high capacity
    const OCR_HCS: u32 = 0x40000000;
    /// Card has finished power up routine if bit is high
    const OCR_NBUSY: u32 = 0x80000000;
    /// Valid bits for voltage setting
    const OCR_VOLTAGE_MASK: u32 = 0x007FFF80;
    send_card_command(
        smhc,
        proto::CMD0,
        0,
        TransferMode::Disable,
        ResponseMode::Disable,
        false,
    );
    sleep(100); // TODO: wait for interrupt instead of sleep
    send_card_command(
        smhc,
        proto::CMD8,
        0x1AA,
        TransferMode::Disable,
        ResponseMode::Short,
        true,
    );
    sleep(100);
    let data = smhc.responses[0].read();
    if data != 0x1AA {
        writeln!(
            serial,
            "unexpected response to CMD8: {:#010X}, expected 0x1AA",
            data
        )
        .ok();
        loop {}
    }
    loop {
        send_card_command(
            smhc,
            proto::CMD55,
            0,
            TransferMode::Disable,
            ResponseMode::Short,
            true,
        );
        sleep(100);
        send_card_command(
            smhc,
            proto::ACMD41,
            OCR_VOLTAGE_MASK & 0x00ff8000 | OCR_HCS,
            TransferMode::Disable,
            ResponseMode::Short,
            false,
        );
        sleep(100);
        let ocr = smhc.responses[0].read();
        if (ocr & OCR_NBUSY) == OCR_NBUSY {
            break;
        }
    }
    const CMD2: u8 = 0x02; // TODO: should be added in `embedded_sdmmc`
    send_card_command(
        smhc,
        CMD2,
        0,
        TransferMode::Disable,
        ResponseMode::Long,
        true,
    );
    sleep(100);
    let cid: u128 = {
        let mut cid = 0u128;
        for i in 0..4 {
            cid |= (smhc.responses[i].read() as u128) << (32 * i);
        }
        cid
    };
    writeln!(serial, "initialize SD card success. CID={:032X}", cid).ok();
    // CID decoder: https://archive.goughlui.com/static/cidecode.htm

    // TODO: support read and write operations

    loop {}
}

#[inline(always)]
fn sleep(n: u32) {
    for _ in 0..n * 100_000 {
        unsafe { asm!("nop") }
    }
}

#[inline(always)]
fn send_card_command(
    smhc: &allwinner_hal::smhc::RegisterBlock,
    cmd: u8,
    arg: u32,
    transfer_mode: TransferMode,
    response_mode: ResponseMode,
    crc_check: bool,
) {
    let (data_trans, trans_dir) = match transfer_mode {
        TransferMode::Disable => (false, TransferDirection::Read),
        TransferMode::Read => (true, TransferDirection::Read),
        TransferMode::Write => (true, TransferDirection::Write),
    };
    let (resp_recv, resp_size) = match response_mode {
        ResponseMode::Disable => (false, false),
        ResponseMode::Short => (true, false),
        ResponseMode::Long => (true, true),
    };
    unsafe {
        smhc.argument.modify(|val| val.set_argument(arg));
        smhc.command.write({
            let mut val = allwinner_hal::smhc::Command::new()
                .set_command_start()
                .set_command_index(cmd)
                .set_transfer_direction(trans_dir)
                .enable_wait_for_complete()
                .enable_auto_stop();
            if data_trans {
                val = val.enable_data_transfer();
            }
            if crc_check {
                val = val.enable_check_response_crc();
            }
            if resp_recv {
                val = val.enable_response_receive();
            }
            if resp_size {
                val = val.enable_long_response();
            }
            val
        });
    };
}

#[inline(always)]
fn calc_clock_factor(freq: Hertz, psi: Hertz) -> (PeriFactorN, u8) {
    let mut err = psi;
    let (mut best_n, mut best_m) = (0, 0);
    for m in 1u8..=16 {
        for n in [1, 2, 4, 8] {
            let actual = psi / n / m as u32;
            let diff = {
                if actual > freq {
                    actual - freq
                } else {
                    freq - actual
                }
            };
            if diff < err {
                err = diff;
                (best_n, best_m) = (n, m);
            }
        }
    }
    let factor_n = match best_n {
        1 => PeriFactorN::N1,
        2 => PeriFactorN::N2,
        4 => PeriFactorN::N4,
        8 => PeriFactorN::N8,
        _ => unreachable!(),
    };
    let factor_m = best_m - 1;
    (factor_n, factor_m)
}

enum TransferMode {
    Disable,
    Read,
    Write,
}

enum ResponseMode {
    Disable,
    Short,
    Long,
}
