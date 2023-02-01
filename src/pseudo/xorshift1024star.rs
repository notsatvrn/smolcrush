use crate::pseudo::splitmix64::SplitMix64;
use crate::DEFAULT_SEED_64;
use crate::rand::Rand64;

// xorshift* implementation with 1024-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](https://prng.di.unimi.it/xorshift1024star.c).
pub struct XorShift1024Star([u64; 16], usize);

impl Rand64 for XorShift1024Star {
    #[inline]
    fn from_seed_u64(seed: u64) -> Self {
        let mut sm64 = SplitMix64::from_seed_u64(seed);

        Self([
            sm64.next_u64(), sm64.next_u64(),
            sm64.next_u64(), sm64.next_u64(),
            sm64.next_u64(), sm64.next_u64(),
            sm64.next_u64(), sm64.next_u64(),
            sm64.next_u64(), sm64.next_u64(),
            sm64.next_u64(), sm64.next_u64(),
            sm64.next_u64(), sm64.next_u64(),
            sm64.next_u64(), sm64.next_u64(),
        ], 0)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.0[self.1];
        self.1 = (self.1 + 1) & 15;
        let s1 = self.0[self.1];
        let s1 = s1 ^ s1.wrapping_shl(31);
        self.0[self.1] = s1 ^ s0 ^ s1.wrapping_shr(11) ^ s0.wrapping_shr(30);
        self.0[self.1].wrapping_mul(0x9e3779b97f4a7c13)
    }
}

impl Default for XorShift1024Star {
    fn default() -> Self {
        Self::from_seed_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift1024star() {
        let mut rng = XorShift1024Star::default();
        assert_eq!(rng.next_u64(), 8011150333094582418);
    }
}

