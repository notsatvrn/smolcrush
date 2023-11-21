use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;

#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};

/// KISS implementation with 128-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-rng.html).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct KISS(u32, u32, u32, u32);

#[inline]
fn seed_from_u32(seed: u32) -> KISS {
    #[cfg(not(feature = "rand_core"))]
    let mut sm32 = SplitMix32::seed_from_u32(seed);
    #[cfg(feature = "rand_core")]
    let mut sm32 = SplitMix32::seed_from_u64(seed as u64);

    KISS(
        sm32.next_u32(), sm32.next_u32(),
        sm32.next_u32(), sm32.next_u32(),
    )
}

#[inline]
fn next_u32(rng: &mut KISS) -> u32 {
    rng.0 = 36969u32.wrapping_mul(rng.0 & 65535).wrapping_add(rng.0.wrapping_shr(16));
    rng.1 = 18000u32.wrapping_mul(rng.1 & 65535).wrapping_add(rng.1.wrapping_shr(16));
    let mwc = rng.0.wrapping_shl(16).wrapping_add(rng.1);

    rng.2 ^= rng.2.wrapping_shl(13);
    rng.2 ^= rng.2.wrapping_shr(17);
    rng.2 ^= rng.2.wrapping_shl(5);

    rng.3 = 69069u32.wrapping_mul(rng.3).wrapping_add(1234567);

    (mwc ^ rng.3).wrapping_add(rng.2)
}

#[cfg(not(feature = "rand_core"))]
impl Rand for KISS {
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
impl RngCore for KISS {
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
impl SeedableRng for KISS {
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

impl Default for KISS {
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
    fn kiss() {
        let mut rng = KISS::default();
        assert_eq!(rng.next_u32(), 2825447813);
    }
}
