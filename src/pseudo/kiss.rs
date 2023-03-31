use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;
use crate::rand::Rand32;

/// KISS implementation with 128-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-rng.html).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct KISS(u32, u32, u32, u32);

impl Rand32 for KISS {
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
        self.0 = 36969u32.wrapping_mul(self.0 & 65535).wrapping_add(self.0.wrapping_shr(16));
        self.1 = 18000u32.wrapping_mul(self.1 & 65535).wrapping_add(self.1.wrapping_shr(16));
        let mwc = self.0.wrapping_shl(16).wrapping_add(self.1);

        self.2 ^= self.2.wrapping_shl(13);
        self.2 ^= self.2.wrapping_shr(17);
        self.2 ^= self.2.wrapping_shl(5);

        self.3 = 69069u32.wrapping_mul(self.3).wrapping_add(1234567);

        (mwc ^ self.3).wrapping_add(self.2)
    }
}

impl Default for KISS {
    fn default() -> Self {
        Self::from_seed_u32(DEFAULT_SEED_32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kiss() {
        let mut rng = KISS::default();
        assert_eq!(rng.next_u32(), 2825447813);
    }
}
