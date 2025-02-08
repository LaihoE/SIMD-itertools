use crate::position::PositionSimd;
use std::slice;

pub trait FindSimd<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn find_simd<F>(&self, f: F) -> Option<&'a T>
    where
        F: Fn(&T) -> bool + 'a;
}

impl<'a, T> FindSimd<'a, T> for slice::Iter<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn find_simd<F>(&self, f: F) -> Option<&'a T>
    where
        F: Fn(&T) -> bool + 'a,
    {
        match self.position_simd(f) {
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
    use rand::Rng;
    use std::fmt::Debug;

    fn test_simd_for_type<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
            + Copy
            + Default
            + Debug
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
            let ops2 = [
                |x: &&T| **x == T::default(),
                |x: &&T| **x != T::default(),
                |x: &&T| **x < T::default(),
                |x: &&T| **x > T::default(),
                |x: &&T| [T::default()].contains(x),
            ];

            for (op_simd, op_scalar) in ops.iter().zip(ops2) {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }

                let ans = v.iter().find(op_scalar);
                let correct = v.iter().find_simd(op_simd);
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
