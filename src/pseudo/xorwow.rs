use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;
use crate::rand::Rand32;

/// xorwow implementation with 192-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct Xorwow(u32, u32, u32, u32, u32, u32);

impl Rand32 for Xorwow {
    #[inline]
    fn from_seed_u32(seed: u32) -> Self {
        let mut sm32 = SplitMix32::from_seed_u32(seed);

        Self(
            sm32.next_u32(), sm32.next_u32(),
            sm32.next_u32(), sm32.next_u32(),
            sm32.next_u32(), sm32.next_u32(),
        )
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let t = self.4;
        let s = self.0;

        self.4 = self.3;
        self.3 = self.2;
        self.2 = self.1;
        self.1 = s;

        let t = t ^ t.wrapping_shr(2);
        let t = t ^ t.wrapping_shl(1);
        let t = t ^ s ^ s.wrapping_shl(4);

        self.0 = t;

        self.5 = self.5.wrapping_add(362437);
        self.5.wrapping_add(t)
    }
}

impl Default for Xorwow {
    fn default() -> Self {
        Self::from_seed_u32(DEFAULT_SEED_32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorwow() {
        let mut rng = Xorwow::default();
        assert_eq!(rng.next_u32(), 1361759);
    }
}
