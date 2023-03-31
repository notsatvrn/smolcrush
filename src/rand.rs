//use core::ops::Range;

pub trait Rand32 {
    fn from_seed_u32(seed: u32) -> Self;

    fn next_u32(&mut self) -> u32;

    #[inline(always)]
    fn next_bytes(&mut self) -> [u8; 16] {
        u128::adapt_u32(self.next_u32()).to_ne_bytes()
    }

    #[inline(always)]
    fn next<T: Output>(&mut self) -> T {
        T::adapt_u32(self.next_u32())
    }

    /*
    #[inline(always)]
    fn next_range<T: Output>(&mut self, range: Range<T>) -> T {
        
    }
    */
}

pub trait Rand64 {
    fn from_seed_u64(seed: u64) -> Self;

    fn next_u64(&mut self) -> u64;

    #[inline(always)]
    fn next_bytes(&mut self) -> [u8; 16] {
        u128::adapt_u64(self.next_u64()).to_ne_bytes()
    }

    #[inline(always)]
    fn next<T: Output>(&mut self) -> T {
        T::adapt_u64(self.next_u64())
    }

    /*
    #[inline(always)]
    fn next_range<T: Output>(&mut self, range: Range<T>) -> T {
        
    }
    */
}

pub trait Output: Sized {
    fn adapt_u64(original: u64) -> Self;
    fn adapt_u32(original: u32) -> Self;
}

const F32_U64: f32 = f32::MAX / u64::MAX as f32;
const F32_U32: f32 = f32::MAX / u32::MAX as f32;

impl Output for f32 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> f32 { original as f32 * F32_U64 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> f32 { original as f32 * F32_U32 }
}

const F64_U64: f64 = f64::MAX / u64::MAX as f64;
const F64_U32: f64 = f64::MAX / u32::MAX as f64;

impl Output for f64 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> Self { original as f64 * F64_U64 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> Self { original as f64 * F64_U32 }
}

impl Output for isize {
    #[inline(always)]
    fn adapt_u64(original: u64) -> isize {
        if cfg!(target_pointer_width = "64") {
            original as isize
        } else {
            i32::adapt_u64(original) as isize
        }
    }
    #[inline(always)]
    fn adapt_u32(original: u32) -> isize {
        if cfg!(target_pointer_width = "64") {
            i64::adapt_u32(original) as isize
        } else {
            original as isize
        }
    }
}

impl Output for i8 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> i8 { u8::adapt_u64(original) as i8 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> i8 { u8::adapt_u32(original) as i8 }
}

impl Output for i16 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> i16 { u16::adapt_u64(original) as i16 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> i16 { u16::adapt_u32(original) as i16 }
}

impl Output for i32 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> i32 { u32::adapt_u64(original) as i32 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> i32 { original as i32 }
}

impl Output for i64 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> i64 { original as i64 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> i64 { u64::adapt_u32(original) as i64 }
}

impl Output for i128 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> Self { u128::adapt_u64(original) as i128 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> Self { u128::adapt_u32(original) as i128 }
}

impl Output for usize {
    fn adapt_u64(original: u64) -> usize {
        if cfg!(target_pointer_width = "64") {
            original as usize
        } else {
            u32::adapt_u64(original) as usize
        }
    }
    fn adapt_u32(original: u32) -> usize {
        if cfg!(target_pointer_width = "64") {
            u64::adapt_u32(original) as usize
        } else {
            original as usize
        }
    }
}

const U64_U8: u64 = u64::MAX / u8::MAX as u64;
const U32_U8: u32 = u32::MAX / u8::MAX as u32;

impl Output for u8 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> u8 { (original / U64_U8) as u8 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> u8 { (original / U32_U8) as u8 }
}

const U64_U16: u64 = u64::MAX / u16::MAX as u64;
const U32_U16: u32 = u32::MAX / u16::MAX as u32;

impl Output for u16 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> u16 { (original / U64_U16) as u16 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> u16 { (original / U32_U16) as u16 }
}

const U64_U32: u64 = u64::MAX / u32::MAX as u64;

impl Output for u32 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> u32 { (original / U64_U32) as u32 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> u32 { original }
}

impl Output for u64 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> u64 { original }
    #[inline(always)]
    fn adapt_u32(original: u32) -> u64 { original as u64 * original as u64 }
}

impl Output for u128 {
    #[inline(always)]
    fn adapt_u64(original: u64) -> u128 { original as u128 * original as u128 }
    #[inline(always)]
    fn adapt_u32(original: u32) -> u128 { u128::adapt_u64(u64::adapt_u32(original)) }
}
