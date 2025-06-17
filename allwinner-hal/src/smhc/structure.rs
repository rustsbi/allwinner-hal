use super::{
    register::{
        AccessMode, BlockSize, BusWidth, CardType, Command, RegisterBlock, TransferDirection,
    },
    ResponseMode, SdCardError, TransferMode,
};
use crate::ccu::{self, Clocks, SmhcClockSource};
use core::arch::asm;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};

/// Managed SMHC structure with peripheral and pins.
pub struct Smhc<SMHC, PADS> {
    smhc: SMHC,
    pads: PADS,
}

impl<SMHC: AsRef<RegisterBlock>, PADS> Smhc<SMHC, PADS> {
    /// Create an SMHC instance.
    #[inline]
    pub fn new<const SMHC_IDX: usize>(
        smhc: SMHC,
        pads: PADS,
        clocks: &Clocks,
        ccu: &ccu::RegisterBlock,
    ) -> Self {
        let divider = 2;
        let (factor_n, factor_m) =
            ccu::calculate_best_peripheral_factors_nm(clocks.psi.0, 20_000_000);
        unsafe {
            smhc.as_ref()
                .clock_control
                .modify(|val| val.disable_card_clock());
        }
        unsafe {
            ccu.smhc_bgr.modify(|val| val.assert_reset::<SMHC_IDX>());
            ccu.smhc_bgr.modify(|val| val.gate_mask::<SMHC_IDX>());
            ccu.smhc_clk[SMHC_IDX].modify(|val| {
                val.set_clock_source(SmhcClockSource::PllPeri1x)
                    .set_factor_n(factor_n)
                    .set_factor_m(factor_m)
                    .enable_clock_gating()
            });
            ccu.smhc_bgr.modify(|val| val.deassert_reset::<SMHC_IDX>());
            ccu.smhc_bgr.modify(|val| val.gate_pass::<SMHC_IDX>());
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.global_control.modify(|val| val.set_software_reset());
            while !smhc.global_control.read().is_software_reset_cleared() {
                core::hint::spin_loop();
            }
            smhc.global_control.modify(|val| val.set_fifo_reset());
            while !smhc.global_control.read().is_fifo_reset_cleared() {
                core::hint::spin_loop();
            }
            smhc.global_control.modify(|val| val.disable_interrupt());
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.command.modify(|val| {
                val.enable_wait_for_complete()
                    .enable_change_clock()
                    .set_command_start()
            });
            while !smhc.command.read().is_command_start_cleared() {
                core::hint::spin_loop();
            }
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.clock_control
                .modify(|val| val.set_card_clock_divider(divider - 1));
            smhc.sample_delay_control.modify(|val| {
                val.set_sample_delay_software(0)
                    .enable_sample_delay_software()
            });
            smhc.clock_control.modify(|val| val.enable_card_clock());
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.command.modify(|val| {
                val.enable_wait_for_complete()
                    .enable_change_clock()
                    .set_command_start()
            });
            while !smhc.command.read().is_command_start_cleared() {
                core::hint::spin_loop();
            }
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.card_type
                .write(CardType::default().set_bus_width(BusWidth::OneBit));
            smhc.block_size
                .write(BlockSize::default().set_block_size(512)); // TODO
        }

        Self { smhc, pads }
    }
    /// Get a temporary borrow on the underlying GPIO pads.
    #[inline]
    pub fn pads<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut PADS) -> T,
    {
        f(&mut self.pads)
    }
    /// Close SMHC and release peripheral.
    #[inline]
    pub fn free(self, ccu: &ccu::RegisterBlock) -> (SMHC, PADS) {
        unsafe {
            const SMHC_IDX: usize = 0; // TODO
            ccu.smhc_bgr.modify(|val| val.assert_reset::<SMHC_IDX>());
            ccu.smhc_bgr.modify(|val| val.gate_mask::<SMHC_IDX>());
        }
        (self.smhc, self.pads)
    }
    /// Send a command to the card.
    #[inline]
    pub fn send_card_command(
        &self,
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
        let smhc = self.smhc.as_ref();
        if data_trans {
            unsafe {
                smhc.byte_count.modify(|w| w.set_byte_count(512)); // TODO
                smhc.global_control
                    .modify(|w| w.set_access_mode(AccessMode::Ahb));
            }
        }
        unsafe {
            smhc.argument.modify(|val| val.set_argument(arg));
            smhc.command.write({
                let mut val = Command::default()
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
    /// Read the response from the card.
    #[inline]
    pub fn read_response(&self) -> u128 {
        let smhc = self.smhc.as_ref();
        let mut response = 0u128;
        for i in 0..4 {
            response |= (smhc.responses[i].read() as u128) << (32 * i);
        }
        response
    }
    /// Read data from first-in-first-out buffer.
    #[inline]
    pub fn read_data(&self, buf: &mut [u8]) {
        let smhc = self.smhc.as_ref();
        for i in 0..buf.len() / 4 {
            while smhc.status.read().fifo_empty() {
                core::hint::spin_loop();
            }
            let data = smhc.fifo.read();
            buf[i * 4] = (data & 0xff) as u8;
            buf[i * 4 + 1] = ((data >> 8) & 0xff) as u8;
            buf[i * 4 + 2] = ((data >> 16) & 0xff) as u8;
            buf[i * 4 + 3] = ((data >> 24) & 0xff) as u8;
        }
    }
}

pub struct SdCard<'a, S, P> {
    smhc: &'a mut Smhc<S, P>,
    block_count: u32,
}

impl<'a, S: AsRef<RegisterBlock>, P> SdCard<'a, S, P> {
    /// Create an SD card instance.
    #[inline]
    pub fn new(smhc: &'a mut Smhc<S, P>) -> Result<Self, SdCardError> {
        /// Host supports high capacity
        const OCR_HCS: u32 = 0x40000000;
        /// Card has finished power up routine if bit is high
        const OCR_NBUSY: u32 = 0x80000000;
        /// Valid bits for voltage setting
        const OCR_VOLTAGE_MASK: u32 = 0x007FFF80;

        // CMD0(reset) -> CMD8(check voltage and sdcard version)
        // -> CMD55+ACMD41(init and read OCR)
        smhc.send_card_command(0, 0, TransferMode::Disable, ResponseMode::Disable, false);
        Self::sleep(100); // TODO: wait for interrupt instead of sleep
        smhc.send_card_command(8, 0x1AA, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);
        let data = smhc.read_response();
        if data != 0x1AA {
            return Err(SdCardError::UnexpectedResponse(8, data));
        }
        loop {
            smhc.send_card_command(55, 0, TransferMode::Disable, ResponseMode::Short, true);
            Self::sleep(100);
            smhc.send_card_command(
                41,
                OCR_VOLTAGE_MASK & 0x00ff8000 | OCR_HCS,
                TransferMode::Disable,
                ResponseMode::Short,
                false,
            );
            Self::sleep(100);
            let ocr = smhc.read_response() as u32;
            if (ocr & OCR_NBUSY) == OCR_NBUSY {
                break;
            }
        }

        // Send CMD2 to get CID.
        smhc.send_card_command(2, 0, TransferMode::Disable, ResponseMode::Long, true);
        Self::sleep(100);
        let _cid = smhc.read_response();

        // Send CMD3 to get RCA.
        smhc.send_card_command(3, 0, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);
        let rca = smhc.read_response() as u32;

        // Send CMD9 to get CSD.
        smhc.send_card_command(9, rca, TransferMode::Disable, ResponseMode::Long, true);
        Self::sleep(100);
        let csd_raw = smhc.read_response();
        let fixed_csd_raw = csd_raw >> 8; // FIXME: 8bit shift for long response, why?
        let (csd_structure, c_size) = Self::parse_csd_v2(fixed_csd_raw);
        if csd_structure != 1 {
            return Err(SdCardError::UnexpectedResponse(9, csd_raw));
        }

        // Send CMD7 to select card.
        smhc.send_card_command(7, rca, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);

        // Set 1 data len, CMD55 -> ACMD6.
        smhc.send_card_command(55, rca, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);
        smhc.send_card_command(6, 0, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);

        Ok(SdCard {
            smhc,
            block_count: (c_size + 1) * 1024,
        })
    }
    /// Get the size of the SD card in kilobytes.
    #[inline]
    pub fn get_size_kb(&self) -> f64 {
        (self.block_count as f64) * (512 as f64) / 1024.0
    }
    /// Read a block from the SD card.
    #[inline]
    pub fn read_block(&self, block: &mut Block, block_idx: u32) {
        loop {
            let smhc = self.smhc.smhc.as_ref();
            unsafe {
                smhc.global_control.modify(|val| val.set_fifo_reset());
                while !smhc.global_control.read().is_fifo_reset_cleared() {
                    core::hint::spin_loop();
                }
                smhc.global_control
                    .modify(|val| val.set_access_mode(AccessMode::Ahb));
                self.smhc.smhc.as_ref().fifo_water_level.modify(|val| {
                    use super::register::BurstSize;
                    val.set_burst_size(BurstSize::SixteenBit)
                        .set_receive_trigger_level(15)
                        .set_transmit_trigger_level(240)
                });
            }
            self.smhc.send_card_command(
                17,
                block_idx,
                TransferMode::Read,
                ResponseMode::Short,
                true,
            );
            self.smhc.read_data(&mut block.contents);
            loop {
                use super::register::Interrupt;
                let status = self.smhc.smhc.as_ref().interrupt_state_raw.read();
                if status.has_interrupt(Interrupt::CommandComplete) {
                    break;
                }
                Self::sleep(100);
            }
            use super::register::Interrupt;
            let status = self.smhc.smhc.as_ref().interrupt_state_raw.read();
            if status.has_interrupt(Interrupt::DataTransferComplete) {
                break;
            }
        }
    }
    /// Parse CSD register version 2.
    #[inline]
    fn parse_csd_v2(csd: u128) -> (u32, u32) {
        let csd_structure = (((csd >> (32 * 3)) & 0xC00000) >> 22) as u32;
        let c_size = (((csd >> 32) & 0x3FFFFF00) >> 8) as u32;
        (csd_structure, c_size)
    }
    /// Sleep for a number of cycles.
    #[inline]
    fn sleep(n: u32) {
        for _ in 0..n * 100_000 {
            unsafe { asm!("nop") }
        }
    }
}

impl<'a, S: AsRef<RegisterBlock>, P> BlockDevice for SdCard<'a, S, P> {
    type Error = core::convert::Infallible;

    #[inline]
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            self.read_block(block, start_block_idx.0 + i as u32);
        }
        Ok(())
    }

    #[inline]
    fn write(&self, _blocks: &[Block], _start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        todo!();
    }

    #[inline]
    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        Ok(embedded_sdmmc::BlockCount(self.block_count))
    }
}
