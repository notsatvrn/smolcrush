use crate::pseudo::splitmix64::SplitMix64;
use crate::DEFAULT_SEED_64;
#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand64;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};
#[cfg(feature = "rand_core")]
use crate::rand::U64_U32;

// xorshift+ implementation with 128-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](https://prng.di.unimi.it/xorshift128plus.c).
pub struct XorShift128Plus(u64, u64);

#[inline]
fn seed_from_u64(seed: u64) -> XorShift128Plus {
    let mut sm64 = SplitMix64::seed_from_u64(seed);

    XorShift128Plus(sm64.next_u64(), sm64.next_u64())
}

#[inline]
fn next_u64(rng: &mut XorShift128Plus) -> u64 {
    let s1 = rng.0;
    let s0 = rng.1;

    let result = s0.wrapping_add(s1);

    rng.0 = s0;
    let s1 = s1 ^ s1.wrapping_shl(23);
    rng.1 = s1 ^ s0 ^ s1.wrapping_shr(18) ^ s0.wrapping_shr(5);

    result
}

#[cfg(not(feature = "rand_core"))]
impl Rand64 for XorShift128Plus {
    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        seed_from_u64(seed)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        next_u64(self)
    }
}

#[cfg(feature = "rand_core")]
impl RngCore for XorShift128Plus {
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
impl SeedableRng for XorShift128Plus {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: [u8; 8]) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        seed_from_u64(seed)
    }
}

impl Default for XorShift128Plus {
    fn default() -> Self {
        Self::seed_from_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift128plus() {
        let mut rng = XorShift128Plus::default();
        assert_eq!(rng.next_u64(), 8455776818987521470);
    }
}
