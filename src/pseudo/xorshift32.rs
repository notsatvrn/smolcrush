use crate::DEFAULT_SEED_32;
use crate::rand::Rand32;

// xorshift implementation with 32-bit state and 32-bit seed/output.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct XorShift32(u32);

impl Rand32 for XorShift32 {
    #[inline]
    fn from_seed_u32(seed: u32) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0.wrapping_shl(13);
        self.0 ^= self.0.wrapping_shr(17);
        self.0 ^= self.0.wrapping_shl(5);
        self.0
    }
}

impl Default for XorShift32 {
    fn default() -> Self {
        Self::from_seed_u32(DEFAULT_SEED_32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift32() {
        let mut rng = XorShift32::default();
        assert_eq!(rng.next_u32(), 3578445708);
    }
}
