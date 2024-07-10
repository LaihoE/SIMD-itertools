use crate::SIMD_LEN;
use std::simd::cmp::SimdPartialEq;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait SimdEq<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn simd_eq(&self, other: &Self) -> bool;
}

impl<'a, T> SimdEq<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn simd_eq(&self, other: &Self) -> bool {
        let a = self.as_slice();
        let b = other.as_slice();
        if a.len() != b.len() {
            return false;
        }
        let (a_prefix, a_simd_chunk, a_suffix) = a.as_simd::<SIMD_LEN>();
        let (b_prefix, b_simd_chunk, b_suffix) = b.as_simd::<SIMD_LEN>();
        // Prefix
        if a_prefix.iter().ne(b_prefix.iter()) {
            return false;
        }
        // SIMD
        for (a_simd, b_simd) in a_simd_chunk.iter().zip(b_simd_chunk) {
            // Note that we use not equal
            if a_simd.simd_ne(*b_simd).to_bitmask() != 0 {
                return false;
            }
        }
        // Suffix
        a_suffix.iter().eq(b_suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
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
        for len in 0..100 {
            for _ in 0..100 {
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

                let ans = v.iter().simd_eq(&v2.iter());
                let correct = v.iter().eq(&v2);

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
    fn test_eq() {
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
