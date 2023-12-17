/// xorshift implementation with 32-bit state and 32-bit seed/output.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift32(u32);

crate::core32! {XorShift32, self {
    self.0 ^= self.0.wrapping_shl(13);
    self.0 ^= self.0.wrapping_shr(17);
    self.0 ^= self.0.wrapping_shl(5);
    self.0
}}
crate::seed!(XorShift32, seed { Self(seed as u32) });
