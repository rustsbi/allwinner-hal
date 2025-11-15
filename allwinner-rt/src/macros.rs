macro_rules! soc {
    (
        $(
            $(#[$doc:meta])*
            pub struct $Ty:ident => $paddr:expr_2021, $DerefTy:ty;
        )+
    ) => {
        $(
            $(#[$doc])*
            #[allow(non_camel_case_types)]
            pub struct $Ty {
                _private: (),
            }
            impl $Ty {
                #[inline]
                pub const fn ptr() -> *const $DerefTy {
                    $paddr as *const _
                }
            }

            impl core::ops::Deref for $Ty {
                type Target = $DerefTy;
                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    unsafe { &*($paddr as *const _) }
                }
            }
            impl core::convert::AsRef<$DerefTy> for $Ty {
                #[inline(always)]
                fn as_ref(&self) -> &$DerefTy {
                    unsafe { &*($paddr as *const _) }
                }
            }
        )+
    };
}

macro_rules! impl_gpio_pins {
    (
        $(
        $px: ident: ($P: expr, $N: expr);
        )+
    ) => {
/// GPIO pads in the current platform.
pub struct Pads {
    $(
    pub $px: Pad<$P, $N>,
    )+
}

impl Pads {
    #[doc(hidden)]
    #[inline]
    pub fn __new() -> Self {
        Self {
            $(
            $px: Pad::__new(),
            )+
        }
    }
}
    };
}

macro_rules! impl_uart {
    ($($i:expr => $UARTi:ident,)+) => {
        $(
            impl allwinner_hal::uart::Instance<'static> for $UARTi {
                #[inline]
                fn register_block(self) -> &'static allwinner_hal::uart::RegisterBlock {
                    unsafe { &*Self::ptr() }
                }
            }

            impl<'a> allwinner_hal::uart::Instance<'a> for &'a mut $UARTi {
                #[inline]
                fn register_block(self) -> &'a allwinner_hal::uart::RegisterBlock {
                    &*self
                }
            }

            impl<'a> UartExt<'a, $i> for &'a mut $UARTi {
                fn serial(
                    self,
                    pads: impl allwinner_hal::uart::Pads<'a, $i>,
                    config: impl Into<allwinner_hal::uart::Config>,
                    clock: impl allwinner_hal::uart::Clock,
                ) -> allwinner_hal::uart::BlockingSerial<'a> {
                    allwinner_hal::uart::BlockingSerial::new(self, pads, config, clock)
                }
            }

            impl UartExt<'static, $i> for $UARTi {
                fn serial(
                    self,
                    pads: impl allwinner_hal::uart::Pads<'static, $i>,
                    config: impl Into<allwinner_hal::uart::Config>,
                    clock: impl allwinner_hal::uart::Clock,
                ) -> allwinner_hal::uart::BlockingSerial<'static> {
                    allwinner_hal::uart::BlockingSerial::new(self, pads, config, clock)
                }
            }

        )+
    };
}

macro_rules! impl_uart_pads {
    ($(($p: expr, $i: expr, $f: expr): $Trait: ident, $into_func: ident, $UARTi: expr;)+) => {
        $(
impl allwinner_hal::uart::$Trait<'static, $UARTi> for Pad<$p, $i> {
    #[inline]
    fn $into_func(self) -> allwinner_hal::gpio::FlexPad<'static> {
        self.into_function::<$f>().into()
    }
}

impl<'a> allwinner_hal::uart::$Trait<'a, $UARTi> for &'a mut Pad<$p, $i> {
    #[inline]
    fn $into_func(self) -> allwinner_hal::gpio::FlexPad<'a> {
        self.into_function::<$f>().into()
    }
}
        )+
    };
}

macro_rules! impl_spi_pads {
    ($(($p: expr, $i: expr, $f: expr): $Trait: ident, $into_func: ident, $UARTi: expr;)+) => {
        $(
impl allwinner_hal::spi::$Trait<'static, $UARTi> for Pad<$p, $i> {
    #[inline]
    fn $into_func(self) -> allwinner_hal::gpio::FlexPad<'static> {
        self.into_function::<$f>().into()
    }
}

impl<'a> allwinner_hal::spi::$Trait<'a, $UARTi> for &'a mut Pad<$p, $i> {
    #[inline]
    fn $into_func(self) -> allwinner_hal::gpio::FlexPad<'a> {
        self.into_function::<$f>().into()
    }
}
        )+
    };
}
