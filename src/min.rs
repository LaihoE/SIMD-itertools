use crate::SIMD_LEN;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait MinSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn min_simd(&self) -> Option<T>;
}

impl<'a, T> MinSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq + std::cmp::Ord,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn min_simd(&self) -> Option<T> {
        let arr = self.as_slice();
        arr.iter().cloned().min()
    }
}

/*
pub trait SimdEq<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
{
    fn min_simd(&self) -> Option<T>;
}

impl<'a, T> SimdEq<'a, T> for slice::Iter<'a, T>
where
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn min_simd(&self) -> Option<T> {
        let arr = self.as_slice();
        arr.iter().copied().min()
    }
}

#[inline(always)]
pub fn min_simd(a: &[i32]) -> Option<i32> {
    a.iter().copied().min()
}

#[inline(always)]
pub fn min_simd_autovec(a: &[i32]) -> i32 {
    a.iter().fold(i32::MAX, |a, &b| a.min(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SIMD_LEN;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::prelude::SliceRandom;
    use rand::Rng;
    use std::fmt::Debug;
    use std::simd::prelude::SimdPartialEq;
    use std::simd::Mask;
    use std::simd::Simd;
    use std::simd::SimdElement;

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
        for len in 0..70 {
            for _ in 0..200 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }

                let needle = v
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .unwrap_or_default();
                let ans = v.iter().min_simd();
                let correct = v.iter().position(|x| *x == needle);

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
*/
