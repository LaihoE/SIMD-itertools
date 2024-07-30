use crate::position::PositionSimd;
use crate::SIMD_LEN;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait MinMaxSimd<'a, T>
where
    T: PartialOrd + Ord + Copy,
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    /// Returns Option<min_element, max_element>
    fn minmax_simd(&self) -> Option<(T, T)>;
    /// Returns Option<position_min, position_max>
    fn minmax_position_simd(&self) -> Option<(usize, usize)>;
}

impl<'a, T> MinMaxSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq + std::cmp::Ord,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn minmax_simd(&self) -> Option<(T, T)>
    where
        T: PartialOrd + Ord + Copy,
    {
        // This seems to robustly produce great assembly.
        let arr = self.as_slice();
        if arr.is_empty() {
            return None;
        }
        let first_element = *arr.first().unwrap();
        let mut smallest = first_element;
        let mut largest = first_element;
        for val in arr {
            smallest = std::cmp::min(*val, smallest);
            largest = std::cmp::max(*val, largest);
        }
        Some((smallest, largest))
    }
    fn minmax_position_simd(&self) -> Option<(usize, usize)>
    where
        T: PartialOrd + Ord + Copy,
        T: SimdElement + std::cmp::PartialEq,
        Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    {
        let arr = self.as_slice();
        if let Some((min, max)) = arr.iter().minmax_simd() {
            // This seems to do oddly well even with big N, maybe some sort of ilp?
            let pos_min = arr.iter().position_simd(min)?;
            let pos_max = arr.iter().position_simd(max)?;
            return Some((pos_min, pos_max));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SIMD_LEN;
    use itertools::Itertools;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::Rng;
    use std::fmt::Debug;
    use std::simd::prelude::SimdPartialEq;
    use std::simd::Mask;
    use std::simd::Simd;
    use std::simd::SimdElement;

    fn test_simd_for_type<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
            + Debug
            + Copy
            + Default
            + SimdElement
            + std::cmp::PartialEq
            + Ord,
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
                let ans = v.iter().minmax_simd();

                // Test against itertools
                let correct = v.iter().minmax();
                if let Some((min, max)) = ans {
                    match correct {
                        itertools::MinMaxResult::MinMax(x, b) => {
                            assert_eq!((min, max), (*x, *b));
                        }
                        itertools::MinMaxResult::OneElement(x) => {
                            assert_eq!(*x, min);
                            assert_eq!(*x, max);
                        }
                        itertools::MinMaxResult::NoElements => {
                            assert_eq!(None, ans);
                        }
                    }
                }
                // Test against manual stdlib
                let min = v.iter().min();
                let max = v.iter().max();
                match (min, max) {
                    (Some(min), Some(max)) => {
                        let pos_min = v.iter().position(|x| x == min);
                        let pos_max = v.iter().position(|x| x == max);
                        match (pos_min, pos_max) {
                            (Some(p_min), Some(p_max)) => {
                                assert_eq!(Some((p_min, p_max)), v.iter().minmax_position_simd());
                            }
                            _ => panic!("Min and max some but could not be found in arr?"),
                        }
                    }
                    (Some(_min), None) => {
                        panic!("min some but max none?");
                    }
                    (None, Some(_max)) => {
                        panic!("min some but max none?");
                    }
                    (None, None) => {
                        assert_eq!(None, ans);
                    }
                }
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
