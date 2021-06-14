use super::{BitSet, BitSetMut};

macro_rules! impl_for_primitive {
    ($ty:ty : $size:literal) => {
        impl BitSet for $ty {
            const UPPER_BOUND: u32 = $size;

            #[inline]
            fn get(&self, index: u32) -> bool {
                assert!(index < $size);
                0 != *self & ((1 as $ty) << index)
            }

            #[inline]
            fn find_set(&self, lower_bound: u32) -> Option<u32> {
                assert!(lower_bound < $size);
                let masked = *self & (!0 as $ty).wrapping_shl(lower_bound);
                match masked.trailing_zeros() {
                    $size => None,
                    index => Some(index),
                }
            }
        }

        impl BitSetMut for $ty {
            #[inline]
            fn set(&mut self, index: u32, bit: bool) {
                assert!(index < $size);
                match bit {
                    true => *self |= (1 as $ty << index),
                    false => *self &= !(1 as $ty << index),
                }
            }
        }
    };

    // ($ty:ty : $size:literal, $($tail_ty:ty : $tail_size:literal),+ $(,)?) => {
    //     impl_for_primitive!($($tail_ty : $tail_size),+);
    // }

    ($ty:ty : $size:literal, $($tail:tt)+) => {
        impl_for_primitive!($ty : $size);
        impl_for_primitive!($($tail)+);
    }
}

impl_for_primitive!(u8 : 8, u16 : 16, u32 : 32, u64 : 64, u128 : 128);
