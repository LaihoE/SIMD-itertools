use crate::SIMD_LEN;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait MaxSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn max_simd(&self) -> Option<T>;
}

impl<'a, T> MaxSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq + std::cmp::Ord,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn max_simd(&self) -> Option<T> {
        let arr = self.as_slice();
        arr.iter().cloned().max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max() {
        for len in 0..100 {
            for _ in 0..100 {
                let mut v1 = vec![0; len];
                for x in v1.iter_mut() {
                    *x = rand::random()
                }

                let correct = v1.iter().max();
                let ans = &v1.iter().max_simd();
                assert_eq!(ans.clone(), correct.cloned());
            }
        }
    }
}
