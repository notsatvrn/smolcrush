use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;
#[cfg(not(feature = "rand_core"))]
use crate::rand::Rand32;

#[cfg(feature = "rand_core")]
use rand_core::impls::fill_bytes_via_next;
#[cfg(feature = "rand_core")]
use rand_core::{RngCore, SeedableRng, Error};

// swb implementation with 8192-bit state and 32-bit seed/output.
// state generated from seed using splitmix32.
// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-rng.html).
pub struct SWB32([u32; 256], u32, u32, usize);

#[inline]
fn seed_from_u32(seed: u32) -> SWB32 {
    let mut sm32 = SplitMix32::seed_from_u64(seed as u64);
    let mut state = [0; 256];

    for i in &mut state {
        *i = sm32.next_u32();
    }

    SWB32(state, sm32.next_u32(), sm32.next_u32(), 0)
}

#[inline]
fn next_u32(rng: &mut SWB32) -> u32 {
    rng.3 = rng.3.wrapping_add(1);

    let bro = rng.1.min(rng.2);

    rng.1 = rng.0[rng.3.wrapping_add(34) & 255];
    rng.2 = rng.0[rng.3.wrapping_add(19) & 255].wrapping_add(bro);

    rng.0[rng.3] = rng.1.wrapping_sub(rng.2);
    rng.0[rng.3]
}

#[cfg(not(feature = "rand_core"))]
impl Rand32 for SWB32 {
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
impl RngCore for SWB32 {
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
impl SeedableRng for SWB32 {
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

impl Default for SWB32 {
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
    fn swb32() {
        let mut rng = SWB32::default();
        assert_eq!(rng.next_u32(), 4294965539);
    }
}
