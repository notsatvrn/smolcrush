use super::splitmix64::SplitMix64;

/// xorshift+ implementation with 128-bit state and 64-bit seed/output.
/// state generated from seed using splitmix64.
/// original implementation [here](https://pself.di.unimi.it/xorshift128plus.c).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift128Plus(u64, u64);

crate::core64! {XorShift128Plus, self {
    let s1 = self.0;
    let s0 = self.1;

    let result = s0.wrapping_add(s1);

    self.0 = s0;
    let s1 = s1 ^ s1.wrapping_shl(23);
    self.1 = s1 ^ s0 ^ s1.wrapping_shr(18) ^ s0.wrapping_shr(5);

    result
}}

crate::seed! {XorShift128Plus, seed {
    let mut sm64 = SplitMix64::seed_from_u64(seed);
    Self(sm64.next_u64(), sm64.next_u64())
}}
