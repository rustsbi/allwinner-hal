//! A733 and T736 chip platforms.

soc! {
    /// Clock Control Unit peripheral.
    pub struct CCU => 0x0200_2000, allwinner_hal::ccu::a733::RegisterBlock;
    /// General Purpose Input/Output peripheral for the main GPIO domain.
    pub struct GPIO => 0x0200_0000, allwinner_hal::gpio::v3::RegisterBlockV3;
}

/// Ownership of an A733 or T736 GPIO pad.
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
    pc13: ('C', 13);
    pc14: ('C', 14);
    pc15: ('C', 15);
    pc16: ('C', 16);
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
    pe10: ('E', 10);
    pe11: ('E', 11);
    pe12: ('E', 12);
    pe13: ('E', 13);
    pe14: ('E', 14);
    pe15: ('E', 15);
    pf0: ('F', 0);
    pf1: ('F', 1);
    pf2: ('F', 2);
    pf3: ('F', 3);
    pf4: ('F', 4);
    pf5: ('F', 5);
    pf6: ('F', 6);
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
    ph0: ('H', 0);
    ph1: ('H', 1);
    ph2: ('H', 2);
    ph3: ('H', 3);
    ph4: ('H', 4);
    ph5: ('H', 5);
    ph6: ('H', 6);
    ph7: ('H', 7);
    ph8: ('H', 8);
    ph9: ('H', 9);
    ph10: ('H', 10);
    ph11: ('H', 11);
    ph12: ('H', 12);
    ph13: ('H', 13);
    ph14: ('H', 14);
    ph15: ('H', 15);
    ph16: ('H', 16);
    ph17: ('H', 17);
    ph18: ('H', 18);
    ph19: ('H', 19);
    pi0: ('I', 0);
    pi1: ('I', 1);
    pi2: ('I', 2);
    pi3: ('I', 3);
    pi4: ('I', 4);
    pi5: ('I', 5);
    pi6: ('I', 6);
    pi7: ('I', 7);
    pi8: ('I', 8);
    pi9: ('I', 9);
    pi10: ('I', 10);
    pi11: ('I', 11);
    pi12: ('I', 12);
    pi13: ('I', 13);
    pi14: ('I', 14);
    pi15: ('I', 15);
    pi16: ('I', 16);
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
    pj16: ('J', 16);
    pj17: ('J', 17);
    pj18: ('J', 18);
    pj19: ('J', 19);
    pj20: ('J', 20);
    pj21: ('J', 21);
    pj22: ('J', 22);
    pj23: ('J', 23);
    pj24: ('J', 24);
    pj25: ('J', 25);
    pj26: ('J', 26);
    pj27: ('J', 27);
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
    pk12: ('K', 12);
    pk13: ('K', 13);
    pk14: ('K', 14);
    pk15: ('K', 15);
    pk16: ('K', 16);
    pk17: ('K', 17);
    pk18: ('K', 18);
    pk19: ('K', 19);
    pk20: ('K', 20);
    pk21: ('K', 21);
    pk22: ('K', 22);
    pk23: ('K', 23);
    pk24: ('K', 24);
    pk25: ('K', 25);
}
