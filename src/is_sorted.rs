use crate::SIMD_LEN;
use std::simd::cmp::SimdPartialOrd;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait IsSortedSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialOrd,
    Simd<T, SIMD_LEN>: SimdPartialOrd<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn is_sorted_simd(&self) -> bool;
}

impl<'a, T> IsSortedSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialOrd,
    Simd<T, SIMD_LEN>: SimdPartialOrd<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn is_sorted_simd(&self) -> bool {
        let a = self.as_slice();

        if a.len() <= SIMD_LEN && !a.is_empty() {
            return a.is_sorted();
        }

        let chunks_a = a.chunks_exact(SIMD_LEN);
        let chunks_b = a[1..].chunks_exact(SIMD_LEN);
        let reminder_a_is_sorted = chunks_a.remainder().iter().is_sorted();
        let reminder_b_is_sorted = chunks_b.remainder().iter().is_sorted();

        // chunk:         [1,2,3,4]
        // offset_by_one: [2,3,4,5]
        // If for all chunk[i] <= offset[i] then the slice is sorted

        for (a, b) in chunks_a.zip(chunks_b) {
            let chunk = Simd::from_slice(a);
            let chunk_offset_by_one = Simd::from_slice(b);
            if chunk.simd_gt(chunk_offset_by_one).to_bitmask() != 0 {
                return false;
            }
        }
        reminder_a_is_sorted | reminder_b_is_sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::Rng;
    use std::fmt::Debug;

    fn test_integers<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
            + Debug
            + Copy
            + Default
            + SimdElement
            + std::cmp::PartialEq
            + std::cmp::PartialOrd
            + std::cmp::Ord,
        Simd<T, SIMD_LEN>: SimdPartialOrd<Mask = Mask<T::Mask, SIMD_LEN>>,
        Standard: Distribution<T>,
    {
        for len in 0..100 {
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }

                if rng.gen_bool(0.5) {
                    v.sort();
                }

                let ans = v.iter().is_sorted();
                let correct = v.iter().is_sorted();

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
    fn test_f32() {
        for len in 0..1000 {
            for _ in 0..5 {
                let mut v: Vec<f32> = vec![0.0; len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }
                if rng.gen_bool(0.5) {
                    v.sort_floats();
                }

                let ans = v.iter().is_sorted();
                let correct = v.iter().is_sorted();

                assert_eq!(
                    ans, correct,
                    "Failed for length {} and type {:?}",
                    len, "f32"
                );
            }
        }
    }
    #[test]
    fn test_f64() {
        for len in 0..1000 {
            for _ in 0..5 {
                let mut v: Vec<f64> = vec![0.0_f64; len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }

                if rng.gen_bool(0.5) {
                    v.sort_floats();
                }

                let ans = v.iter().is_sorted();
                let correct = v.iter().is_sorted();

                assert_eq!(
                    ans, correct,
                    "Failed for length {} and type {:?}",
                    len, "f64"
                );
            }
        }
    }

    #[test]
    fn test_simd_is_sorted() {
        test_integers::<i8>();
        test_integers::<i16>();
        test_integers::<i32>();
        test_integers::<i64>();
        test_integers::<u8>();
        test_integers::<u16>();
        test_integers::<u32>();
        test_integers::<u64>();
        test_integers::<usize>();
        test_integers::<isize>();
    }
}
