use super::splitmix32::SplitMix32;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{Error, RngCore, SeedableRng};

/// xorwow implementation with 192-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct Xorwow(u32, u32, u32, u32, u32, u32);

impl RngCore for Xorwow {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let t = self.4;
        let s = self.0;

        self.4 = self.3;
        self.3 = self.2;
        self.2 = self.1;
        self.1 = s;

        let t = t ^ t.wrapping_shr(2);
        let t = t ^ t.wrapping_shl(1);
        let t = t ^ s ^ s.wrapping_shl(4);

        self.0 = t;

        self.5 = self.5.wrapping_add(362437);
        self.5.wrapping_add(t)
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

impl SeedableRng for Xorwow {
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
            sm32.next_u32(),
            sm32.next_u32(),
        )
    }
}

impl Default for Xorwow {
    #[inline]
    fn default() -> Self {
        Self::seed_from_u64(crate::DEFAULT_SEED)
    }
}
