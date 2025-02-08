use crate::position::PositionSimd;

pub trait ArgminSimd<T> {
    fn argmin_simd(&self) -> Option<usize>;
}
pub trait ArgminSimdFast<T> {
    fn argmin_simd_fast(&self) -> Option<usize>;
}

impl<T> ArgminSimd<T> for [T]
where
    T: PartialOrd + Copy + std::cmp::Ord,
{
    /// If you are only interested in the value and do not care about the position of the smallest value:
    /// ```arr.iter().copied().min()``` should produce optimal code.
    ///
    /// WARNING:
    /// The behavior with floats differs from the one in the standard library.
    /// This function is much more performant but has slightly different behavior when comparing NaN values.
    ///
    /// the following comparison is used: ```a < b {a} else { b }```
    fn argmin_simd(&self) -> Option<usize> {
        match self.iter().cloned().min() {
            Some(min) => self.iter().position_simd(|x| *x == min), //position_autovec(self, |x| *x == max),
            None => None,
        }
    }
}

impl<T> ArgminSimdFast<T> for [T]
where
    T: PartialOrd + Copy,
{
    /// WARNING:
    /// The behavior with floats differs from the one in the standard library.
    /// This function is much more performant but has slightly different behavior when comparing NaN values.
    ///
    /// the following comparison is used: ```a < b {a} else { b }``` instead of min() (requires Ord)
    fn argmin_simd_fast(&self) -> Option<usize> {
        match self
            .iter()
            .copied()
            .reduce(|a, b| if a < b { a } else { b })
        {
            Some(min) => self.iter().position_simd(|x| *x == min), //position_autovec(self, |x| *x == max),
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
                let ans = v.argmin_simd();
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
                let ans = v.argmin_simd_fast();
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
