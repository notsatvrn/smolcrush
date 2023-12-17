use super::splitmix32::SplitMix32;

/// xorwow implementation with 192-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct Xorwow(u32, u32, u32, u32, u32, u32);

crate::core32! {Xorwow, self {
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
}}

crate::seed! {Xorwow, seed {
    let mut sm32 = SplitMix32::seed_from_u64(seed);

    Self(
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
    )
}}
