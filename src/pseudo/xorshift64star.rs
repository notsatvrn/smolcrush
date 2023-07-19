use crate::DEFAULT_SEED_64;
#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand64;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};
#[cfg(feature = "rand_core")]
use crate::rand::U64_U32;

// xorshift* implementation with 64-bit state and 64-bit seed/output.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct XorShift64Star(u64);

#[inline]
fn next_u64(rng: &mut XorShift64Star) -> u64 {
    rng.0 ^= rng.0.wrapping_shl(12);
    rng.0 ^= rng.0.wrapping_shr(25);
    rng.0 ^= rng.0.wrapping_shl(27);
    rng.0.wrapping_mul(0x2545F4914F6CDD1D)
}

#[cfg(not(feature = "rand_core"))]
impl Rand64 for XorShift64Star {
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
impl RngCore for XorShift64Star {
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
impl SeedableRng for XorShift64Star {
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

impl Default for XorShift64Star {
    fn default() -> Self {
        Self::seed_from_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift64star() {
        let mut rng = XorShift64Star::default();
        assert_eq!(rng.next_u64(), 794785870555032153);
    }
}
