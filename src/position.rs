use crate::SIMD_LEN;
use crate::UNROLL_FACTOR;
use std::simd::cmp::SimdPartialEq;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};
use std::slice;

pub trait PositionSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn position_simd(&self, needle: T) -> Option<usize>;
}

impl<'a, T> PositionSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    fn position_simd(&self, needle: T) -> Option<usize> {
        let arr = self.as_slice();
        let (prefix, simd_data, suffix) = arr.as_simd::<SIMD_LEN>();
        // Prefix
        if let Some(pos) = prefix.iter().position(|x| *x == needle) {
            return Some(pos);
        }
        // SIMD
        let simd_needle = Simd::splat(needle);
        let mut unrolled_loops = 0;
        // Unrolled loops
        let mut chunks_iter = simd_data.chunks_exact(UNROLL_FACTOR);
        for chunks in chunks_iter.by_ref() {
            let mut mask = Mask::default();
            for chunk in chunks {
                mask |= chunk.simd_eq(simd_needle);
            }
            if mask.any() {
                for (mask_idx, c) in chunks.iter().enumerate() {
                    let mask = c.simd_eq(simd_needle);
                    if mask.any() {
                        return Some(
                            prefix.len()
                                + (unrolled_loops * (SIMD_LEN * UNROLL_FACTOR))      // Full outer loops
                                + mask_idx * SIMD_LEN                           // nth inner loop
                                + mask.to_bitmask().trailing_zeros() as usize, // nth element in matching mask
                        );
                    }
                }
            }
            unrolled_loops += 1;
        }
        // Remaining simd loops that where not divisible by UNROLL_FACTOR
        for (idx, chunk) in chunks_iter.remainder().iter().enumerate() {
            let mask = chunk.simd_eq(simd_needle).to_bitmask();
            if mask != 0 {
                return Some(
                    prefix.len()
                        + (unrolled_loops * UNROLL_FACTOR * SIMD_LEN)
                        + (idx * SIMD_LEN)
                        + (mask.trailing_zeros() as usize),
                );
            }
        }
        // Suffix
        match suffix.iter().position(|x| *x == needle) {
            Some(pos) => Some(prefix.len() + (simd_data.len() * SIMD_LEN) + pos),
            None => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::prelude::SliceRandom;
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
        for len in 0..500 {
            for _ in 0..200 {
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

                let ans = v.iter().position_simd(needle);
                let correct = v.iter().position(|x| *x == needle);

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
