// SplitMix
#[cfg(feature = "splixmix32")]
pub mod splitmix32;
#[cfg(feature = "splixmix64")]
pub mod splitmix64;

// WyRand
#[cfg(feature = "wyrand")]
pub mod wyrand;

// xorshift
#[cfg(feature = "xorshift32")]
pub mod xorshift32;
#[cfg(feature = "xorshift64")]
pub mod xorshift64;
#[cfg(feature = "xorshift64star")]
pub mod xorshift64star;
#[cfg(feature = "xorshift128")]
pub mod xorshift128;
#[cfg(feature = "xorshift128plus")]
pub mod xorshift128plus;
#[cfg(feature = "xorshift1024plus")]
pub mod xorshift1024plus;
#[cfg(feature = "xorshift1024star")]
pub mod xorshift1024star;

// other George Marsaglia PRNGs
#[cfg(feature = "kiss")]
pub mod kiss;
#[cfg(feature = "swb32")]
pub mod swb32;
#[cfg(feature = "swb64")]
pub mod swb64;

// xorwow
#[cfg(feature = "xorwow")]
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
