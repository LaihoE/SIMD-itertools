use crate::position::PositionSimd;
use std::slice;

pub trait ArgminSimd<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn argmin_simd(&self) -> Option<usize>;
    fn argmin_simd_fast(&self) -> Option<usize>;
}

impl<'a, T> ArgminSimd<'a, T> for slice::Iter<'a, T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + Copy + std::cmp::Ord,
{
    fn argmin_simd(&self) -> Option<usize> {
        match self.as_slice().iter().copied().min() {
            Some(min) => self.position_simd(|x| *x == min),
            None => None,
        }
    }
    fn argmin_simd_fast(&self) -> Option<usize> {
        match self
            .as_slice()
            .iter()
            .reduce(|a, b| if a < b { a } else { b })
        {
            Some(min) => self.position_simd(|x| *x == *min),
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
            + Debug
            + Copy
            + Default
            + std::cmp::PartialEq
            + Ord,
        Standard: Distribution<T>,
    {
        for len in 0..1000 {
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }
                // normal
                let ans = v.iter().argmin_simd();
                let correct = v
                    .iter()
                    .position(|x| *x == v.iter().cloned().min().unwrap());
                assert_eq!(
                    ans,
                    correct,
                    "Failed for length {} and type {:?} {:?}",
                    len,
                    std::any::type_name::<T>(),
                    v
                );
                // fast
                let ans = v.iter().argmin_simd_fast();
                let correct = v.iter().position(|x| {
                    *x == v
                        .iter()
                        .copied()
                        .reduce(|a, b| if a < b { a } else { b })
                        .unwrap()
                });
                assert_eq!(
                    ans,
                    correct,
                    "Failed for length {} and type {:?} {:?}",
                    len,
                    std::any::type_name::<T>(),
                    v
                );
            }
        }
    }

    #[test]
    fn test_simd_min() {
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
    }
}
