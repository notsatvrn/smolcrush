//use core::ops::{Add, Div, Range};

pub trait Rand32 {
    fn from_seed_u32(seed: u32) -> Self;

    fn next_u32(&mut self) -> u32;

    #[inline(always)]
    fn next<T: Output>(&mut self) -> T {
        T::convert_u32(self.next_u32())
    }

    /*
    #[inline(always)]
    fn next_range<T>(&mut self, range: Range<T>) -> T
    where
        T: Output + Add<Output = T> + Div<Output = T>
    {
        compute_range(self.next::<T>(), T::bounds(), range)
    }
    */
}

pub trait Rand64 {
    fn from_seed_u64(seed: u64) -> Self;

    fn next_u64(&mut self) -> u64;

    #[inline(always)]
    fn next<T: Output>(&mut self) -> T {
        T::convert_u64(self.next_u64())
    }

    /*
    #[inline(always)]
    fn next_range<T>(&mut self, range: Range<T>) -> T
    where
        T: Output + Add<Output = T> + Div<Output = T>
    {
        compute_range(self.next::<T>(), T::bounds(), range)
    }
    */
}

/*
#[inline(always)]
fn compute_range<T>(
    value: T,
    bounds: (T, T),
    range: Range<T>,
) -> T
where
    T: Output + Add<Output = T> + Div<Output = T>
{
    range.start + value / (bounds.1 / range.end)
}
*/

pub trait Output: Sized {
    fn convert_u64(original: u64) -> Self;
    fn convert_u32(original: u32) -> Self;
    fn bounds() -> (Self, Self);
}

impl Output for f32 {
    fn convert_u64(original: u64) -> f32 { original as f32 * (f32::MAX / u64::MAX as f32) }
    fn convert_u32(original: u32) -> f32 { original as f32 * (f32::MAX / u32::MAX as f32) }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for f64 {
    fn convert_u64(original: u64) -> Self { original as f64 * (f64::MAX / u64::MAX as f64) }
    fn convert_u32(original: u32) -> Self { original as f64 * (f64::MAX / u32::MAX as f64) }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for isize {
    fn convert_u64(original: u64) -> isize {
        if isize::MAX as i64 == i64::MAX {
            return i64::convert_u64(original) as isize;
        }

        i32::convert_u64(original) as isize
    }
    fn convert_u32(original: u32) -> isize {
        if isize::MAX as i64 == i64::MAX {
            return i64::convert_u32(original) as isize;
        }

        i32::convert_u32(original) as isize
    }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for i8 {
    fn convert_u64(original: u64) -> i8 { u8::convert_u64(original) as i8 }
    fn convert_u32(original: u32) -> i8 { u8::convert_u32(original) as i8 }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for i16 {
    fn convert_u64(original: u64) -> i16 { u16::convert_u64(original) as i16 }
    fn convert_u32(original: u32) -> i16 { u16::convert_u32(original) as i16 }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for i32 {
    fn convert_u64(original: u64) -> i32 { u32::convert_u64(original) as i32 }
    fn convert_u32(original: u32) -> i32 { original as i32 }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for i64 {
    fn convert_u64(original: u64) -> i64 { original as i64 }
    fn convert_u32(original: u32) -> i64 { u64::convert_u32(original) as i64 }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for i128 {
    fn convert_u64(original: u64) -> Self { u128::convert_u64(original) as i128 }
    fn convert_u32(original: u32) -> Self { u128::convert_u32(original) as i128 }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for usize {
    fn convert_u64(original: u64) -> usize {
        if usize::MAX as u64 == u64::MAX {
            return original as usize;
        }

        u32::convert_u64(original) as usize
    }
    fn convert_u32(original: u32) -> usize {
        if usize::MAX as u64 == u64::MAX {
            return u64::convert_u32(original) as usize;
        }

        original as usize
    }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for u8 {
    fn convert_u64(original: u64) -> u8 { (original / (u64::MAX / u8::MAX as u64)) as u8 }
    fn convert_u32(original: u32) -> u8 { (original / (u32::MAX / u8::MAX as u32)) as u8 }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for u16 {
    fn convert_u64(original: u64) -> u16 { (original / (u64::MAX / u16::MAX as u64)) as u16 }
    fn convert_u32(original: u32) -> u16 { (original / (u32::MAX / u16::MAX as u32)) as u16 }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for u32 {
    fn convert_u64(original: u64) -> u32 { (original / (u64::MAX / u32::MAX as u64)) as u32 }
    fn convert_u32(original: u32) -> u32 { original }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for u64 {
    fn convert_u64(original: u64) -> u64 { original }
    fn convert_u32(original: u32) -> u64 { original as u64 * (u64::MAX / u32::MAX as u64) }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}

impl Output for u128 {
    fn convert_u64(original: u64) -> u128 { original as u128 * (u128::MAX / u64::MAX as u128) } 
    fn convert_u32(original: u32) -> u128 { original as u128 * (u128::MAX / u32::MAX as u128) }
    fn bounds() -> (Self, Self) { (Self::MIN, Self::MAX) }
}
