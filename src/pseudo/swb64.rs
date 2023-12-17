use super::splitmix64::SplitMix64;

/// swb implementation with 8192-bit state and 64-bit seed/output.
/// state generated from seed using splitmix64.
/// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-self.html).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct SWB64([u64; 256], u64, u64, usize);

crate::core64! {SWB64, self {
    self.3 = self.3.wrapping_add(1);

    let bro = self.1.min(self.2);

    self.1 = self.0[self.3.wrapping_add(34) & 255];
    self.2 = self.0[self.3.wrapping_add(19) & 255].wrapping_add(bro);

    self.0[self.3] = self.1.wrapping_sub(self.2);
    self.0[self.3]
}}

crate::seed! {SWB64, seed {
    let mut sm64 = SplitMix64::seed_from_u64(seed);
    let mut state = [0; 256];

    for i in &mut state {
        *i = sm64.next_u64();
    }

    Self(state, sm64.next_u64(), sm64.next_u64(), 0)
}}
