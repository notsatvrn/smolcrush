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

// xorshift* implementation with 1024-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](https://prng.di.unimi.it/xorshift1024star.c).
pub struct XorShift1024Star([u64; 16], usize);

#[inline]
fn seed_from_u64(seed: u64) -> XorShift1024Star {
    let mut sm64 = SplitMix64::seed_from_u64(seed);

    XorShift1024Star([
        sm64.next_u64(), sm64.next_u64(),
        sm64.next_u64(), sm64.next_u64(),
        sm64.next_u64(), sm64.next_u64(),
        sm64.next_u64(), sm64.next_u64(),
        sm64.next_u64(), sm64.next_u64(),
        sm64.next_u64(), sm64.next_u64(),
        sm64.next_u64(), sm64.next_u64(),
        sm64.next_u64(), sm64.next_u64(),
    ], 0)
}

#[inline]
fn next_u64(rng: &mut XorShift1024Star) -> u64 {
    let s0 = rng.0[rng.1];
    rng.1 = (rng.1 + 1) & 15;
    let s1 = rng.0[rng.1];
    let s1 = s1 ^ s1.wrapping_shl(31);
    rng.0[rng.1] = s1 ^ s0 ^ s1.wrapping_shr(11) ^ s0.wrapping_shr(30);
    rng.0[rng.1].wrapping_mul(0x9e3779b97f4a7c13)
}

#[cfg(not(feature = "rand_core"))]
impl Rand64 for XorShift1024Star {
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
impl RngCore for XorShift1024Star {
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
impl SeedableRng for XorShift1024Star {
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

impl Default for XorShift1024Star {
    fn default() -> Self {
        Self::seed_from_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift1024star() {
        let mut rng = XorShift1024Star::default();
        assert_eq!(rng.next_u64(), 8011150333094582418);
    }
}
