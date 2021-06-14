use crate::{BitSet, BitSetMut};

#[derive(Debug)]
pub struct Layered<T, B, const N: usize> {
    top: T,
    bottom: [B; N],
}

impl<T, B, const N: usize> Default for Layered<T, B, N>
where
    T: Default + BitSet,
    B: Default + BitSet,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, B, const N: usize> Layered<T, B, N>
where
    T: BitSet,
    B: BitSet,
{
    fn new() -> Self
    where
        T: Default,
        B: Default,
    {
        assert_eq!(T::UPPER_BOUND, N as u32);
        assert_eq!(T::UPPER_BOUND as usize, N);

        use core::mem::MaybeUninit;
        let bottom = unsafe {
            let mut array: [MaybeUninit<B>; N] = MaybeUninit::uninit().assume_init();
            for item in &mut array {
                // Leaks all previously written elements on panic. It is safe though.
                core::ptr::write(item, MaybeUninit::new(B::default()));
            }
            (&array as *const _ as *const [B; N]).read()
        };

        Layered {
            top: T::default(),
            bottom,
        }
    }
}

impl<T, B, const N: usize> BitSet for Layered<T, B, N>
where
    T: BitSet,
    B: BitSet,
{
    const UPPER_BOUND: u32 = B::UPPER_BOUND * N as u32;

    fn get(&self, index: u32) -> bool {
        assert!(index < Self::UPPER_BOUND);

        let t = index / B::UPPER_BOUND;
        let b = index % B::UPPER_BOUND;

        self.bottom[t as usize].get(b)
    }

    fn find_set(&self, lower_bound: u32) -> Option<u32> {
        assert!(lower_bound < Self::UPPER_BOUND);

        let t = lower_bound / B::UPPER_BOUND;
        let b = lower_bound % B::UPPER_BOUND;

        if b == 0 {
            let t = self.top.find_set(t)?;
            let b = self.bottom[t as usize].find_set(0)?;
            Some(t * B::UPPER_BOUND + b)
        } else {
            if self.top.get(t) {
                if let Some(b) = self.bottom[t as usize].find_set(b) {
                    return Some(t * B::UPPER_BOUND + b);
                }
            }

            let t = self.top.find_set(t + 1)?;
            let b = self.bottom[t as usize].find_set(0)?;
            Some(t * B::UPPER_BOUND + b)
        }
    }
}

impl<T, B, const N: usize> BitSetMut for Layered<T, B, N>
where
    T: BitSetMut,
    B: BitSetMut,
{
    fn set(&mut self, index: u32, bit: bool) {
        assert!(index < Self::UPPER_BOUND);
        let t = index / B::UPPER_BOUND;

        if bit {
            if !self.top.get(t) {
                self.top.set(t, true);
                let u = index % B::UPPER_BOUND;
                self.bottom[t as usize].set(u, true)
            }
        } else {
            if self.top.get(t) {
                let u = index % B::UPPER_BOUND;
                self.bottom[t as usize].set(u, false);
                if self.bottom[t as usize].is_empty() {
                    self.top.set(t, false);
                }
            }
        }
    }
}
