use crate::DEFAULT_SEED_32;
#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand32;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};

// xorshift implementation with 32-bit state and 32-bit seed/output.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct XorShift32(u32);

#[inline]
fn next_u32(rng: &mut XorShift32) -> u32 {
    rng.0 ^= rng.0.wrapping_shl(13);
    rng.0 ^= rng.0.wrapping_shr(17);
    rng.0 ^= rng.0.wrapping_shl(5);
    rng.0
}

#[cfg(not(feature = "rand_core"))]
impl Rand32 for XorShift32 {
    #[inline]
    fn seed_from_u32(seed: u32) -> Self {
        Self(seed)
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        next_u32(self)
    }
}

#[cfg(feature = "rand_core")]
impl RngCore for XorShift32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        next_u32(self)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.next_u32() as u64 * self.next_u32() as u64
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
impl SeedableRng for XorShift32 {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: [u8; 8]) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        Self(seed as u32)
    }
}

impl Default for XorShift32 {
    fn default() -> Self {
        #[cfg(not(feature = "rand_core"))]
        let output = Self::seed_from_u32(DEFAULT_SEED_32);
        #[cfg(feature = "rand_core")]
        let output = Self::seed_from_u64(DEFAULT_SEED_32 as u64);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift32() {
        let mut rng = XorShift32::default();
        assert_eq!(rng.next_u32(), 3578445708);
    }
}
