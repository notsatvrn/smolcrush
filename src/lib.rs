//! A smol RNG library for Rust.
//!
//! ## Features
//! - One dependency by default ([`rand_core`](https://crates.io/crates/rand_core))
//! - `no_std` and WASM support
//! - No alloc usage
//! - 100% safe
//!
//! ## Optional Features
//! - **`system-rng`** adds support for using system RNG using the [`getrandom`](https://crates.io/crates/getrandom) crate
//! - **`zeroize`** adds zeroing support to all RNGs using the [`zeroize`](https://crates.io/crates/zeroize) crate
//!
//! ## **M**inimum **S**upported **R**ust **V**ersion (MSRV)
//!
//! The current MSRV is 1.60.0.

#![no_std]
#![forbid(future_incompatible, unsafe_code)]

pub const DEFAULT_SEED: u64 = 0xB0BACAFEBADDC0DE;

#[cfg(feature = "system-rng")]
pub use rand_core::OsRng as OSRand;
pub mod pseudo;

// impl macros

macro_rules! core32 {
    ($struct:ident, $sel:ident $next:block) => {
        use rand_core::{Error, RngCore};
        impl RngCore for $struct {
            #[inline]
            fn next_u32(&mut $sel) -> u32 $next

            #[inline]
            fn next_u64(&mut self) -> u64 {
                self.next_u32() as u64 | self.next_u32() as u64 >> 32
            }

            #[inline]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                use rand_core::impls::fill_bytes_via_next;
                fill_bytes_via_next(self, dest);
            }

            #[inline]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
                self.fill_bytes(dest);
                Ok(())
            }
        }
    };
}
pub(crate) use core32;

macro_rules! core64 {
    ($struct:ident, $sel:ident $next:block) => {
        use rand_core::{Error, RngCore};
        impl RngCore for $struct {
            #[inline]
            fn next_u32(&mut self) -> u32 {
                self.next_u64() as u32
            }

            #[inline]
            fn next_u64(&mut $sel) -> u64 $next

            #[inline]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                use rand_core::impls::fill_bytes_via_next;
                fill_bytes_via_next(self, dest);
            }

            #[inline]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
                self.fill_bytes(dest);
                Ok(())
            }
        }
    };
}
pub(crate) use core64;

macro_rules! seed {
    ($struct:ident, $seed:ident $block:block) => {
        use rand_core::SeedableRng;
        impl SeedableRng for $struct {
            type Seed = [u8; 8];

            #[inline]
            fn from_seed(seed: [u8; 8]) -> Self {
                let seed = u64::from_le_bytes(seed);
                Self::seed_from_u64(seed)
            }

            #[inline]
            fn seed_from_u64($seed: u64) -> Self $block
        }

        impl Default for $struct {
            #[inline]
            fn default() -> Self {
                Self::seed_from_u64(crate::DEFAULT_SEED)
            }
        }
    };
}
pub(crate) use seed;
