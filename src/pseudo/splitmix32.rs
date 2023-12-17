/// splitmix implementation with 32-bit state and 32-bit seed/output.
/// original implementation [here](https://stackoverflow.com/a/52056161).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct SplitMix32(u32);

crate::core32! {SplitMix32, self {
    self.0 = self.0.wrapping_add(0x9E3779B9);
    let mut z = self.0;
    z = (z ^ z.wrapping_shr(15)).wrapping_mul(0x85EBCA6B);
    z = (z ^ z.wrapping_shr(13)).wrapping_mul(0xC2B2AE35);
    z.wrapping_shr(16)
}}
crate::seed!(SplitMix32, seed { Self(seed as u32) });
