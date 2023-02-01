use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;
use crate::rand::Rand32;

// xorshift implementation with 128-bit state and 32-bit seed/output.
// state generated from seed using splitmix32.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct XorShift128(u32, u32, u32, u32);

impl Rand32 for XorShift128 {
    #[inline]
    fn from_seed_u32(seed: u32) -> Self {
        let mut sm32 = SplitMix32::from_seed_u32(seed);

        Self(
            sm32.next_u32(), sm32.next_u32(),
            sm32.next_u32(), sm32.next_u32(),
        )
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let t = self.3;
        let s = self.0;

        self.3 = self.2;
        self.2 = self.1;
        self.1 = s;

        let t = t ^ t.wrapping_shl(11);
        let t = t ^ t.wrapping_shr(8);

        self.0 = t ^ s ^ s.wrapping_shr(19);

        self.0
    }
}

impl Default for XorShift128 {
    fn default() -> Self {
        Self::from_seed_u32(DEFAULT_SEED_32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift128() {
        let mut rng = XorShift128::default();
        assert_eq!(rng.next_u32(), 65450646);
    }
}
