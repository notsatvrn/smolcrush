#![no_std]
#![forbid(unsafe_code)]

pub const DEFAULT_SEED_32: u32 = 0xB0BACAFE;
pub const DEFAULT_SEED_64: u64 = 0xB0BACAFEBADDC0DE;

pub mod rand;
pub mod pseudo;
