use std::simd::cmp::SimdPartialOrd;
use std::simd::prelude::SimdPartialEq;
use std::simd::usizex8;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::slice;

pub trait FilterSimd<'a, T>
where
    T: SimdElement + std::cmp::PartialEq + std::cmp::PartialOrd,
    Simd<T, 8>: SimdPartialEq<Mask = Mask<T::Mask, 8>>,
{
    fn filter_simd_lt(&self, needle: T) -> Vec<T>;
    fn filter_simd_gt(&self, needle: T) -> Vec<T>;
    fn filter_simd_eq(&self, needle: T) -> Vec<T>;
}

// TODO REMOVE DUPLICATION?
impl<'a, T> FilterSimd<'a, T> for slice::Iter<'a, T>
where
    T: SimdElement + std::cmp::PartialEq + std::cmp::PartialOrd + Default,
    Simd<T, 8>: SimdPartialOrd<Mask = Mask<T::Mask, 8>>,
{
    fn filter_simd_lt(&self, needle: T) -> Vec<T> {
        let a = self.as_slice();
        let mut indicies = vec![];
        let (prefix, simd_chunk, suffix) = a.as_simd::<8>();
        let prefix_filters = prefix.iter().filter(|x| **x < needle);

        indicies.extend(prefix_filters);
        let prefix_filters_len = indicies.len();
        indicies.resize(std::cmp::max(prefix_filters_len, 64), T::default());
        let simd_needle = Simd::splat(needle);
        let mut simd_idx = prefix_filters_len;

        // SIMD
        for chunk in simd_chunk {
            let x = chunk.simd_lt(simd_needle);
            let bitmask = x.to_bitmask();
            if bitmask != 0 {
                let idxs = SET_BITS_TO_INDICIES[bitmask as usize];
                chunk.scatter(&mut indicies[simd_idx..], idxs);
                simd_idx += bitmask.count_ones() as usize;
                if simd_idx <= indicies.len() {
                    indicies.resize(indicies.len() + 64, T::default());
                }
            }
        }

        indicies.truncate(simd_idx);
        let suffix_filters = suffix.iter().filter(|x| **x < needle);
        indicies.extend(suffix_filters);
        indicies
    }
    fn filter_simd_gt(&self, needle: T) -> Vec<T> {
        let a = self.as_slice();
        let mut indicies = vec![];
        let (prefix, simd_chunk, suffix) = a.as_simd::<8>();
        let prefix_filters = prefix.iter().filter(|x| **x > needle);

        indicies.extend(prefix_filters);
        let prefix_filters_len = indicies.len();
        indicies.resize(std::cmp::max(prefix_filters_len, 64), T::default());
        let simd_needle = Simd::splat(needle);
        let mut simd_idx = prefix_filters_len;

        // SIMD
        for chunk in simd_chunk {
            let x = chunk.simd_gt(simd_needle);
            let bitmask = x.to_bitmask();
            if bitmask != 0 {
                let idxs = SET_BITS_TO_INDICIES[bitmask as usize];
                chunk.scatter(&mut indicies[simd_idx..], idxs);
                simd_idx += bitmask.count_ones() as usize;

                if simd_idx <= indicies.len() {
                    indicies.resize(indicies.len() + 64, T::default());
                }
            }
        }

        indicies.truncate(simd_idx);
        let suffix_filters = suffix.iter().filter(|x| **x > needle);
        indicies.extend(suffix_filters);
        indicies
    }
    fn filter_simd_eq(&self, needle: T) -> Vec<T> {
        let a = self.as_slice();
        let mut indicies = vec![];
        let (prefix, simd_chunk, suffix) = a.as_simd::<8>();
        let prefix_filters = prefix.iter().filter(|x| **x == needle);

        indicies.extend(prefix_filters);
        let prefix_filters_len = indicies.len();
        indicies.resize(std::cmp::max(prefix_filters_len, 64), T::default());
        let simd_needle = Simd::splat(needle);
        let mut simd_idx = prefix_filters_len;

        // SIMD
        for chunk in simd_chunk {
            let x = chunk.simd_eq(simd_needle);
            let bitmask = x.to_bitmask();
            if bitmask != 0 {
                let idxs = SET_BITS_TO_INDICIES[bitmask as usize];
                chunk.scatter(&mut indicies[simd_idx..], idxs);
                simd_idx += bitmask.count_ones() as usize;
                if simd_idx <= indicies.len() {
                    indicies.resize(indicies.len() + 64, T::default());
                }
            }
        }

        indicies.truncate(simd_idx);
        let suffix_filters = suffix.iter().filter(|x| **x == needle);
        indicies.extend(suffix_filters);
        indicies
    }
}

const SET_BITS_TO_INDICIES: [usizex8; 256] = [
    usizex8::from_array([255, 255, 255, 255, 255, 255, 255, 255]),
    usizex8::from_array([0, 255, 255, 255, 255, 255, 255, 255]),
    usizex8::from_array([255, 0, 255, 255, 255, 255, 255, 255]),
    usizex8::from_array([0, 1, 255, 255, 255, 255, 255, 255]),
    usizex8::from_array([255, 255, 0, 255, 255, 255, 255, 255]),
    usizex8::from_array([0, 255, 1, 255, 255, 255, 255, 255]),
    usizex8::from_array([255, 0, 1, 255, 255, 255, 255, 255]),
    usizex8::from_array([0, 1, 2, 255, 255, 255, 255, 255]),
    usizex8::from_array([255, 255, 255, 0, 255, 255, 255, 255]),
    usizex8::from_array([0, 255, 255, 1, 255, 255, 255, 255]),
    usizex8::from_array([255, 0, 255, 1, 255, 255, 255, 255]),
    usizex8::from_array([0, 1, 255, 2, 255, 255, 255, 255]),
    usizex8::from_array([255, 255, 0, 1, 255, 255, 255, 255]),
    usizex8::from_array([0, 255, 1, 2, 255, 255, 255, 255]),
    usizex8::from_array([255, 0, 1, 2, 255, 255, 255, 255]),
    usizex8::from_array([0, 1, 2, 3, 255, 255, 255, 255]),
    usizex8::from_array([255, 255, 255, 255, 0, 255, 255, 255]),
    usizex8::from_array([0, 255, 255, 255, 1, 255, 255, 255]),
    usizex8::from_array([255, 0, 255, 255, 1, 255, 255, 255]),
    usizex8::from_array([0, 1, 255, 255, 2, 255, 255, 255]),
    usizex8::from_array([255, 255, 0, 255, 1, 255, 255, 255]),
    usizex8::from_array([0, 255, 1, 255, 2, 255, 255, 255]),
    usizex8::from_array([255, 0, 1, 255, 2, 255, 255, 255]),
    usizex8::from_array([0, 1, 2, 255, 3, 255, 255, 255]),
    usizex8::from_array([255, 255, 255, 0, 1, 255, 255, 255]),
    usizex8::from_array([0, 255, 255, 1, 2, 255, 255, 255]),
    usizex8::from_array([255, 0, 255, 1, 2, 255, 255, 255]),
    usizex8::from_array([0, 1, 255, 2, 3, 255, 255, 255]),
    usizex8::from_array([255, 255, 0, 1, 2, 255, 255, 255]),
    usizex8::from_array([0, 255, 1, 2, 3, 255, 255, 255]),
    usizex8::from_array([255, 0, 1, 2, 3, 255, 255, 255]),
    usizex8::from_array([0, 1, 2, 3, 4, 255, 255, 255]),
    usizex8::from_array([255, 255, 255, 255, 255, 0, 255, 255]),
    usizex8::from_array([0, 255, 255, 255, 255, 1, 255, 255]),
    usizex8::from_array([255, 0, 255, 255, 255, 1, 255, 255]),
    usizex8::from_array([0, 1, 255, 255, 255, 2, 255, 255]),
    usizex8::from_array([255, 255, 0, 255, 255, 1, 255, 255]),
    usizex8::from_array([0, 255, 1, 255, 255, 2, 255, 255]),
    usizex8::from_array([255, 0, 1, 255, 255, 2, 255, 255]),
    usizex8::from_array([0, 1, 2, 255, 255, 3, 255, 255]),
    usizex8::from_array([255, 255, 255, 0, 255, 1, 255, 255]),
    usizex8::from_array([0, 255, 255, 1, 255, 2, 255, 255]),
    usizex8::from_array([255, 0, 255, 1, 255, 2, 255, 255]),
    usizex8::from_array([0, 1, 255, 2, 255, 3, 255, 255]),
    usizex8::from_array([255, 255, 0, 1, 255, 2, 255, 255]),
    usizex8::from_array([0, 255, 1, 2, 255, 3, 255, 255]),
    usizex8::from_array([255, 0, 1, 2, 255, 3, 255, 255]),
    usizex8::from_array([0, 1, 2, 3, 255, 4, 255, 255]),
    usizex8::from_array([255, 255, 255, 255, 0, 1, 255, 255]),
    usizex8::from_array([0, 255, 255, 255, 1, 2, 255, 255]),
    usizex8::from_array([255, 0, 255, 255, 1, 2, 255, 255]),
    usizex8::from_array([0, 1, 255, 255, 2, 3, 255, 255]),
    usizex8::from_array([255, 255, 0, 255, 1, 2, 255, 255]),
    usizex8::from_array([0, 255, 1, 255, 2, 3, 255, 255]),
    usizex8::from_array([255, 0, 1, 255, 2, 3, 255, 255]),
    usizex8::from_array([0, 1, 2, 255, 3, 4, 255, 255]),
    usizex8::from_array([255, 255, 255, 0, 1, 2, 255, 255]),
    usizex8::from_array([0, 255, 255, 1, 2, 3, 255, 255]),
    usizex8::from_array([255, 0, 255, 1, 2, 3, 255, 255]),
    usizex8::from_array([0, 1, 255, 2, 3, 4, 255, 255]),
    usizex8::from_array([255, 255, 0, 1, 2, 3, 255, 255]),
    usizex8::from_array([0, 255, 1, 2, 3, 4, 255, 255]),
    usizex8::from_array([255, 0, 1, 2, 3, 4, 255, 255]),
    usizex8::from_array([0, 1, 2, 3, 4, 5, 255, 255]),
    usizex8::from_array([255, 255, 255, 255, 255, 255, 0, 255]),
    usizex8::from_array([0, 255, 255, 255, 255, 255, 1, 255]),
    usizex8::from_array([255, 0, 255, 255, 255, 255, 1, 255]),
    usizex8::from_array([0, 1, 255, 255, 255, 255, 2, 255]),
    usizex8::from_array([255, 255, 0, 255, 255, 255, 1, 255]),
    usizex8::from_array([0, 255, 1, 255, 255, 255, 2, 255]),
    usizex8::from_array([255, 0, 1, 255, 255, 255, 2, 255]),
    usizex8::from_array([0, 1, 2, 255, 255, 255, 3, 255]),
    usizex8::from_array([255, 255, 255, 0, 255, 255, 1, 255]),
    usizex8::from_array([0, 255, 255, 1, 255, 255, 2, 255]),
    usizex8::from_array([255, 0, 255, 1, 255, 255, 2, 255]),
    usizex8::from_array([0, 1, 255, 2, 255, 255, 3, 255]),
    usizex8::from_array([255, 255, 0, 1, 255, 255, 2, 255]),
    usizex8::from_array([0, 255, 1, 2, 255, 255, 3, 255]),
    usizex8::from_array([255, 0, 1, 2, 255, 255, 3, 255]),
    usizex8::from_array([0, 1, 2, 3, 255, 255, 4, 255]),
    usizex8::from_array([255, 255, 255, 255, 0, 255, 1, 255]),
    usizex8::from_array([0, 255, 255, 255, 1, 255, 2, 255]),
    usizex8::from_array([255, 0, 255, 255, 1, 255, 2, 255]),
    usizex8::from_array([0, 1, 255, 255, 2, 255, 3, 255]),
    usizex8::from_array([255, 255, 0, 255, 1, 255, 2, 255]),
    usizex8::from_array([0, 255, 1, 255, 2, 255, 3, 255]),
    usizex8::from_array([255, 0, 1, 255, 2, 255, 3, 255]),
    usizex8::from_array([0, 1, 2, 255, 3, 255, 4, 255]),
    usizex8::from_array([255, 255, 255, 0, 1, 255, 2, 255]),
    usizex8::from_array([0, 255, 255, 1, 2, 255, 3, 255]),
    usizex8::from_array([255, 0, 255, 1, 2, 255, 3, 255]),
    usizex8::from_array([0, 1, 255, 2, 3, 255, 4, 255]),
    usizex8::from_array([255, 255, 0, 1, 2, 255, 3, 255]),
    usizex8::from_array([0, 255, 1, 2, 3, 255, 4, 255]),
    usizex8::from_array([255, 0, 1, 2, 3, 255, 4, 255]),
    usizex8::from_array([0, 1, 2, 3, 4, 255, 5, 255]),
    usizex8::from_array([255, 255, 255, 255, 255, 0, 1, 255]),
    usizex8::from_array([0, 255, 255, 255, 255, 1, 2, 255]),
    usizex8::from_array([255, 0, 255, 255, 255, 1, 2, 255]),
    usizex8::from_array([0, 1, 255, 255, 255, 2, 3, 255]),
    usizex8::from_array([255, 255, 0, 255, 255, 1, 2, 255]),
    usizex8::from_array([0, 255, 1, 255, 255, 2, 3, 255]),
    usizex8::from_array([255, 0, 1, 255, 255, 2, 3, 255]),
    usizex8::from_array([0, 1, 2, 255, 255, 3, 4, 255]),
    usizex8::from_array([255, 255, 255, 0, 255, 1, 2, 255]),
    usizex8::from_array([0, 255, 255, 1, 255, 2, 3, 255]),
    usizex8::from_array([255, 0, 255, 1, 255, 2, 3, 255]),
    usizex8::from_array([0, 1, 255, 2, 255, 3, 4, 255]),
    usizex8::from_array([255, 255, 0, 1, 255, 2, 3, 255]),
    usizex8::from_array([0, 255, 1, 2, 255, 3, 4, 255]),
    usizex8::from_array([255, 0, 1, 2, 255, 3, 4, 255]),
    usizex8::from_array([0, 1, 2, 3, 255, 4, 5, 255]),
    usizex8::from_array([255, 255, 255, 255, 0, 1, 2, 255]),
    usizex8::from_array([0, 255, 255, 255, 1, 2, 3, 255]),
    usizex8::from_array([255, 0, 255, 255, 1, 2, 3, 255]),
    usizex8::from_array([0, 1, 255, 255, 2, 3, 4, 255]),
    usizex8::from_array([255, 255, 0, 255, 1, 2, 3, 255]),
    usizex8::from_array([0, 255, 1, 255, 2, 3, 4, 255]),
    usizex8::from_array([255, 0, 1, 255, 2, 3, 4, 255]),
    usizex8::from_array([0, 1, 2, 255, 3, 4, 5, 255]),
    usizex8::from_array([255, 255, 255, 0, 1, 2, 3, 255]),
    usizex8::from_array([0, 255, 255, 1, 2, 3, 4, 255]),
    usizex8::from_array([255, 0, 255, 1, 2, 3, 4, 255]),
    usizex8::from_array([0, 1, 255, 2, 3, 4, 5, 255]),
    usizex8::from_array([255, 255, 0, 1, 2, 3, 4, 255]),
    usizex8::from_array([0, 255, 1, 2, 3, 4, 5, 255]),
    usizex8::from_array([255, 0, 1, 2, 3, 4, 5, 255]),
    usizex8::from_array([0, 1, 2, 3, 4, 5, 6, 255]),
    usizex8::from_array([255, 255, 255, 255, 255, 255, 255, 0]),
    usizex8::from_array([0, 255, 255, 255, 255, 255, 255, 1]),
    usizex8::from_array([255, 0, 255, 255, 255, 255, 255, 1]),
    usizex8::from_array([0, 1, 255, 255, 255, 255, 255, 2]),
    usizex8::from_array([255, 255, 0, 255, 255, 255, 255, 1]),
    usizex8::from_array([0, 255, 1, 255, 255, 255, 255, 2]),
    usizex8::from_array([255, 0, 1, 255, 255, 255, 255, 2]),
    usizex8::from_array([0, 1, 2, 255, 255, 255, 255, 3]),
    usizex8::from_array([255, 255, 255, 0, 255, 255, 255, 1]),
    usizex8::from_array([0, 255, 255, 1, 255, 255, 255, 2]),
    usizex8::from_array([255, 0, 255, 1, 255, 255, 255, 2]),
    usizex8::from_array([0, 1, 255, 2, 255, 255, 255, 3]),
    usizex8::from_array([255, 255, 0, 1, 255, 255, 255, 2]),
    usizex8::from_array([0, 255, 1, 2, 255, 255, 255, 3]),
    usizex8::from_array([255, 0, 1, 2, 255, 255, 255, 3]),
    usizex8::from_array([0, 1, 2, 3, 255, 255, 255, 4]),
    usizex8::from_array([255, 255, 255, 255, 0, 255, 255, 1]),
    usizex8::from_array([0, 255, 255, 255, 1, 255, 255, 2]),
    usizex8::from_array([255, 0, 255, 255, 1, 255, 255, 2]),
    usizex8::from_array([0, 1, 255, 255, 2, 255, 255, 3]),
    usizex8::from_array([255, 255, 0, 255, 1, 255, 255, 2]),
    usizex8::from_array([0, 255, 1, 255, 2, 255, 255, 3]),
    usizex8::from_array([255, 0, 1, 255, 2, 255, 255, 3]),
    usizex8::from_array([0, 1, 2, 255, 3, 255, 255, 4]),
    usizex8::from_array([255, 255, 255, 0, 1, 255, 255, 2]),
    usizex8::from_array([0, 255, 255, 1, 2, 255, 255, 3]),
    usizex8::from_array([255, 0, 255, 1, 2, 255, 255, 3]),
    usizex8::from_array([0, 1, 255, 2, 3, 255, 255, 4]),
    usizex8::from_array([255, 255, 0, 1, 2, 255, 255, 3]),
    usizex8::from_array([0, 255, 1, 2, 3, 255, 255, 4]),
    usizex8::from_array([255, 0, 1, 2, 3, 255, 255, 4]),
    usizex8::from_array([0, 1, 2, 3, 4, 255, 255, 5]),
    usizex8::from_array([255, 255, 255, 255, 255, 0, 255, 1]),
    usizex8::from_array([0, 255, 255, 255, 255, 1, 255, 2]),
    usizex8::from_array([255, 0, 255, 255, 255, 1, 255, 2]),
    usizex8::from_array([0, 1, 255, 255, 255, 2, 255, 3]),
    usizex8::from_array([255, 255, 0, 255, 255, 1, 255, 2]),
    usizex8::from_array([0, 255, 1, 255, 255, 2, 255, 3]),
    usizex8::from_array([255, 0, 1, 255, 255, 2, 255, 3]),
    usizex8::from_array([0, 1, 2, 255, 255, 3, 255, 4]),
    usizex8::from_array([255, 255, 255, 0, 255, 1, 255, 2]),
    usizex8::from_array([0, 255, 255, 1, 255, 2, 255, 3]),
    usizex8::from_array([255, 0, 255, 1, 255, 2, 255, 3]),
    usizex8::from_array([0, 1, 255, 2, 255, 3, 255, 4]),
    usizex8::from_array([255, 255, 0, 1, 255, 2, 255, 3]),
    usizex8::from_array([0, 255, 1, 2, 255, 3, 255, 4]),
    usizex8::from_array([255, 0, 1, 2, 255, 3, 255, 4]),
    usizex8::from_array([0, 1, 2, 3, 255, 4, 255, 5]),
    usizex8::from_array([255, 255, 255, 255, 0, 1, 255, 2]),
    usizex8::from_array([0, 255, 255, 255, 1, 2, 255, 3]),
    usizex8::from_array([255, 0, 255, 255, 1, 2, 255, 3]),
    usizex8::from_array([0, 1, 255, 255, 2, 3, 255, 4]),
    usizex8::from_array([255, 255, 0, 255, 1, 2, 255, 3]),
    usizex8::from_array([0, 255, 1, 255, 2, 3, 255, 4]),
    usizex8::from_array([255, 0, 1, 255, 2, 3, 255, 4]),
    usizex8::from_array([0, 1, 2, 255, 3, 4, 255, 5]),
    usizex8::from_array([255, 255, 255, 0, 1, 2, 255, 3]),
    usizex8::from_array([0, 255, 255, 1, 2, 3, 255, 4]),
    usizex8::from_array([255, 0, 255, 1, 2, 3, 255, 4]),
    usizex8::from_array([0, 1, 255, 2, 3, 4, 255, 5]),
    usizex8::from_array([255, 255, 0, 1, 2, 3, 255, 4]),
    usizex8::from_array([0, 255, 1, 2, 3, 4, 255, 5]),
    usizex8::from_array([255, 0, 1, 2, 3, 4, 255, 5]),
    usizex8::from_array([0, 1, 2, 3, 4, 5, 255, 6]),
    usizex8::from_array([255, 255, 255, 255, 255, 255, 0, 1]),
    usizex8::from_array([0, 255, 255, 255, 255, 255, 1, 2]),
    usizex8::from_array([255, 0, 255, 255, 255, 255, 1, 2]),
    usizex8::from_array([0, 1, 255, 255, 255, 255, 2, 3]),
    usizex8::from_array([255, 255, 0, 255, 255, 255, 1, 2]),
    usizex8::from_array([0, 255, 1, 255, 255, 255, 2, 3]),
    usizex8::from_array([255, 0, 1, 255, 255, 255, 2, 3]),
    usizex8::from_array([0, 1, 2, 255, 255, 255, 3, 4]),
    usizex8::from_array([255, 255, 255, 0, 255, 255, 1, 2]),
    usizex8::from_array([0, 255, 255, 1, 255, 255, 2, 3]),
    usizex8::from_array([255, 0, 255, 1, 255, 255, 2, 3]),
    usizex8::from_array([0, 1, 255, 2, 255, 255, 3, 4]),
    usizex8::from_array([255, 255, 0, 1, 255, 255, 2, 3]),
    usizex8::from_array([0, 255, 1, 2, 255, 255, 3, 4]),
    usizex8::from_array([255, 0, 1, 2, 255, 255, 3, 4]),
    usizex8::from_array([0, 1, 2, 3, 255, 255, 4, 5]),
    usizex8::from_array([255, 255, 255, 255, 0, 255, 1, 2]),
    usizex8::from_array([0, 255, 255, 255, 1, 255, 2, 3]),
    usizex8::from_array([255, 0, 255, 255, 1, 255, 2, 3]),
    usizex8::from_array([0, 1, 255, 255, 2, 255, 3, 4]),
    usizex8::from_array([255, 255, 0, 255, 1, 255, 2, 3]),
    usizex8::from_array([0, 255, 1, 255, 2, 255, 3, 4]),
    usizex8::from_array([255, 0, 1, 255, 2, 255, 3, 4]),
    usizex8::from_array([0, 1, 2, 255, 3, 255, 4, 5]),
    usizex8::from_array([255, 255, 255, 0, 1, 255, 2, 3]),
    usizex8::from_array([0, 255, 255, 1, 2, 255, 3, 4]),
    usizex8::from_array([255, 0, 255, 1, 2, 255, 3, 4]),
    usizex8::from_array([0, 1, 255, 2, 3, 255, 4, 5]),
    usizex8::from_array([255, 255, 0, 1, 2, 255, 3, 4]),
    usizex8::from_array([0, 255, 1, 2, 3, 255, 4, 5]),
    usizex8::from_array([255, 0, 1, 2, 3, 255, 4, 5]),
    usizex8::from_array([0, 1, 2, 3, 4, 255, 5, 6]),
    usizex8::from_array([255, 255, 255, 255, 255, 0, 1, 2]),
    usizex8::from_array([0, 255, 255, 255, 255, 1, 2, 3]),
    usizex8::from_array([255, 0, 255, 255, 255, 1, 2, 3]),
    usizex8::from_array([0, 1, 255, 255, 255, 2, 3, 4]),
    usizex8::from_array([255, 255, 0, 255, 255, 1, 2, 3]),
    usizex8::from_array([0, 255, 1, 255, 255, 2, 3, 4]),
    usizex8::from_array([255, 0, 1, 255, 255, 2, 3, 4]),
    usizex8::from_array([0, 1, 2, 255, 255, 3, 4, 5]),
    usizex8::from_array([255, 255, 255, 0, 255, 1, 2, 3]),
    usizex8::from_array([0, 255, 255, 1, 255, 2, 3, 4]),
    usizex8::from_array([255, 0, 255, 1, 255, 2, 3, 4]),
    usizex8::from_array([0, 1, 255, 2, 255, 3, 4, 5]),
    usizex8::from_array([255, 255, 0, 1, 255, 2, 3, 4]),
    usizex8::from_array([0, 255, 1, 2, 255, 3, 4, 5]),
    usizex8::from_array([255, 0, 1, 2, 255, 3, 4, 5]),
    usizex8::from_array([0, 1, 2, 3, 255, 4, 5, 6]),
    usizex8::from_array([255, 255, 255, 255, 0, 1, 2, 3]),
    usizex8::from_array([0, 255, 255, 255, 1, 2, 3, 4]),
    usizex8::from_array([255, 0, 255, 255, 1, 2, 3, 4]),
    usizex8::from_array([0, 1, 255, 255, 2, 3, 4, 5]),
    usizex8::from_array([255, 255, 0, 255, 1, 2, 3, 4]),
    usizex8::from_array([0, 255, 1, 255, 2, 3, 4, 5]),
    usizex8::from_array([255, 0, 1, 255, 2, 3, 4, 5]),
    usizex8::from_array([0, 1, 2, 255, 3, 4, 5, 6]),
    usizex8::from_array([255, 255, 255, 0, 1, 2, 3, 4]),
    usizex8::from_array([0, 255, 255, 1, 2, 3, 4, 5]),
    usizex8::from_array([255, 0, 255, 1, 2, 3, 4, 5]),
    usizex8::from_array([0, 1, 255, 2, 3, 4, 5, 6]),
    usizex8::from_array([255, 255, 0, 1, 2, 3, 4, 5]),
    usizex8::from_array([0, 255, 1, 2, 3, 4, 5, 6]),
    usizex8::from_array([255, 0, 1, 2, 3, 4, 5, 6]),
    usizex8::from_array([0, 1, 2, 3, 4, 5, 6, 7]),
];

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::fmt::Debug;

    fn test_filter<T>()
    where
        T: rand::distributions::uniform::SampleUniform
            + PartialEq
            + Debug
            + Copy
            + Default
            + SimdElement
            + std::cmp::PartialEq
            + PartialOrd,
        Simd<T, 8>: SimdPartialOrd<Mask = Mask<T::Mask, 8>>,
        Standard: Distribution<T>,
    {
        for len in 0..1000 {
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }
                let needle = v.choose(&mut rng).cloned().unwrap_or(T::default());
                let ans = v.iter().filter_simd_lt(needle);
                let correct = v.iter().filter(|x| **x < needle).cloned().collect_vec();
                assert_eq!(
                    ans,
                    correct,
                    "Failed for LT length {} and type {:?} {:?} {:?}",
                    len,
                    std::any::type_name::<T>(),
                    needle,
                    v
                );
            }
        }
        for len in 0..1000 {
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }
                let needle = v.choose(&mut rng).cloned().unwrap_or(T::default());
                let ans = v.iter().filter_simd_gt(needle);
                let correct = v.iter().filter(|x| **x > needle).cloned().collect_vec();
                assert_eq!(
                    ans,
                    correct,
                    "Failed GT for length {} and type {:?} {:?} {:?}",
                    len,
                    std::any::type_name::<T>(),
                    needle,
                    v
                );
            }
        }
        for len in 0..1000 {
            for _ in 0..5 {
                let mut v: Vec<T> = vec![T::default(); len];
                let mut rng = rand::thread_rng();
                for x in v.iter_mut() {
                    *x = rng.gen()
                }
                let needle = v.choose(&mut rng).cloned().unwrap_or(T::default());
                let ans = v.iter().filter_simd_eq(needle);
                let correct = v.iter().filter(|x| **x == needle).cloned().collect_vec();
                assert_eq!(
                    ans,
                    correct,
                    "Failed EQ for length {} and type {:?} {:?} {:?}",
                    len,
                    std::any::type_name::<T>(),
                    needle,
                    v
                );
            }
        }
    }

    #[test]
    fn test_filter_simd() {
        test_filter::<u8>();
        test_filter::<i32>();
        test_filter::<i8>();
        test_filter::<i16>();
        test_filter::<i64>();
        test_filter::<u8>();
        test_filter::<u16>();
        test_filter::<u64>();
        test_filter::<usize>();
        test_filter::<isize>();
        test_filter::<f32>();
        test_filter::<f64>();
    }
}
