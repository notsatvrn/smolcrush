/// xorshift implementation with 64-bit state and 64-bit seed/output.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift64(u64);

crate::core64! {XorShift64, self {
    self.0 ^= self.0.wrapping_shl(13);
    self.0 ^= self.0.wrapping_shr(7);
    self.0 ^= self.0.wrapping_shl(17);
    self.0
}}
crate::seed!(XorShift64, seed { Self(seed) });
