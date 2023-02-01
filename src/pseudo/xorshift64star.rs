use crate::DEFAULT_SEED_64;
use crate::rand::Rand64;

// xorshift* implementation with 64-bit state and 64-bit seed/output.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct XorShift64Star(u64);

impl Rand64 for XorShift64Star {
    #[inline]
    fn from_seed_u64(seed: u64) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0.wrapping_shl(12);
        self.0 ^= self.0.wrapping_shr(25);
        self.0 ^= self.0.wrapping_shl(27);
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

impl Default for XorShift64Star {
    fn default() -> Self {
        Self::from_seed_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift64star() {
        let mut rng = XorShift64Star::default();
        assert_eq!(rng.next_u64(), 794785870555032153);
    }
}
