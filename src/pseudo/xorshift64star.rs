/// xorshift* implementation with 64-bit state and 64-bit seed/output.
/// original implementation [here](https://en.wikipedia.org/wiki/Xorshift).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct XorShift64Star(u64);

crate::core64! {XorShift64Star, self {
    self.0 ^= self.0.wrapping_shl(12);
    self.0 ^= self.0.wrapping_shr(25);
    self.0 ^= self.0.wrapping_shl(27);
    self.0.wrapping_mul(0x2545F4914F6CDD1D)
}}
crate::seed!(XorShift64Star, seed { Self(seed) });
