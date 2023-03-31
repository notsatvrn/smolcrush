use crate::pseudo::splitmix32::SplitMix32;
use crate::DEFAULT_SEED_32;
use crate::rand::Rand32;

// swb implementation with 8192-bit state and 32-bit seed/output.
// state generated from seed using splitmix32.
// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-rng.html).
pub struct SWB32([u32; 256], u32, u32, usize);

impl Rand32 for SWB32 {
    #[inline]
    fn from_seed_u32(seed: u32) -> Self {
        let mut sm32 = SplitMix32::from_seed_u32(seed);
        let mut state = [0; 256];

        for i in &mut state {
            *i = sm32.next_u32();
        }

        Self(state, sm32.next_u32(), sm32.next_u32(), 0)
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.3 = self.3.wrapping_add(1);

        let bro = self.1.min(self.2);

        self.1 = self.0[self.3.wrapping_add(34) & 255];
        self.2 = self.0[self.3.wrapping_add(19) & 255].wrapping_add(bro);

        self.0[self.3] = self.1.wrapping_sub(self.2);
        self.0[self.3]
    }
}

impl Default for SWB32 {
    fn default() -> Self {
        Self::from_seed_u32(DEFAULT_SEED_32)
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
