use crate::DEFAULT_SEED_64;
#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand64;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};
#[cfg(feature = "rand_core")]
use crate::rand::U64_U32;

// splitmix implementation with 64-bit state and 64-bit seed/output.
// original implementation [here](https://prng.di.unimi.it/splitmix64.c).
pub struct SplitMix64(u64);

#[inline]
fn next_u64(rng: &mut SplitMix64) -> u64 {
    rng.0 = rng.0.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = rng.0;
    z = (z ^ z.wrapping_shr(30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ z.wrapping_shr(27)).wrapping_mul(0x94D049BB133111EB);
    z ^ z.wrapping_shr(31)
}

#[cfg(not(feature = "rand_core"))]
impl Rand64 for SplitMix64 {
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
impl RngCore for SplitMix64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        (next_u64(self) / U64_U32) as u32
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
impl SeedableRng for SplitMix64 {
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

impl Default for SplitMix64 {
    fn default() -> Self {
        Self::seed_from_u64(DEFAULT_SEED_64)
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
