use super::{
    Pads,
    register::{BurstControl, GlobalControl, RegisterBlock, TransferControl},
};
use crate::gpio::FlexPad;
use embedded_hal::spi::Mode;

/// Managed SPI structure with peripheral and pins.
///
/// Transfers use bursts of at most 63 bytes and return with the bus idle and
/// both FIFOs empty. Reads and missing transmit bytes send `0xff` on MOSI.
/// Polling waits indefinitely for the hardware to make progress.
pub struct Spi<'a, SPI> {
    spi: SPI,
    #[allow(unused)]
    pads: (
        Option<FlexPad<'a>>,
        Option<FlexPad<'a>>,
        Option<FlexPad<'a>>,
    ),
}

// Ref: rustsbi-d1 project
impl<'a, SPI: AsRef<RegisterBlock>> Spi<'a, SPI> {
    // Like U-Boot's sun4i_spi_xfer, leave one entry free in the 64-byte FIFO.
    const MAX_BURST: usize = 63;

    /// Create an SPI instance.
    pub fn new<const I: usize>(
        spi: SPI,
        pads: impl Pads<'a, I>,
        mode: impl Into<Mode>,
        // freq: Hertz,
        // clock: impl Clock,
        // ccu: &ccu::d1::RegisterBlock,
    ) -> Self {
        // TODO move clock out of SPI initialization
        // // 1. unwrap parameters
        // let (Hertz(psi), Hertz(freq)) = (clock.spi_clock(), freq);
        // let (factor_n, factor_m) = ccu::calculate_best_peripheral_factors_nm(psi, freq);
        // // 2. init peripheral clocks
        // // Reset and reconfigure clock source and divider
        // unsafe { PINS::Clock::reconfigure(ccu, SpiClockSource::PllPeri1x, factor_m, factor_n) };
        // 3. global configuration and soft reset
        unsafe {
            spi.as_ref().gcr.write(
                GlobalControl::default()
                    .set_enabled(true)
                    .set_master_mode()
                    .set_transmit_pause_enable(true)
                    .software_reset(),
            )
        };
        while spi.as_ref().gcr.read().is_software_reset_finished() {
            core::hint::spin_loop();
        }
        // 4. configure work mode
        unsafe {
            spi.as_ref()
                .tcr
                .write(TransferControl::default().set_work_mode(mode.into()))
        };
        // Finally, return ownership of this structure.
        Spi {
            spi,
            pads: pads.into_spi_pads(),
        }
    }
}

impl<'a, SPI: AsRef<RegisterBlock>> embedded_hal::spi::ErrorType for Spi<'a, SPI> {
    type Error = embedded_hal::spi::ErrorKind;
}

impl<'a, SPI: AsRef<RegisterBlock>> embedded_hal::spi::SpiBus for Spi<'a, SPI> {
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let mut read = read;
        let mut write = write;
        if read.is_empty() && write.is_empty() {
            return Ok(());
        }
        let spi = self.spi.as_ref();
        prepare_transfer(spi);
        while !read.is_empty() || !write.is_empty() {
            let count = read.len().max(write.len()).min(Self::MAX_BURST);
            let (read_chunk, read_rest) = read.split_at_mut(read.len().min(count));
            let (write_chunk, write_rest) = write.split_at(write.len().min(count));
            start_chunk(spi, write_chunk, count);
            finish_chunk(spi, read_chunk, count);
            read = read_rest;
            write = write_rest;
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        let spi = self.spi.as_ref();
        prepare_transfer(spi);
        for chunk in words.chunks_mut(Self::MAX_BURST) {
            let count = chunk.len();
            // Stage the entire original chunk in TX FIFO before overwriting it.
            start_chunk(spi, chunk, count);
            finish_chunk(spi, chunk, count);
        }
        Ok(())
    }

    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.transfer(words, &[])
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.transfer(&mut [], words)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let spi = self.spi.as_ref();
        while !spi.tcr.read().burst_finished() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

/// Reset both FIFOs before a nonempty bus operation, while the bus is idle.
fn prepare_transfer(spi: &RegisterBlock) {
    while !spi.tcr.read().burst_finished() {
        core::hint::spin_loop();
    }
    // SAFETY: this driver owns the idle controller and no previous operation
    // has unread data. FCR reset requests clear themselves when complete.
    unsafe { spi.fcr.modify(|v| v.reset_fifos()) };
    while !spi.fcr.read().fifo_reset_finished() {
        core::hint::spin_loop();
    }
}

/// Start a nonempty burst fitting in both FIFOs, padding short writes.
///
/// Callers supply `write.len() <= count <= MAX_BURST` and finish each burst.
fn start_chunk(spi: &RegisterBlock, write: &[u8], count: usize) {
    // Counter and mode registers must not be changed while XCH is set.
    while !spi.tcr.read().burst_finished() {
        core::hint::spin_loop();
    }
    // SAFETY: this driver owns the idle controller. Every burst fits in the
    // FIFOs; prepare_transfer resets them and finish_chunk drains every RX byte.
    unsafe {
        spi.mbc.write(count as u32);
        spi.mtc.write(count as u32);
        spi.bcc
            .write(BurstControl::default().set_master_single_mode_transmit_counter(count as u32));
    }
    for &word in write {
        spi.txd.write_u8(word);
    }
    for _ in write.len()..count {
        spi.txd.write_u8(0xff);
    }
    // SAFETY: the counters and all TX bytes are ready, and XCH is clear.
    unsafe { spi.tcr.modify(|v| v.start_burst_exchange()) };
}

/// Finish a burst and drain all RX bytes, including bytes not requested.
///
/// Callers supply `read.len() <= count`, matching the preceding start_chunk.
fn finish_chunk(spi: &RegisterBlock, read: &mut [u8], count: usize) {
    while !spi.tcr.read().burst_finished() {
        core::hint::spin_loop();
    }
    // new() leaves DHB clear, so every transmitted byte enters RX FIFO.
    // Follow U-Boot's XCH completion check (commit 56e497eba1bd): the RX FIFO
    // count is not a reliable completion signal. The burst fits in the FIFO,
    // so XCH clearing lets us read exactly count bytes without another poll.
    for word in read.iter_mut() {
        *word = spi.rxd.read_u8();
    }
    for _ in read.len()..count {
        let _ = spi.rxd.read_u8();
    }
}
