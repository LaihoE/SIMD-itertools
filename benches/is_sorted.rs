#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::IsSortedSimd;
use simd_itertools::SIMD_LEN;
use std::fmt::Debug;
use std::simd::prelude::SimdPartialOrd;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};

fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    c: &mut Criterion,
    name: &str,
) where
    T: SimdElement + std::cmp::PartialEq + Debug + std::cmp::PartialOrd,
    Simd<T, SIMD_LEN>: SimdPartialOrd<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let len = 1000;
    let v1 = vec![T::default(); len];
    let v2 = vec![T::default(); len];
    assert_eq!(v1, v2);

    c.bench_function(&format!("SIMD all equal {}", name), |b| {
        b.iter(|| black_box(v1.iter().is_sorted_simd()));
    });
    c.bench_function(&format!("trivial all equal {}", name), |b| {
        b.iter(|| black_box(v1.iter().is_sorted()));
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
    benchmark_contains::<f32>(c, "f32");
    benchmark_contains::<f64>(c, "f64");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
