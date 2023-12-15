use super::splitmix32::SplitMix32;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{Error, RngCore, SeedableRng};

/// KISS implementation with 128-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-self.html).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct KISS(u32, u32, u32, u32);

impl RngCore for KISS {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0 = 36969u32
            .wrapping_mul(self.0 & 65535)
            .wrapping_add(self.0.wrapping_shr(16));
        self.1 = 18000u32
            .wrapping_mul(self.1 & 65535)
            .wrapping_add(self.1.wrapping_shr(16));
        let mwc = self.0.wrapping_shl(16).wrapping_add(self.1);

        self.2 ^= self.2.wrapping_shl(13);
        self.2 ^= self.2.wrapping_shr(17);
        self.2 ^= self.2.wrapping_shl(5);

        self.3 = 69069u32.wrapping_mul(self.3).wrapping_add(1234567);

        (mwc ^ self.3).wrapping_add(self.2)
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

impl SeedableRng for KISS {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: [u8; 8]) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        let mut sm32 = SplitMix32::seed_from_u64(seed);

        Self(
            sm32.next_u32(),
            sm32.next_u32(),
            sm32.next_u32(),
            sm32.next_u32(),
        )
    }
}

impl Default for KISS {
    #[inline]
    fn default() -> Self {
        Self::seed_from_u64(crate::DEFAULT_SEED)
    }
}
