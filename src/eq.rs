use crate::SIMD_LEN;
use crate::UNROLL_FACTOR;
use std::simd::cmp::SimdPartialEq;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait EqSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn eq_simd(&self, other: &Self) -> bool;
}

impl<'a, T> EqSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn eq_simd(&self, other: &Self) -> bool {
        let a = self.as_slice();
        let b = other.as_slice();
        if a.len() != b.len() {
            return false;
        }

        let mut chunks_a = a.chunks_exact(SIMD_LEN * UNROLL_FACTOR);
        let mut chunks_b = b.chunks_exact(SIMD_LEN * UNROLL_FACTOR);
        let mut mask = Mask::default();

        for (aa, bb) in chunks_a.by_ref().zip(chunks_b.by_ref()) {
            for (aaa, bbb) in aa.chunks_exact(SIMD_LEN).zip(bb.chunks_exact(SIMD_LEN)) {
                mask |= Simd::from_slice(aaa).simd_ne(Simd::from_slice(bbb));
            }
            if mask.any() {
                return false;
            }
        }
        return chunks_a.remainder().eq(chunks_b.remainder());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::Rng;

    fn test_for_type<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
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
                let mut v2: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v2.iter_mut() {
                    *x = rng.gen()
                }

                let ans = v.iter().eq_simd(&v2.iter());
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
    fn test_for_type_equal_values<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
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
                let v2 = v.clone();
                let ans = v.iter().eq_simd(&v2.iter());
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
    fn test_reminder_eq() {
        let a = [0; SIMD_LEN + 4];
        let b = [0; SIMD_LEN + 4];
        let expected = a.into_iter().eq(b);
        let got = a.iter().eq_simd(&b.iter());
        assert_eq!(expected, got);
    }
    #[test]
    fn test_a_and_b_unequal_split() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let expected = a.into_iter().eq(b);
        let got = a.iter().eq_simd(&b.iter());
        assert_eq!(expected, got);
    }
    #[test]
    fn test_eq_simd() {
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

        test_for_type_equal_values::<i8>();
        test_for_type_equal_values::<i16>();
        test_for_type_equal_values::<i32>();
        test_for_type_equal_values::<i64>();
        test_for_type_equal_values::<u8>();
        test_for_type_equal_values::<u16>();
        test_for_type_equal_values::<u32>();
        test_for_type_equal_values::<u64>();
        test_for_type_equal_values::<usize>();
        test_for_type_equal_values::<isize>();
        test_for_type_equal_values::<f32>();
        test_for_type_equal_values::<f64>();
    }
}
