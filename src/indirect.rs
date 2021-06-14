use super::{BitSet, BitSetMut};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

impl<T> BitSet for &'_ T
where
    T: BitSet,
{
    const UPPER_BOUND: u32 = T::UPPER_BOUND;

    fn get(&self, index: u32) -> bool {
        T::get(*self, index)
    }

    fn find_set(&self, lower_bound: u32) -> Option<u32> {
        T::find_set(*self, lower_bound)
    }
}

impl<T> BitSet for &'_ mut T
where
    T: BitSet,
{
    const UPPER_BOUND: u32 = T::UPPER_BOUND;

    fn get(&self, index: u32) -> bool {
        T::get(*self, index)
    }

    fn find_set(&self, lower_bound: u32) -> Option<u32> {
        T::find_set(*self, lower_bound)
    }
}

impl<T> BitSetMut for &'_ mut T
where
    T: BitSetMut,
{
    fn set(&mut self, index: u32, bit: bool) {
        T::set(*self, index, bit)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitSet for Box<T>
where
    T: BitSet,
{
    const UPPER_BOUND: u32 = T::UPPER_BOUND;

    fn get(&self, index: u32) -> bool {
        T::get(&**self, index)
    }

    fn find_set(&self, lower_bound: u32) -> Option<u32> {
        T::find_set(&**self, lower_bound)
    }
}

#[cfg(feature = "alloc")]
impl<T> BitSetMut for Box<T>
where
    T: BitSetMut,
{
    fn set(&mut self, index: u32, bit: bool) {
        T::set(&mut **self, index, bit)
    }
}
