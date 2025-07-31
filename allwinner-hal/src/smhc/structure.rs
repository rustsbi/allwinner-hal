use super::{
    ResponseMode, SdCardError, TransferMode,
    register::{
        AccessMode, BlockSize, BusWidth, CardType, Command, RegisterBlock, TransferDirection,
    },
};
use crate::ccu::{self, Clocks, SmhcClockSource};
use core::arch::asm;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct IDMACDescriptor {
    pub des0: IDMACDescriptor0,
    pub des1: IDMACDescriptor1,
    des2: u32,
    des3: u32,
}

impl IDMACDescriptor {
    #[inline]
    pub fn get_buffer_address(&self) -> u32 {
        self.des2
    }
    #[inline]
    pub fn set_buffer_address(&mut self, value: u32) {
        self.des2 = value
    }

    #[inline]
    pub fn get_next_descriptor_address(&self) -> u32 {
        self.des3
    }
    #[inline]
    pub fn set_next_descriptor_address(&mut self, value: u32) {
        self.des3 = value
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct IDMACDescriptor0(u32);

impl IDMACDescriptor0 {
    const DES_OWN_FLAG: u32 = 1u32 << 31;
    const ERR_FLAG: u32 = 1u32 << 30;
    const ED_RING: u32 = 1u32 << 5;
    const CHAIN_MOD: u32 = 1u32 << 4;
    const FIRST_FLAG: u32 = 1u32 << 3;
    const LAST_FLAG: u32 = 1u32 << 2;
    const CUR_TXRX_OVER_INT_DI: u32 = 1u32 << 1;

    #[inline]
    pub fn is_hold_enable(&self) -> bool {
        (self.0 & Self::DES_OWN_FLAG) != 0
    }
    #[inline]
    pub fn enable_hold(&mut self) {
        self.0 |= Self::DES_OWN_FLAG
    }
    #[inline]
    pub fn disable_hold(&mut self) {
        self.0 &= !Self::DES_OWN_FLAG
    }

    #[inline]
    pub fn is_error_enable(&self) -> bool {
        (self.0 & Self::ERR_FLAG) != 0
    }
    #[inline]
    pub fn enable_error(&mut self) {
        self.0 |= Self::ERR_FLAG
    }
    #[inline]
    pub fn disable_error(&mut self) {
        self.0 &= !Self::ERR_FLAG
    }

    #[inline]
    pub fn is_end_ring_enable(&self) -> bool {
        (self.0 & Self::ED_RING) != 0
    }
    #[inline]
    pub fn enable_end_ring(&mut self) {
        self.0 |= Self::ED_RING
    }
    #[inline]
    pub fn disable_end_ring(&mut self) {
        self.0 &= !Self::ED_RING;
    }

    #[inline]
    pub fn is_chain_enable(&self) -> bool {
        (self.0 & Self::CHAIN_MOD) != 0
    }
    #[inline]
    pub fn enable_chain(&mut self) {
        self.0 |= Self::CHAIN_MOD
    }
    // This flag "must be set to 1", so there is no disable function

    #[inline]
    pub fn is_first_flag_enable(&self) -> bool {
        (self.0 & Self::FIRST_FLAG) != 0
    }
    #[inline]
    pub fn enable_first_flag(&mut self) {
        self.0 |= Self::FIRST_FLAG
    }
    #[inline]
    pub fn disable_first_flag(&mut self) {
        self.0 &= !Self::FIRST_FLAG
    }

    #[inline]
    pub fn is_last_flag_enable(&self) -> bool {
        (self.0 & Self::LAST_FLAG) != 0
    }
    #[inline]
    pub fn enable_last_flag(&mut self) {
        self.0 |= Self::LAST_FLAG
    }
    #[inline]
    pub fn disable_last_flag(&mut self) {
        self.0 &= !Self::LAST_FLAG
    }

    #[inline]
    pub fn is_disable_interrupt_on_completion(&self) -> bool {
        (self.0 & Self::CUR_TXRX_OVER_INT_DI) != 0
    }
    #[inline]
    pub fn enable_disable_interrupt_on_completion(&mut self) {
        self.0 |= Self::CUR_TXRX_OVER_INT_DI
    }
    #[inline]
    pub fn disable_disable_interrupt_on_completion(&mut self) {
        self.0 &= !Self::CUR_TXRX_OVER_INT_DI
    }
}

impl Default for IDMACDescriptor0 {
    fn default() -> Self {
        let mut result = IDMACDescriptor0(0);
        result.enable_chain();
        result.enable_disable_interrupt_on_completion();
        result.enable_hold();
        result
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct IDMACDescriptor1(u32);

impl IDMACDescriptor1 {
    const BUFFER_SIZE_MASK: u32 = (1u32 << 13) - 1;
    const MAX_BUFFER_SIZE: u32 = Self::BUFFER_SIZE_MASK;

    pub fn buffer_size(&self) -> u32 {
        self.0 & Self::BUFFER_SIZE_MASK
    }

    pub fn set_buffer_size(&mut self, value: u32) {
        self.0 = value & Self::BUFFER_SIZE_MASK
    }
}

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
                    .enable_change_card_clock()
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
                    .enable_change_card_clock()
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
        unsafe {
            smhc.argument.write(arg);
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

const MAX_DMA_DES_COUNT: usize = 16;

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

        const MAX_RETRIES: u8 = 10;
        let mut attempts = 0;
        let mut success = false;

        while attempts < MAX_RETRIES {
            smhc.send_card_command(8, 0x1AA, TransferMode::Disable, ResponseMode::Short, true);
            Self::sleep(100);
            let data = smhc.read_response();
            if data == 0x1AA {
                success = true;
                break;
            }
            attempts += 1;
        }

        if !success {
            return Err(SdCardError::UnexpectedResponse(8, smhc.read_response()));
        }

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
    pub fn read_block(&self, blocks: &mut [Block], start_block_idx: u32) {
        log::trace!(
            "read block from {}, length = {}",
            start_block_idx,
            blocks.len()
        );
        let length = blocks.len() as u32;
        if length == 0 {
            panic!("Invalid read block length = 0");
        }
        loop {
            let mut dma_desc: [IDMACDescriptor; MAX_DMA_DES_COUNT] =
                [Default::default(); MAX_DMA_DES_COUNT];
            let smhc = self.smhc.smhc.as_ref();
            unsafe {
                smhc.global_control
                    .modify(|val| val.set_dma_reset().set_fifo_reset().enable_dma());
                while !smhc.global_control.read().is_dma_reset_cleared() {
                    core::hint::spin_loop();
                }
                while !smhc.global_control.read().is_fifo_reset_cleared() {
                    core::hint::spin_loop();
                }
                smhc.dma_interrupt_enable.modify(|val| {
                    val.enable_rx_int()
                        .enable_card_err_sum_int()
                        .enable_des_unavl_int()
                        .enable_fatal_berr_int()
                        .enable_tx_int()
                });
                smhc.dma_control
                    .modify(|val| val.enable_dma().enable_fix_burst_size());
                smhc.fifo_water_level.modify(|val| {
                    use super::register::BurstSize;
                    val.set_burst_size(BurstSize::SixteenBit)
                        .set_receive_trigger_level(15)
                        .set_transmit_trigger_level(240)
                });
                smhc.byte_count.write(Block::LEN_U32 * length);
                smhc.dma_descriptor_base
                    .modify(|_| (core::ptr::addr_of!(dma_desc[0]) as u32) >> 2);
            }
            for i in 0..blocks.len() {
                dma_desc[i].des1.set_buffer_size(Block::LEN_U32);
                dma_desc[i]
                    .set_buffer_address((core::ptr::addr_of!(blocks[i].contents) as u32) >> 2);
                // TODO
                dma_desc[i].set_next_descriptor_address(
                    (core::ptr::addr_of!(dma_desc[i + 1]) as u32) >> 2,
                );
            }
            dma_desc[0].des0.enable_first_flag();
            dma_desc[blocks.len() - 1].des0.enable_last_flag();
            dma_desc[blocks.len() - 1].des0.enable_end_ring();
            dma_desc[blocks.len() - 1].set_next_descriptor_address(0);
            dma_desc[blocks.len() - 1]
                .des0
                .disable_disable_interrupt_on_completion();
            unsafe {
                asm!("fence");
            };
            if length == 1 {
                self.smhc.send_card_command(
                    17,
                    start_block_idx,
                    TransferMode::Read,
                    ResponseMode::Short,
                    true,
                );
            } else {
                self.smhc.send_card_command(
                    18,
                    start_block_idx,
                    TransferMode::Read,
                    ResponseMode::Short,
                    true,
                );
            }
            // for block in &mut *blocks {
            //     self.smhc.read_data(&mut block.contents);
            // }
            const MAX_RETRY_TIME: u32 = 16;
            for i in 0..MAX_RETRY_TIME {
                if i != 0 {
                    log::debug!("SD read retry for command complete: {}", i);
                }
                let status = self.smhc.smhc.as_ref().interrupt_state_raw.read();
                if status.has_interrupt(Interrupt::CommandComplete) {
                    break;
                }
                Self::sleep(100);
            }
            for i in 0..MAX_RETRY_TIME {
                if i != 0 {
                    log::debug!("SD read retry for DMA Read Complete: {}", i);
                }
                let status = smhc.dma_state.read();
                if status.rx_int_occurs() {
                    break;
                }
                Self::sleep(100);
            }
            // Reset DMA State
            unsafe {
                let status = smhc.dma_state.read();
                smhc.dma_state.write(status);
            }
            use super::register::Interrupt;
            let status = smhc.interrupt_state_raw.read();
            unsafe {
                smhc.interrupt_state_raw.write(status);
            }
            if length == 1 {
                if status.has_interrupt(Interrupt::DataTransferComplete) {
                    break;
                }
            } else {
                if status.has_interrupt(Interrupt::DataTransferComplete)
                    & status.has_interrupt(Interrupt::AutoCommandDone)
                {
                    break;
                }
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
    fn read(&self, blocks: &mut [Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        let mut less_blocks = blocks;
        let mut current_idx = start_block_idx.0;
        while less_blocks.len() >= MAX_DMA_DES_COUNT {
            let result = less_blocks.split_at_mut(MAX_DMA_DES_COUNT);
            less_blocks = result.1;
            self.read_block(result.0, current_idx);
            current_idx += MAX_DMA_DES_COUNT as u32;
        }
        if less_blocks.len() > 0 {
            self.read_block(less_blocks, current_idx);
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
