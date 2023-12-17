use super::splitmix32::SplitMix32;

/// swb implementation with 8192-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-self.html).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct SWB32([u32; 256], u32, u32, usize);

crate::core32! {SWB32, self {
    self.3 = self.3.wrapping_add(1);

    let bro = self.1.min(self.2);

    self.1 = self.0[self.3.wrapping_add(34) & 255];
    self.2 = self.0[self.3.wrapping_add(19) & 255].wrapping_add(bro);

    self.0[self.3] = self.1.wrapping_sub(self.2);
    self.0[self.3]
}}

crate::seed! {SWB32, seed {
    let mut sm32 = SplitMix32::seed_from_u64(seed);

    let mut state = [0; 256];
    for i in &mut state {
        *i = sm32.next_u32();
    }

    Self(state, sm32.next_u32(), sm32.next_u32(), 0)
}}
