#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use simd_itertools::ContainsSimd;
use simd_itertools::SIMD_LEN;
use std::fmt::Debug;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};

#[inline(always)]
fn trivial<T>(arr: &[T], val: T) -> bool
where
    T: std::cmp::PartialEq,
{
    arr.iter().contains(&val)
}

fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    c: &mut Criterion,
    name: &str,
) where
    T: SimdElement + std::cmp::PartialEq + TryFrom<i32> + Debug,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<i32>>::Error: Debug,
{
    let len = 1000;
    let v1 = vec![T::default(); len];
    let v2 = vec![T::default(); len];
    let needle: T = 55.try_into().unwrap();
    assert_eq!(v1, v2);

    c.bench_function(&format!("SIMD all equal {}", name), |b| {
        b.iter(|| black_box(v1.iter().contains_simd(needle)))
    });
    c.bench_function(&format!("trivial all equal {}", name), |b| {
        b.iter(|| trivial(black_box(&v1), needle))
    });
}

fn benchmark_contains_floats<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    c: &mut Criterion,
    name: &str,
) where
    T: SimdElement + std::cmp::PartialEq + TryFrom<f32> + Debug + std::convert::From<f32>,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<f32>>::Error: Debug,
{
    let len = 1000;
    let v1 = vec![T::default(); len];
    let v2 = vec![T::default(); len];
    let needle: T = 55.0.try_into().unwrap();
    assert_eq!(v1, v2);

    c.bench_function(&format!("SIMD all equal {}", name), |b| {
        b.iter(|| black_box(v1.iter().contains_simd(needle)))
    });
    c.bench_function(&format!("trivial all equal {}", name), |b| {
        b.iter(|| trivial(black_box(&v1), needle))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_contains::<u8>(c, "u8");
    benchmark_contains::<i8>(c, "i8");
    benchmark_contains::<u16>(c, "u16");
    benchmark_contains::<i16>(c, "i16");
    benchmark_contains::<u32>(c, "u32");
    benchmark_contains::<i32>(c, "i32");
    benchmark_contains::<u64>(c, "u64");
    benchmark_contains::<i64>(c, "i64");
    benchmark_contains::<isize>(c, "isize");
    benchmark_contains::<usize>(c, "usize");
    benchmark_contains_floats::<f32>(c, "f32");
    benchmark_contains_floats::<f64>(c, "f64");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
