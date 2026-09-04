//! Register values shared by multiple Allwinner CCU layouts.

/// A combined bus clock-gating and reset register.
///
/// The low half-word contains `COUNT` clock-gate bits and the high half-word
/// contains matching active-low reset bits. This layout is used by grouped
/// peripheral registers such as UART, SPI, and SMHC on multiple CCU platforms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BusGatingReset<const COUNT: usize = 16>(pub(crate) u32);

impl<const COUNT: usize> BusGatingReset<COUNT> {
    const fn mask<const I: usize>() -> u32 {
        assert!(COUNT <= 16);
        assert!(I < COUNT);
        1 << I
    }

    /// Disable the clock gate for peripheral `I`.
    #[inline]
    pub const fn gate_mask<const I: usize>(self) -> Self {
        Self(self.0 & !Self::mask::<I>())
    }

    /// Enable the clock gate for peripheral `I`.
    #[inline]
    pub const fn gate_pass<const I: usize>(self) -> Self {
        Self(self.0 | Self::mask::<I>())
    }

    /// Assert the active-low reset signal for peripheral `I`.
    #[inline]
    pub const fn assert_reset<const I: usize>(self) -> Self {
        Self(self.0 & !(Self::mask::<I>() << 16))
    }

    /// Deassert the active-low reset signal for peripheral `I`.
    #[inline]
    pub const fn deassert_reset<const I: usize>(self) -> Self {
        Self(self.0 | (Self::mask::<I>() << 16))
    }
}

/// A single-peripheral bus clock-gating and reset register.
///
/// Bit 0 controls the clock gate and bit 16 controls the active-low reset.
/// Some chip-specific GAR/BGR registers contain additional fields; this type
/// models only the gate/reset pair common to those registers and preserves all
/// other bits when one of its methods is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SingleBusGatingReset(pub(crate) u32);

impl SingleBusGatingReset {
    const GATE: u32 = 1 << 0;
    const RESET: u32 = 1 << 16;

    /// Disable the peripheral clock gate.
    #[inline]
    pub const fn gate_mask(self) -> Self {
        Self(self.0 & !Self::GATE)
    }

    /// Enable the peripheral clock gate.
    #[inline]
    pub const fn gate_pass(self) -> Self {
        Self(self.0 | Self::GATE)
    }

    /// Assert the active-low peripheral reset signal.
    #[inline]
    pub const fn assert_reset(self) -> Self {
        Self(self.0 & !Self::RESET)
    }

    /// Deassert the active-low peripheral reset signal.
    #[inline]
    pub const fn deassert_reset(self) -> Self {
        Self(self.0 | Self::RESET)
    }
}

#[cfg(test)]
mod tests {
    use super::{BusGatingReset, SingleBusGatingReset};

    #[test]
    fn grouped_gate_and_reset_fields() {
        let value = BusGatingReset::<16>(0)
            .gate_pass::<3>()
            .deassert_reset::<3>();
        assert_eq!(value.0, 0x0008_0008);
        assert_eq!(value.gate_mask::<3>().assert_reset::<3>().0, 0);
    }

    #[test]
    fn single_gate_and_reset_fields() {
        let value = SingleBusGatingReset(0).gate_pass().deassert_reset();
        assert_eq!(value.0, 0x0001_0001);
        assert_eq!(value.gate_mask().assert_reset().0, 0);
    }
}
