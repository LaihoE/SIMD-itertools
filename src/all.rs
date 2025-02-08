use crate::LANE_COUNT;
use multiversion::multiversion;
use std::slice;

#[multiversion(targets = "simd")]
fn all_simd_internal<F, T>(v: &[T], f: F) -> bool
where
    F: Fn(&T) -> bool,
{
    let mut chunks = v.chunks_exact(LANE_COUNT);
    for chunk in chunks.by_ref() {
        if !chunk.iter().fold(true, |acc, x| acc & f(x)) {
            return false;
        }
    }
    chunks.remainder().iter().fold(true, |acc, x| acc & f(x))
}

pub trait AllSimd<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn all_simd<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool;
}

impl<'a, T> AllSimd<'a, T> for slice::Iter<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn all_simd<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        all_simd_internal(self.as_slice(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::Rng;

    fn test_simd_for_type<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
            + Copy
            + Default
            + std::cmp::PartialEq
            + std::cmp::PartialOrd,
        Standard: Distribution<T>,
    {
        for len in 0..5000 {
            let ops = [
                |x: &T| *x == T::default(),
                |x: &T| *x != T::default(),
                |x: &T| *x < T::default(),
                |x: &T| *x > T::default(),
                |x: &T| [T::default()].contains(x),
            ];

            for op in ops {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }

                let ans = v.iter().all_simd(op);
                let correct = v.iter().all(op);
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
    fn test_simd_all() {
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
