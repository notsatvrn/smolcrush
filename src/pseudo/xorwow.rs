use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;

#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand32;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};

/// xorwow implementation with 192-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
pub struct Xorwow(u32, u32, u32, u32, u32, u32);

#[inline]
fn seed_from_u32(seed: u32) -> Xorwow {
    #[cfg(not(feature = "rand_core"))]
    let mut sm32 = SplitMix32::seed_from_u32(seed);
    #[cfg(feature = "rand_core")]
    let mut sm32 = SplitMix32::seed_from_u64(seed as u64);

    Xorwow(
        sm32.next_u32(), sm32.next_u32(),
        sm32.next_u32(), sm32.next_u32(),
        sm32.next_u32(), sm32.next_u32(),
    )
}

#[inline]
fn next_u32(rng: &mut Xorwow) -> u32 {
    let t = rng.4;
    let s = rng.0;

    rng.4 = rng.3;
    rng.3 = rng.2;
    rng.2 = rng.1;
    rng.1 = s;

    let t = t ^ t.wrapping_shr(2);
    let t = t ^ t.wrapping_shl(1);
    let t = t ^ s ^ s.wrapping_shl(4);

    rng.0 = t;

    rng.5 = rng.5.wrapping_add(362437);
    rng.5.wrapping_add(t)
}

#[cfg(not(feature = "rand_core"))]
impl Rand32 for Xorwow {
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
impl RngCore for Xorwow {
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
impl SeedableRng for Xorwow {
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

impl Default for Xorwow {
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
    fn xorwow() {
        let mut rng = Xorwow::default();
        assert_eq!(rng.next_u32(), 1361759);
    }
}
