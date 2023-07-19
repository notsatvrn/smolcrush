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

// swb implementation with 8192-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-rng.html).
pub struct SWB64([u64; 256], u64, u64, usize);

#[inline]
fn seed_from_u64(seed: u64) -> SWB64 {
    let mut sm64 = SplitMix64::seed_from_u64(seed);
    let mut state = [0; 256];

    for i in &mut state {
        *i = sm64.next_u64();
    }

    SWB64(state, sm64.next_u64(), sm64.next_u64(), 0)
}

#[inline]
fn next_u64(rng: &mut SWB64) -> u64 {
    rng.3 = rng.3.wrapping_add(1);

    let bro = rng.1.min(rng.2);

    rng.1 = rng.0[rng.3.wrapping_add(34) & 255];
    rng.2 = rng.0[rng.3.wrapping_add(19) & 255].wrapping_add(bro);

    rng.0[rng.3] = rng.1.wrapping_sub(rng.2);
    rng.0[rng.3]
}

#[cfg(not(feature = "rand_core"))]
impl Rand64 for SWB64 {
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
impl RngCore for SWB64 {
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
impl SeedableRng for SWB64 {
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

impl Default for SWB64 {
    fn default() -> Self {
        Self::seed_from_u64(DEFAULT_SEED_64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swb64() {
        let mut rng = SWB64::default();
        assert_eq!(rng.next_u64(), 7874828165597159784);
    }
}
