use crate::SIMD_LEN;
use std::simd::cmp::SimdPartialEq;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};
use std::slice;

pub trait ContainsSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn contains_simd(&self, needle: T) -> bool;
}

impl<'a, T> ContainsSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn contains_simd(&self, needle: T) -> bool {
        let arr = self.as_slice();
        let (prefix, aligned_chunks, suffix) = arr.as_simd::<SIMD_LEN>();
        // Prefix
        if prefix.contains(&needle) {
            return true;
        }
        // SIMD
        let simd_needle = Simd::splat(needle);
        for chunk in aligned_chunks {
            let mask = chunk.simd_eq(simd_needle).to_bitmask();
            if mask != 0 {
                return true;
            }
        }
        // Suffix
        suffix.contains(&needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::fmt::Debug;

    fn test_simd_contains_for_type<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
            + Debug
            + Copy
            + Default
            + SimdElement
            + std::cmp::PartialEq,
        Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
        Standard: Distribution<T>,
    {
        for len in 0..100 {
            for _ in 0..100 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }
                let needle = v.choose(&mut rng).cloned().unwrap_or(T::default());
                let ans = v.iter().contains_simd(needle);
                let correct = v.iter().contains(&needle);
                assert_eq!(
                    ans,
                    correct,
                    "Failed for length {} and type {:?}",
                    len,
                    std::any::type_name::<T>()
                );
            }
        }
    }

    #[test]
    fn test_simd_contains() {
        test_simd_contains_for_type::<i8>();
        test_simd_contains_for_type::<i16>();
        test_simd_contains_for_type::<i32>();
        test_simd_contains_for_type::<i64>();
        test_simd_contains_for_type::<u8>();
        test_simd_contains_for_type::<u16>();
        test_simd_contains_for_type::<u32>();
        test_simd_contains_for_type::<u64>();
        test_simd_contains_for_type::<usize>();
        test_simd_contains_for_type::<isize>();
        test_simd_contains_for_type::<f32>();
        test_simd_contains_for_type::<f64>();
    }
}
