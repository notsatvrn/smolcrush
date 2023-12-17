use super::splitmix64::SplitMix64;

/// xorshift* implementation with 1024-bit state and 64-bit seed/output.
/// state generated from seed using splitmix64.
/// original implementation [here](https://pself.di.unimi.it/xorshift1024star.c).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift1024Star([u64; 16], usize);

crate::core64! {XorShift1024Star, self {
    let s0 = self.0[self.1];
    self.1 = (self.1 + 1) & 15;
    let s1 = self.0[self.1];
    let s1 = s1 ^ s1.wrapping_shl(31);
    self.0[self.1] = s1 ^ s0 ^ s1.wrapping_shr(11) ^ s0.wrapping_shr(30);
    self.0[self.1].wrapping_mul(0x9e3779b97f4a7c13)
}}

crate::seed! {XorShift1024Star, seed {
    let mut sm64 = SplitMix64::seed_from_u64(seed);

    XorShift1024Star(
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
}}
