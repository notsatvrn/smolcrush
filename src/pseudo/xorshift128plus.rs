use crate::pseudo::splitmix64::SplitMix64;
use crate::DEFAULT_SEED_64;
use crate::rand::Rand64;

// xorshift+ implementation with 128-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](https://prng.di.unimi.it/xorshift128plus.c).
pub struct XorShift128Plus(u64, u64);

impl Rand64 for XorShift128Plus {
    #[inline]
    fn from_seed_u64(seed: u64) -> Self {
        let mut sm64 = SplitMix64::from_seed_u64(seed);

        Self(sm64.next_u64(), sm64.next_u64())
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s1 = self.0;
        let s0 = self.1;

        let result = s0.wrapping_add(s1);

        self.0 = s0;
        let s1 = s1 ^ s1.wrapping_shl(23);
        self.1 = s1 ^ s0 ^ s1.wrapping_shr(18) ^ s0.wrapping_shr(5);

        result
    }
}

impl Default for XorShift128Plus {
    fn default() -> Self {
        Self::from_seed_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift128plus() {
        let mut rng = XorShift128Plus::default();
        assert_eq!(rng.next_u64(), 8455776818987521470);
    }
}

