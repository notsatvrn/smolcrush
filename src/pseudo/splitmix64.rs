/// splitmix implementation with 64-bit state and 64-bit seed/output.
/// original implementation [here](https://pself.di.unimi.it/splitmix64.c).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct SplitMix64(u64);

crate::core64! {SplitMix64, self {
    self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = self.0;
    z = (z ^ z.wrapping_shr(30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ z.wrapping_shr(27)).wrapping_mul(0x94D049BB133111EB);
    z ^ z.wrapping_shr(31)
}}
crate::seed!(SplitMix64, seed { Self(seed) });
