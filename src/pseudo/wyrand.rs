/// wyrand implementation with 64-bit state and 64-bit seed/output.
/// original implementation [here](https://github.com/wangyi-fudan/wyhash/blob/master/wyhash.h).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
pub struct WyRand(u64);

crate::core64! {WyRand, self {
    self.0 = self.0.wrapping_add(0xA0761D6478BD642F);
    let y = (self.0 as u128).wrapping_mul((self.0 as u128) ^ 0xE7037ED1A0B428DB);
    (y.wrapping_shr(64) ^ y) as u64
}}
crate::seed!(WyRand, seed { Self(seed) });
