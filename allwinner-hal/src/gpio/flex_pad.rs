use core::marker::PhantomData;

use super::Function;

pub struct FlexPad<'a> {
    _base: PhantomData<&'a super::RegisterBlock>,
}

impl<'a, const P: char, const N: u8, const F: u8> From<Function<'a, P, N, F>> for FlexPad<'a> {
    #[inline]
    fn from(value: Function<'a, P, N, F>) -> Self {
        let _ = value;
        FlexPad { _base: PhantomData }
    }
}
