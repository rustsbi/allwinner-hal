macro_rules! impl_gpio_constructors {
    (@one pad, $constructor:ident, $register:ty, $version:ident) => {
        // Macro internal function for ROM runtime; DO NOT USE.
        #[doc(hidden)]
        #[inline]
        pub unsafe fn $constructor(port: char, number: u8, gpio: &'a $register) -> Self {
            crate::gpio::mode::set_mode(Self {
                gpio: gpio.as_any(),
                port,
                version: crate::gpio::register::GpioVersion::$version,
                number,
            })
        }
    };
    (@one function, $constructor:ident, $register:ty, $version:ident) => {
        // Macro internal function for ROM runtime; DO NOT USE.
        #[doc(hidden)]
        #[inline]
        pub unsafe fn $constructor(gpio: &'a $register) -> Self {
            crate::gpio::mode::set_mode(Self {
                version: crate::gpio::register::GpioVersion::$version,
                gpio: gpio.as_any(),
            })
        }
    };
    (@versions $kind:ident) => {
        impl_gpio_constructors!(@one $kind, __new_v1, crate::gpio::v1::RegisterBlockV1, V1);
        impl_gpio_constructors!(@one $kind, __new_v2, crate::gpio::v2::RegisterBlockV2, V2);
        impl_gpio_constructors!(@one $kind, __new_v3, crate::gpio::v3::RegisterBlockV3, V3);
        impl_gpio_constructors!(@one $kind, __new_v4, crate::gpio::v4::RegisterBlockV4, V4);
    };
    ($kind:ident) => {
        impl_gpio_constructors!(@versions $kind);
    };
}
