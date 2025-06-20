//! V853, V851s, V851se chip platforms.

soc! {
    /// General Purpose Input/Output peripheral for PA, PC, PD and PL pads.
    pub struct GPIO => 0x4004A400, allwinner_hal::gpio::RegisterBlock;
}

/// Ownership of a V853 GPIO pad.
pub struct Pad<const P: char, const N: u8> {
    _private: (),
}

impl<const P: char, const N: u8> Pad<P, N> {
    /// Macro internal constructor.
    #[doc(hidden)]
    #[inline]
    pub const fn __new() -> Self {
        Self { _private: () }
    }
}

impl_gpio_pins! {
    pa0: ('A', 0);
    pa1: ('A', 1);
    pa2: ('A', 2);
    pa3: ('A', 3);
    pa4: ('A', 4);
    pa5: ('A', 5);
    pa6: ('A', 6);
    pa7: ('A', 7);
    pa8: ('A', 8);
    pa9: ('A', 9);
    pa10: ('A', 10);
    pa11: ('A', 11);
    pa12: ('A', 12);
    pa13: ('A', 13);
    pa14: ('A', 14);
    pa15: ('A', 15);
    pa16: ('A', 16);
    pa17: ('A', 17);
    pa18: ('A', 18);
    pa19: ('A', 19);
    pa20: ('A', 20);
    pa21: ('A', 21);
    pa22: ('A', 22);
    pa23: ('A', 23);
    pa24: ('A', 24);
    pa25: ('A', 25);
    pa26: ('A', 26);
    pa27: ('A', 27);
    pa28: ('A', 28);
    pa29: ('A', 29);

    pb0: ('B', 0);
    pb1: ('B', 1);
    pb2: ('B', 2);
    pb3: ('B', 3);
    pb4: ('B', 4);
    pb5: ('B', 5);
    pb6: ('B', 6);
    pb7: ('B', 7);
    pb8: ('B', 8);
    pb9: ('B', 9);
    pb10: ('B', 10);
    pb11: ('B', 11);
    pb12: ('B', 12);
    pb13: ('B', 13);
    pb14: ('B', 14);
    pb15: ('B', 15);

    pc0: ('C', 0);
    pc1: ('C', 1);
    pc2: ('C', 2);
    pc3: ('C', 3);
    pc4: ('C', 4);
    pc5: ('C', 5);
    pc6: ('C', 6);
    pc7: ('C', 7);
    pc8: ('C', 8);
    pc9: ('C', 9);
    pc10: ('C', 10);
    pc11: ('C', 11);
    pc12: ('C', 12);
}
