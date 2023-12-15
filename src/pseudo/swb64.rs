use super::splitmix64::SplitMix64;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{Error, RngCore, SeedableRng};

// swb implementation with 8192-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-self.html).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct SWB64([u64; 256], u64, u64, usize);

impl RngCore for SWB64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.3 = self.3.wrapping_add(1);

        let bro = self.1.min(self.2);

        self.1 = self.0[self.3.wrapping_add(34) & 255];
        self.2 = self.0[self.3.wrapping_add(19) & 255].wrapping_add(bro);

        self.0[self.3] = self.1.wrapping_sub(self.2);
        self.0[self.3]
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

impl SeedableRng for SWB64 {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: [u8; 8]) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        let mut sm64 = SplitMix64::seed_from_u64(seed);
        let mut state = [0; 256];

        for i in &mut state {
            *i = sm64.next_u64();
        }

        Self(state, sm64.next_u64(), sm64.next_u64(), 0)
    }
}

impl Default for SWB64 {
    #[inline]
    fn default() -> Self {
        Self::seed_from_u64(crate::DEFAULT_SEED)
    }
}
