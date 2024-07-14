use crate::SIMD_LEN;
use std::simd::cmp::SimdPartialEq;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};
use std::slice;

pub trait PositionSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn position_simd(&self, needle: T) -> Option<usize>;
}

impl<'a, T> PositionSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn position_simd(&self, needle: T) -> Option<usize> {
        let arr = self.as_slice();
        let (prefix, simd_data, suffix) = arr.as_simd::<SIMD_LEN>();
        // Prefix
        if let Some(pos) = prefix.iter().position(|x| *x == needle) {
            return Some(pos);
        }
        // SIMD
        let simd_needle = Simd::splat(needle);
        for (chunk_idx, chunk) in simd_data.iter().enumerate() {
            let mask = chunk.simd_eq(simd_needle).to_bitmask();
            if mask != 0 {
                // Example:
                // needle = 10
                // prefix = [1,2,3]
                // SIMD = [[4,5,6,7], [8,9,10,11]]
                // 3 + (1 * 4) + (trailing_zeros(0b0010) == 2) = 9
                return Some(
                    prefix.len() + (chunk_idx * SIMD_LEN) + (mask.trailing_zeros() as usize),
                );
            }
        }
        // Suffix
        match suffix.iter().position(|x| *x == needle) {
            Some(pos) => Some(prefix.len() + (simd_data.len() * SIMD_LEN) + pos),
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

    fn test_for_type<T>()
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
        for len in 0..1000 {
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }

                let needle = v
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .unwrap_or_default();
                let ans = v.iter().position_simd(needle);
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
    fn test_simd() {
        test_for_type::<i8>();
        test_for_type::<i16>();
        test_for_type::<i32>();
        test_for_type::<i64>();
        test_for_type::<u8>();
        test_for_type::<u16>();
        test_for_type::<u32>();
        test_for_type::<u64>();
        test_for_type::<usize>();
        test_for_type::<isize>();
        test_for_type::<f32>();
        test_for_type::<f64>();
    }
}
