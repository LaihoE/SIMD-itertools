use crate::LANE_COUNT;
use std::slice;

pub trait ContainsSimd<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn contains_simd(&self, elem: &T) -> bool;
}
impl<'a, T> ContainsSimd<'a, T> for slice::Iter<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn contains_simd(&self, elem: &T) -> bool
    where
        T: PartialEq,
    {
        let mut chunks = self.as_slice().chunks_exact(LANE_COUNT);
        for chunk in chunks.by_ref() {
            if chunk.iter().fold(false, |acc, x| acc | (x == elem)) {
                return true;
            }
        }
        chunks.remainder().contains(elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::seq::SliceRandom;
    use rand::Rng;

    fn test_simd_for_type<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
            + Copy
            + Default
            + std::cmp::PartialEq,
        Standard: Distribution<T>,
    {
        for len in 0..500 {
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
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
                let ans = v.iter().contains_simd(&needle);
                let correct = v.iter().contains(&needle);
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
    fn test_simd_contains() {
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
