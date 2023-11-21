use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;

#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};

// xorshift implementation with 128-bit state and 32-bit seed/output.
// state generated from seed using splitmix32.
// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift128(u32, u32, u32, u32);

#[inline]
fn seed_from_u32(seed: u32) -> XorShift128 {
    #[cfg(not(feature = "rand_core"))]
    let mut sm32 = SplitMix32::seed_from_u32(seed);
    #[cfg(feature = "rand_core")]
    let mut sm32 = SplitMix32::seed_from_u64(seed as u64);

    XorShift128(
        sm32.next_u32(), sm32.next_u32(),
        sm32.next_u32(), sm32.next_u32(),
    )
}

#[inline]
fn next_u32(rng: &mut XorShift128) -> u32 {
    let t = rng.3;
    let s = rng.0;

    rng.3 = rng.2;
    rng.2 = rng.1;
    rng.1 = s;

    let t = t ^ t.wrapping_shl(11);
    let t = t ^ t.wrapping_shr(8);

    rng.0 = t ^ s ^ s.wrapping_shr(19);

    rng.0
}

#[cfg(not(feature = "rand_core"))]
impl Rand for XorShift128 {
    #[inline]
    fn seed_from_u32(seed: u32) -> Self {
        seed_from_u32(seed)
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        next_u32(self)
    }
}

#[cfg(feature = "rand_core")]
impl RngCore for XorShift128 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        next_u32(self)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.next_u32() as u64 | self.next_u32() as u64 >> 32
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
impl SeedableRng for XorShift128 {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: [u8; 8]) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        seed_from_u32(seed as u32)
    }
}

impl Default for XorShift128 {
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
    fn xorshift128() {
        let mut rng = XorShift128::default();
        assert_eq!(rng.next_u32(), 65450646);
    }
}
