pub trait Rand: Sized {
    // seeding

    #[inline]
    fn seed_from_u32(seed: u32) -> Self {
        Self::seed_from_u64(seed as u64 | seed as u64 >> 32)
    }
    #[inline]
    fn seed_from_u64(seed: u64) -> Self {
        Self::seed_from_u32(seed as u32)
    }

    // next basic integer types

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.next_u32() as u64 | self.next_u32() as u64 >> 32
    }
    #[inline]
    fn next_u128(&mut self) -> u128 {
        self.next_u64() as u128 | self.next_u64() as u128 >> 64
    }

    // next advanced

    #[inline]
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        let mut i: usize = 0;

        while i < bytes.len() {
            bytes[i] = self.next_u32() as u8;
            i += 1;
        }
    }

    #[inline]
    fn next<T: Output>(&mut self) -> T {
        T::from_rand(self)
    }
}

pub trait Output: Sized {
    fn from_rand<T: Rand>(rand: &mut T) -> Self;
}

pub(crate) const F32_EXACT_MAX: u64 = 2e+23 as u64 - 1;
pub(crate) const F64_EXACT_MAX: u128 = 2e+53 as u128 - 1;

impl Output for f32 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self {
        ((rand.next_u32() as u64 * F32_EXACT_MAX) / u32::MAX as u64) as f32
    }
}

impl Output for f64 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self {
        ((rand.next_u64() as u128 * F64_EXACT_MAX) / u64::MAX as u128) as f64
    }
}

impl Output for isize {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self {
        if cfg!(target_pointer_width = "64") {
            rand.next_u64() as isize
        } else {
            rand.next_u32() as isize
        }
    }
}

impl Output for i8 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u32() as i8 }
}

impl Output for i16 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u32() as i16 }
}

impl Output for i32 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u32() as i32 }
}

impl Output for i64 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u64() as i64 }
}

impl Output for i128 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u128() as i128 }
}

impl Output for usize {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self {
        if cfg!(target_pointer_width = "64") {
            rand.next_u64() as usize
        } else {
            rand.next_u32() as usize
        }
    }
}

impl Output for u8 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u32() as u8 }
}

impl Output for u16 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u32() as u16 }
}

impl Output for u32 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u32() }
}

impl Output for u64 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u64() }
}

impl Output for u128 {
    #[inline]
    fn from_rand<T: Rand>(rand: &mut T) -> Self { rand.next_u128() }
}
