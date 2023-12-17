use super::splitmix32::SplitMix32;

/// xorshift implementation with 128-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift128(u32, u32, u32, u32);

crate::core32! {XorShift128, self {
    let t = self.3;
    let s = self.0;

    self.3 = self.2;
    self.2 = self.1;
    self.1 = s;

    let t = t ^ t.wrapping_shl(11);
    let t = t ^ t.wrapping_shr(8);

    self.0 = t ^ s ^ s.wrapping_shr(19);

    self.0
}}

crate::seed! {XorShift128, seed {
    let mut sm32 = SplitMix32::seed_from_u64(seed);

    Self(
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
    )
}}
