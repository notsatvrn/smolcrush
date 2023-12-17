use super::splitmix32::SplitMix32;

/// KISS implementation with 128-bit state and 32-bit seed/output.
/// state generated from seed using splitmix32.
/// original implementation [here](http://www.cse.yorku.ca/~oz/marsaglia-self.html).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct KISS(u32, u32, u32, u32);

crate::core32! {KISS, self {
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
}}

crate::seed! {KISS, seed {
    let mut sm32 = SplitMix32::seed_from_u64(seed);

    Self(
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
        sm32.next_u32(),
    )
}}
