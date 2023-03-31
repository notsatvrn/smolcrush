use crate::DEFAULT_SEED_64;
use crate::rand::Rand64;

// wyrand implementation with 64-bit state and 64-bit seed/output.
// original implementation [here](https://github.com/wangyi-fudan/wyhash/blob/master/wyhash.h).
pub struct WyRand(u64);

impl Rand64 for WyRand {
    #[inline]
    fn from_seed_u64(seed: u64) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0xA0761D6478BD642F);
        let y = (self.0 as u128).wrapping_mul((self.0 as u128) ^ 0xE7037ED1A0B428DB);
        (y.wrapping_shr(64) ^ y) as u64
    }
}

impl Default for WyRand {
    fn default() -> Self {
        Self::from_seed_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wyrand() {
        let mut rng = WyRand::default();
        assert_eq!(rng.next_u64(), 6736572058214918811);
    }
}
