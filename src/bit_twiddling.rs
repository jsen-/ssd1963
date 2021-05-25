macro_rules! impl_bit_index {
    ($ty:ty, $n:literal) => {
        impl BitIndex<$n> for $ty {}
    };
    ($ty:ty, $($n:literal),+) => {
        impl SetBit for $ty {
            type Output = Self;
            fn set_bit<const N: usize>(self, val: bool) -> Self::Output
            where
                Self: BitIndex<N>,
            {
                let bit = if val { 1 << N } else { 0 };
                self & !(1 << N) | bit
            }
        }
        $(impl_bit_index!($ty, $n);)+
    };
}
impl_bit_index!(u8, 0, 1, 2, 3, 4, 5, 6, 7);
// impl_bit_index!(u16, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

pub trait BitIndex<const N: usize> {}
pub trait SetBit {
    type Output;
    fn set_bit<const N: usize>(self, val: bool) -> Self::Output
    where
        Self: BitIndex<N>;
}

impl SetBit for &mut u8 {
    type Output = ();
    fn set_bit<const N: usize>(self, val: bool) -> Self::Output
    where
        Self: BitIndex<N>,
    {
        let bit = if val { 1 << N } else { 0 };
        *self = *self & !(1 << N) | bit;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bits() {
        assert_eq!(0u8.set_bit::<0>(true), 1);
        assert_eq!(0u8.set_bit::<1>(true), 2);
        assert_eq!(1u8.set_bit::<1>(true), 3);
        assert_eq!(1u8.set_bit::<0>(false), 0);
        assert_eq!(1u8.set_bit::<1>(false), 1);
    }
}
