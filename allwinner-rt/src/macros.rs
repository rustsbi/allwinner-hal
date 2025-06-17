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
    ($($px: ident:($P: expr, $N: expr);)+) => {
/// GPIO pads in current platform.
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
                fn serial<PADS>(
                    self,
                    pads: PADS,
                    config: impl Into<allwinner_hal::uart::Config>,
                    clocks: &Clocks,
                    ccu: &allwinner_hal::ccu::RegisterBlock,
                ) -> allwinner_hal::uart::Serial<'a, PADS>
                where
                    PADS: allwinner_hal::uart::Pads<$i>,
                {
                    allwinner_hal::uart::Serial::new(self, pads, config, clocks, ccu)
                }
            }

            impl UartExt<'static, $i> for $UARTi {
                fn serial<PADS>(
                    self,
                    pads: PADS,
                    config: impl Into<allwinner_hal::uart::Config>,
                    clocks: &Clocks,
                    ccu: &allwinner_hal::ccu::RegisterBlock,
                ) -> allwinner_hal::uart::Serial<'static, PADS>
                where
                    PADS: allwinner_hal::uart::Pads<$i>,
                {
                    allwinner_hal::uart::Serial::new(self, pads, config, clocks, ccu)
                }
            }

        )+
    };
}
