#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod indirect;
mod layered;
mod option;
mod primitive;

pub use layered::Layered;

/// Common trait for all bitsets in `bitseto` crate.
pub trait BitSet {
    /// Upper bound for this bitset indices.
    /// Implementation is expected to panic if index equal to or greter than `UPPER_BOUND` is speicifed.
    const UPPER_BOUND: u32;

    /// Returns bit state at specified index.
    ///
    /// # Panics
    ///
    /// This function may panic if `index` is equal to or greter than `UPPER_BOUND`.
    fn get(&self, index: u32) -> bool;

    /// Returns index of first set bit at index equal to or greter than specified.
    ///
    /// # Panics
    ///
    /// This function may panic if `index` is equal to or greter than `UPPER_BOUND`.
    fn find_set(&self, lower_bound: u32) -> Option<u32>;

    /// Returns true if no bit is set.
    fn is_empty(&self) -> bool {
        self.find_set(0).is_none()
    }
}

/// Common trait for all mutable bitsets in `bitseto` crate.
pub trait BitSetMut: BitSet {
    /// Sets bit state at specified index.
    ///
    /// # Panics
    ///
    /// This function may panic if `index` is equal to or greter than `UPPER_BOUND`.
    fn set(&mut self, index: u32, bit: bool);
}

pub type BitSet8 = u8;
pub type BitSet16 = u16;
pub type BitSet32 = u32;
pub type BitSet64 = u64;
pub type BitSet128 = u128;
pub type BitSet256 = layered::Layered<u32, u8, 32>;
pub type BitSet512 = layered::Layered<u64, u8, 64>;
pub type BitSet1024 = layered::Layered<u64, u16, 64>;
pub type BitSet2048 = layered::Layered<u64, u32, 64>;
pub type BitSet4096 = layered::Layered<u64, u64, 64>;
pub type BitSet8192 = layered::Layered<u64, u128, 64>;
pub type BitSet16384 = layered::Layered<u128, u128, 128>;

#[cfg(feature = "alloc")]
pub type BitSet32768 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet512>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet65536 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet1024>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet131072 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet2048>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet262144 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet4096>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet524288 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet8192>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet1048576 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet16384>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet2097152 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet32768>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet4194304 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet65536>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet8388608 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet131072>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet16777216 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet262144>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet33554432 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet524288>>, 64>;

#[cfg(feature = "alloc")]
pub type BitSet67108864 = layered::Layered<u64, Option<alloc::boxed::Box<BitSet1048576>>, 64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bounds() {
        assert_eq!(BitSet8::UPPER_BOUND, 8);
        assert_eq!(BitSet16::UPPER_BOUND, 16);
        assert_eq!(BitSet32::UPPER_BOUND, 32);
        assert_eq!(BitSet64::UPPER_BOUND, 64);
        assert_eq!(BitSet128::UPPER_BOUND, 128);
        assert_eq!(BitSet256::UPPER_BOUND, 256);
        assert_eq!(BitSet512::UPPER_BOUND, 512);
        assert_eq!(BitSet1024::UPPER_BOUND, 1024);
        assert_eq!(BitSet2048::UPPER_BOUND, 2048);
        assert_eq!(BitSet4096::UPPER_BOUND, 4096);
        assert_eq!(BitSet8192::UPPER_BOUND, 8192);
        assert_eq!(BitSet16384::UPPER_BOUND, 16384);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn check_large_bounds() {
        assert_eq!(BitSet32768::UPPER_BOUND, 32768);
        assert_eq!(BitSet65536::UPPER_BOUND, 65536);
        assert_eq!(BitSet131072::UPPER_BOUND, 131072);
        assert_eq!(BitSet262144::UPPER_BOUND, 262144);
        assert_eq!(BitSet524288::UPPER_BOUND, 524288);
        assert_eq!(BitSet1048576::UPPER_BOUND, 1048576);
        assert_eq!(BitSet2097152::UPPER_BOUND, 2097152);
        assert_eq!(BitSet4194304::UPPER_BOUND, 4194304);
        assert_eq!(BitSet8388608::UPPER_BOUND, 8388608);
        assert_eq!(BitSet16777216::UPPER_BOUND, 16777216);
        assert_eq!(BitSet33554432::UPPER_BOUND, 33554432);
        assert_eq!(BitSet67108864::UPPER_BOUND, 67108864);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn check_set() {
        let mut bits = Layered::<u64, u64, 64>::default();

        assert_eq!(bits.get(421), false);
        bits.set(421, true);
        assert_eq!(bits.get(421), true);

        bits.set(213, true);

        assert_eq!(bits.find_set(0), Some(213));
        assert_eq!(bits.find_set(300), Some(421));
        assert_eq!(bits.find_set(421 + 1), None);

        bits.set(213, false);
        assert_eq!(bits.find_set(0), Some(421));
        assert_eq!(bits.find_set(421 + 1), None);

        bits.set(421, false);
        assert_eq!(bits.get(421), false);
    }
}
