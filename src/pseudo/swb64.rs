use crate::pseudo::splitmix64::SplitMix64;
use crate::DEFAULT_SEED_64;
use crate::rand::Rand64;

// swb implementation with 8192-bit state and 64-bit seed/output.
// state generated from seed using splitmix64.
// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-rng.html).
pub struct SWB64([u64; 256], u64, u64, usize);

impl Rand64 for SWB64 {
    #[inline]
    fn from_seed_u64(seed: u64) -> Self {
        let mut sm64 = SplitMix64::from_seed_u64(seed);
        let mut state = [0; 256];

        for i in &mut state {
            *i = sm64.next_u64();
        }

        Self(state, sm64.next_u64(), sm64.next_u64(), 0)
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
}

impl Default for SWB64 {
    fn default() -> Self {
        Self::from_seed_u64(DEFAULT_SEED_64)
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
