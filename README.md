# `smolcrush`

A smol RNG library for Rust.

## Features
- One dependency by default ([`rand_core`](https://crates.io/crates/rand_core))
- `no_std` and WASM support
- No alloc usage
- 100% safe

## Optional Features
- **`system-rng`** adds support for using system RNG using the [`getrandom`](https://crates.io/crates/getrandom) crate
- **`zeroize`** adds zeroing support to all RNGs using the [`zeroize`](https://crates.io/crates/zeroize) crate

## **M**inimum **S**upported **R**ust **V**ersion (MSRV)

The current MSRV is 1.60.0.
