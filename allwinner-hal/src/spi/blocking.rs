use super::{
    Pads,
    register::{GlobalControl, RegisterBlock, TransferControl},
};
use crate::gpio::FlexPad;
use embedded_hal::spi::Mode;

/// Managed SPI structure with peripheral and pins.
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
    /// Create an SPI instance.
    pub fn new<const I: usize>(
        spi: SPI,
        pads: impl Pads<'a, I>,
        mode: impl Into<Mode>,
        // freq: Hertz,
        // clock: impl Clock,
        // ccu: &ccu::RegisterBlock,
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
        assert!(read.len() + write.len() <= u32::MAX as usize);
        let spi = self.spi.as_ref();
        unsafe { spi.mbc.write((read.len() + write.len()) as u32) };
        unsafe { spi.mtc.write(write.len() as u32) };
        let bcc = spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(write.len() as u32);
        unsafe { spi.bcc.write(bcc) };
        unsafe { spi.tcr.write(spi.tcr.read().start_burst_exchange()) };
        for &word in write {
            while spi.fsr.read().transmit_fifo_counter() > 63 {
                core::hint::spin_loop();
            }
            spi.txd.write_u8(word)
        }
        for word in read {
            while spi.fsr.read().receive_fifo_counter() == 0 {
                core::hint::spin_loop();
            }
            *word = spi.rxd.read_u8()
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        assert!(words.len() * 2 <= u32::MAX as usize);
        let spi = self.spi.as_ref();
        unsafe { spi.mbc.write((words.len() * 2) as u32) };
        unsafe { spi.mtc.write(words.len() as u32) };
        let bcc = spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(words.len() as u32);
        unsafe { spi.bcc.write(bcc) };
        unsafe { spi.tcr.write(spi.tcr.read().start_burst_exchange()) };
        for &word in words.iter() {
            while spi.fsr.read().transmit_fifo_counter() > 63 {
                core::hint::spin_loop();
            }
            spi.txd.write_u8(word)
        }
        for word in words {
            while spi.fsr.read().receive_fifo_counter() == 0 {
                core::hint::spin_loop();
            }
            *word = spi.rxd.read_u8()
        }
        Ok(())
    }

    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        assert!(words.len() <= u32::MAX as usize);
        let spi = self.spi.as_ref();
        unsafe { spi.mbc.write(words.len() as u32) };
        unsafe { spi.mtc.write(0) };
        let bcc = spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(0);
        unsafe { spi.bcc.write(bcc) };
        unsafe { spi.tcr.write(spi.tcr.read().start_burst_exchange()) };
        for word in words {
            while spi.fsr.read().receive_fifo_counter() == 0 {
                core::hint::spin_loop();
            }
            *word = spi.rxd.read_u8()
        }
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        assert!(words.len() <= u32::MAX as usize);
        let spi = self.spi.as_ref();
        unsafe { spi.mbc.write(words.len() as u32) };
        unsafe { spi.mtc.write(words.len() as u32) };
        let bcc = spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(words.len() as u32);
        unsafe { spi.bcc.write(bcc) };
        unsafe { spi.tcr.write(spi.tcr.read().start_burst_exchange()) };
        for &word in words {
            while spi.fsr.read().transmit_fifo_counter() > 63 {
                core::hint::spin_loop();
            }
            spi.txd.write_u8(word)
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let spi = self.spi.as_ref();
        while !spi.tcr.read().burst_finished() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}
