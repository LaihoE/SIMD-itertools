use crate::PositionSimd;
use std::slice;

pub struct SimdFilter<'a, T, F>
where
    T: std::cmp::PartialEq + Copy,
    F: Fn(&T) -> bool,
{
    pub position: usize,
    pub f: F,
    pub arr: &'a [T],
}

impl<'a, T, F> Iterator for SimdFilter<'a, T, F>
where
    T: std::cmp::PartialEq + Copy,
    F: Fn(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.arr[self.position..].iter().position_simd(&self.f) {
            Some(pos) => {
                self.position += pos + 1;
                Some(self.arr[self.position - 1])
            }
            None => None,
        }
    }
}

pub trait FilterSimd<'a, T>
where
    T: std::cmp::PartialEq + Copy,
{
    fn filter_simd<F>(&self, f: F) -> SimdFilter<'a, T, F>
    where
        F: Fn(&T) -> bool + 'a;
}

impl<'a, T> FilterSimd<'a, T> for slice::Iter<'a, T>
where
    T: std::cmp::PartialEq + Copy,
{
    /// This is the least optimal of all functions.
    /// current implementation relies on sparsity of elems.
    ///
    ///
    /// This kind of pattern is fast:
    /// ```[0,0,0,0,0,0,0,0,0,0,1,1,0,1,1,0,0,0,0,0,0]```
    ///
    /// This kind of pattern is slow (similar to scalar speed):
    /// ```[1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1]```
    ///
    /// The speed comes from checking if a chunks contains any wanted element.
    ///
    ///
    ///  ```(0..10000).collect_vec().iter().filter_simd(|x| *x % 100 == 0).collect::<Vec<i32>>()```
    /// is ~4x faster on x86 with avx2
    ///
    ///  ```(0..10000).collect_vec().iter().filter_simd(|x| *x % 10 == 0).collect::<Vec<i32>>()```
    /// is ~2x faster on x86 with avx2
    ///
    ///```(0..10000).collect_vec().iter().filter_simd(|x| *x % 1 == 0).collect::<Vec<i32>>()```
    /// is 30% slower than scalar on x86 with avx2
    ///
    /// Something like this works well on all patterns on x86:
    fn filter_simd<F>(&self, f: F) -> SimdFilter<'a, T, F>
    where
        F: Fn(&T) -> bool + 'a,
    {
        SimdFilter {
            position: 0,
            f,
            arr: self.as_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::Rng;
    use std::fmt::Debug;

    use crate::FilterSimd;

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

                let ans = v.iter().filter_simd(op_simd).collect_vec();
                let correct = v.iter().filter(op_scalar).cloned().collect_vec();
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
