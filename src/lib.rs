//! A cute and smol RNG library.
//!
//! ## Features
//! - Zero dependencies (by default).
//! - `no_std` and WASM compatible.
//! - No alloc usage.
//! - 100% safe.
//!
//! ## Optional Features
//! - `system-rng` adds support for using system RNG, which uses the [`getrandom`](https://crates.io/crates/getrandom) crate.
//! - `zeroize` adds zeroing support to all RNGs, which uses the [`zeroize`](https://crates.io/crates/zeroize) crate.
//!
//! ## **M**inimum **S**upported **R**ust **V**ersion (MSRV)
//!
//! The current MSRV is 1.60.0.

#![no_std]
#![forbid(unsafe_code)]

pub const DEFAULT_SEED_32: u32 = 0xB0BACAFE;
pub const DEFAULT_SEED_64: u64 = 0xB0BACAFEBADDC0DE;

pub mod rand;
pub mod pseudo;
