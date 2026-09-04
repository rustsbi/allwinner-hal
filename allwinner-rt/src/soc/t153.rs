//! T153 chip platform.

soc! {
    /// Clock Control Unit peripheral.
    pub struct CCU => 0x0200_2000, allwinner_hal::ccu::t153::RegisterBlock;
    /// General Purpose Input/Output peripheral for the main GPIO domain.
    pub struct GPIO => 0x0360_4000, allwinner_hal::gpio::v3::RegisterBlockV3;
}

/// Ownership of a T153 GPIO pad.
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
    __new_v3;
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
    pd0: ('D', 0);
    pd1: ('D', 1);
    pd2: ('D', 2);
    pd3: ('D', 3);
    pd4: ('D', 4);
    pd5: ('D', 5);
    pd6: ('D', 6);
    pd7: ('D', 7);
    pd8: ('D', 8);
    pd9: ('D', 9);
    pd10: ('D', 10);
    pd11: ('D', 11);
    pd12: ('D', 12);
    pd13: ('D', 13);
    pd14: ('D', 14);
    pd15: ('D', 15);
    pd16: ('D', 16);
    pd17: ('D', 17);
    pd18: ('D', 18);
    pd19: ('D', 19);
    pd20: ('D', 20);
    pd21: ('D', 21);
    pd22: ('D', 22);
    pd23: ('D', 23);
    pd29: ('D', 29);
    pd30: ('D', 30);
    pd31: ('D', 31);
    pe0: ('E', 0);
    pe1: ('E', 1);
    pe2: ('E', 2);
    pe3: ('E', 3);
    pe4: ('E', 4);
    pe5: ('E', 5);
    pe6: ('E', 6);
    pe7: ('E', 7);
    pe8: ('E', 8);
    pe9: ('E', 9);
    pf0: ('F', 0);
    pf1: ('F', 1);
    pf2: ('F', 2);
    pf3: ('F', 3);
    pf4: ('F', 4);
    pf5: ('F', 5);
    pf6: ('F', 6);
    pf8: ('F', 8);
    pf9: ('F', 9);
    pf10: ('F', 10);
    pf11: ('F', 11);
    pf14: ('F', 14);
    pf17: ('F', 17);
    pf19: ('F', 19);
    pf20: ('F', 20);
    pf21: ('F', 21);
    pf22: ('F', 22);
    pf23: ('F', 23);
    pf24: ('F', 24);
    pf25: ('F', 25);
    pf26: ('F', 26);
    pf27: ('F', 27);
    pg0: ('G', 0);
    pg1: ('G', 1);
    pg2: ('G', 2);
    pg3: ('G', 3);
    pg4: ('G', 4);
    pg5: ('G', 5);
    pg6: ('G', 6);
    pg7: ('G', 7);
    pg8: ('G', 8);
    pg9: ('G', 9);
    pg10: ('G', 10);
    pg11: ('G', 11);
    pg12: ('G', 12);
    pg13: ('G', 13);
    pg14: ('G', 14);
    pg15: ('G', 15);
    pj0: ('J', 0);
    pj1: ('J', 1);
    pj2: ('J', 2);
    pj3: ('J', 3);
    pj4: ('J', 4);
    pj5: ('J', 5);
    pj6: ('J', 6);
    pj7: ('J', 7);
    pj8: ('J', 8);
    pj9: ('J', 9);
    pj10: ('J', 10);
    pj11: ('J', 11);
    pj12: ('J', 12);
    pj13: ('J', 13);
    pj14: ('J', 14);
    pj15: ('J', 15);
    pk0: ('K', 0);
    pk1: ('K', 1);
    pk2: ('K', 2);
    pk3: ('K', 3);
    pk4: ('K', 4);
    pk5: ('K', 5);
    pk6: ('K', 6);
    pk7: ('K', 7);
    pk8: ('K', 8);
    pk9: ('K', 9);
    pk10: ('K', 10);
    pk11: ('K', 11);
}
