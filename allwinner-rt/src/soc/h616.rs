//! H313, H616 and H618 chip platforms.

soc! {
    /// General Purpose Input/Output peripheral for the main GPIO domain.
    pub struct GPIO => 0x0300_B000, allwinner_hal::gpio::v1::RegisterBlockV1;
}

/// Ownership of an H313, H616 or H618 GPIO pad.
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
    __new_v1;
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
    pd24: ('D', 24);
    pd25: ('D', 25);
    pd26: ('D', 26);
    pd27: ('D', 27);
    pd28: ('D', 28);
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
    pe16: ('E', 16);
    pe17: ('E', 17);
    pe18: ('E', 18);
    pe19: ('E', 19);
    pe20: ('E', 20);
    pe21: ('E', 21);
    pe22: ('E', 22);
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
    pg15: ('G', 15);
    pg16: ('G', 16);
    pg17: ('G', 17);
    pg18: ('G', 18);
    pg19: ('G', 19);
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
}
