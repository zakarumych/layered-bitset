use crate::{BitSet, BitSetMut};

impl<T> BitSet for Option<T>
where
    T: BitSet,
{
    const UPPER_BOUND: u32 = T::UPPER_BOUND;

    fn get(&self, index: u32) -> bool {
        match self {
            None => false,
            Some(bits) => bits.get(index),
        }
    }

    fn find_set(&self, lower_bound: u32) -> Option<u32> {
        match self {
            None => None,
            Some(bits) => bits.find_set(lower_bound),
        }
    }
}

impl<T> BitSetMut for Option<T>
where
    T: BitSetMut + Default,
{
    fn set(&mut self, index: u32, bit: bool) {
        match (&mut *self, bit) {
            (None, true) => {
                let mut bits = T::default();
                bits.set(index, true);
                *self = Some(bits);
            }
            (None, false) => {}
            (Some(bits), false) => {
                bits.set(index, false);
                if bits.is_empty() {
                    *self = None;
                }
            }
            (Some(bits), true) => bits.set(index, true),
        }
    }
}
