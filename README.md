# `tinycrush`

a very tiny, very cute RNG library.

## info
- zero dependencies (by default)
- `no_std` and WASM compatible
- no alloc usage
- 100% safe

## optional features
- `system-rng` adds support for using system RNG, which uses the [`getrandom`](https://crates.io/crates/getrandom) crate.
- `zeroize` adds zeroing support to all RNGs, which uses the [`zeroize`](https://crates.io/crates/zeroize) crate.
