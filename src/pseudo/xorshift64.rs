use crate::DEFAULT_SEED_64;

#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};

// xorshift implementation with 64-bit state and 64-bit seed/output.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift64(u64);

#[inline]
fn next_u64(rng: &mut XorShift64) -> u64 {
    rng.0 ^= rng.0.wrapping_shl(13);
    rng.0 ^= rng.0.wrapping_shr(7);
    rng.0 ^= rng.0.wrapping_shl(17);
    rng.0
}

#[cfg(not(feature = "rand_core"))]
impl Rand for XorShift64 {
    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        next_u64(self)
    }
}

#[cfg(feature = "rand_core")]
impl RngCore for XorShift64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        next_u64(self) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        next_u64(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(feature = "rand_core")]
impl SeedableRng for XorShift64 {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: [u8; 8]) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        Self(seed)
    }
}

impl Default for XorShift64 {
    fn default() -> Self {
        Self::seed_from_u64(DEFAULT_SEED_64)
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
