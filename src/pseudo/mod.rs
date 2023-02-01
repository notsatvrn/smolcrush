// SplitMix
pub mod splitmix32;
pub mod splitmix64;

// WyRand
pub mod wyrand64;

// xorshift
pub mod xorshift32;
pub mod xorshift64;
pub mod xorshift64star;
pub mod xorshift128;
pub mod xorshift128plus;
pub mod xorshift1024plus;
pub mod xorshift1024star;

// other George Marsaglia PRNGs
pub mod kiss;
pub mod swb32;
pub mod swb64;

// xorwow
pub mod xorwow;

/*
#[cfg(test)]
mod tests {
    use super::splitmix64::SplitMix64;
    use crate::rand::Rand64;

    #[test]
    fn range() {
        let mut rng = SplitMix64::default();

        for _ in 0..5 {
            let v = rng.next_range::<f32>(-10.0..10.0);
            println!("{}", v);
            assert!(10.0 >= v && v >= -10.0);
        }
    }
}
*/
