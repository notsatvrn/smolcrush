use crate::DEFAULT_SEED_64;
use crate::rand::Rand64;

// splitmix implementation with 64-bit state and 64-bit seed/output.
// original implementation [here](https://prng.di.unimi.it/splitmix64.c).
pub struct SplitMix64(u64);

impl Rand64 for SplitMix64 {
    #[inline]
    fn from_seed_u64(seed: u64) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
	    z = (z ^ z.wrapping_shr(30)).wrapping_mul(0xBF58476D1CE4E5B9);
	    z = (z ^ z.wrapping_shr(27)).wrapping_mul(0x94D049BB133111EB);
    	z ^ z.wrapping_shr(31)
    }
}

impl Default for SplitMix64 {
    fn default() -> Self {
        Self::from_seed_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64() {
        let mut rng = SplitMix64::default();
        assert_eq!(rng.next_u64(), 9229435967816235190);
    }
}
