use crate::DEFAULT_SEED_32;
use crate::rand::Rand32;

/// splitmix implementation with 32-bit state and 32-bit seed/output.
/// original implementation [here](https://stackoverflow.com/a/52056161).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct SplitMix32(u32);

impl Rand32 for SplitMix32 {
    #[inline]
    fn from_seed_u32(seed: u32) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(0x9E3779B9);
        let mut z = self.0;
	    z = (z ^ z.wrapping_shr(15)).wrapping_mul(0x85EBCA6B);
	    z = (z ^ z.wrapping_shr(13)).wrapping_mul(0xC2B2AE35);
    	z.wrapping_shr(16)
    }
}

impl Default for SplitMix32 {
    fn default() -> Self {
        Self::from_seed_u32(DEFAULT_SEED_32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix32() {
        let mut rng = SplitMix32::default();
        assert_eq!(rng.next_u32(), 62345);
    }
}
