use crate::position::PositionSimd;
use crate::SIMD_LEN;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait FindSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn find_simd(&self, needle: T) -> Option<&'a T>;
}

impl<'a, T> FindSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn find_simd(&self, needle: T) -> Option<&'a T> {
        match self.position_simd(needle) {
            Some(idx) => Some(&self.as_slice()[idx]),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::prelude::SliceRandom;
    use rand::Rng;
    use std::fmt::Debug;

    fn test_simd_for_type<T>()
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
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }
                let mut v2: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v2.iter_mut() {
                    *x = rng.gen()
                }
                let needle = match rng.gen_bool(0.5) {
                    true => v.choose(&mut rng).cloned().unwrap_or(T::default()),
                    false => loop {
                        let n = rng.gen();
                        if !v.contains(&n) {
                            break n;
                        }
                    },
                };
                let ans = v.iter().find_simd(needle);
                let correct = v.iter().find(|x| **x == needle);

                assert_eq!(
                    ans,
                    correct,
                    "Failed for length {} and type {:?} arr: {:?} needle: {:?}",
                    len,
                    std::any::type_name::<T>(),
                    v,
                    needle
                );
            }
        }
    }

    #[test]
    fn test_simd_find() {
        test_simd_for_type::<i8>();
        test_simd_for_type::<i16>();
        test_simd_for_type::<i32>();
        test_simd_for_type::<i64>();
        test_simd_for_type::<u8>();
        test_simd_for_type::<u16>();
        test_simd_for_type::<u32>();
        test_simd_for_type::<u64>();
        test_simd_for_type::<usize>();
        test_simd_for_type::<isize>();
        test_simd_for_type::<f32>();
        test_simd_for_type::<f64>();
    }
}
