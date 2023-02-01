use crate::DEFAULT_SEED_64;
use crate::rand::Rand64;

// xorshift implementation with 64-bit state and 64-bit seed/output.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct XorShift64(u64);

impl Rand64 for XorShift64 {
    #[inline]
    fn from_seed_u64(seed: u64) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0.wrapping_shl(13);
        self.0 ^= self.0.wrapping_shr(7);
        self.0 ^= self.0.wrapping_shl(17);
        self.0
    }
}

impl Default for XorShift64 {
    fn default() -> Self {
        Self::from_seed_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift64() {
        let mut rng = XorShift64::default();
        assert_eq!(rng.next_u64(), 5118981001015299295);
    }
}
