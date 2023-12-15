use super::splitmix64::SplitMix64;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{Error, RngCore, SeedableRng};

// xorshift+ implementation with 1024-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](https://vigna.di.unimi.it/ftp/papers/xorshiftplus.pdf).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift1024Plus([u64; 16], usize);

impl RngCore for XorShift1024Plus {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.0[self.1];
        self.1 = (self.1 + 1) & 15;
        let s1 = self.0[self.1];
        let result = s0.wrapping_add(s1);
        let s1 = s1 ^ s1.wrapping_shl(31);
        self.0[self.1] = s1 ^ s0 ^ s1.wrapping_shr(11) ^ s0.wrapping_shr(30);
        result
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

impl SeedableRng for XorShift1024Plus {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: [u8; 8]) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        let mut sm64 = SplitMix64::seed_from_u64(seed);

        Self(
            [
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
                sm64.next_u64(),
            ],
            0,
        )
    }
}

impl Default for XorShift1024Plus {
    #[inline]
    fn default() -> Self {
        Self::seed_from_u64(crate::DEFAULT_SEED)
    }
}
