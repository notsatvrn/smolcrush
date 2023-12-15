use rand_core::impls::fill_bytes_via_next;
use rand_core::{Error, RngCore, SeedableRng};

/// splitmix implementation with 32-bit state and 32-bit seed/output.
/// original implementation [here](https://stackoverflow.com/a/52056161).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct SplitMix32(u32);

impl RngCore for SplitMix32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(0x9E3779B9);
        let mut z = self.0;
        z = (z ^ z.wrapping_shr(15)).wrapping_mul(0x85EBCA6B);
        z = (z ^ z.wrapping_shr(13)).wrapping_mul(0xC2B2AE35);
        z.wrapping_shr(16)
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

impl SeedableRng for SplitMix32 {
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

impl Default for SplitMix32 {
    #[inline]
    fn default() -> Self {
        Self::seed_from_u64(crate::DEFAULT_SEED)
    }
}
