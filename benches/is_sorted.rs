#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::IsSortedSimd;
use simd_itertools::SIMD_LEN;
use std::fmt::Debug;
use std::simd::prelude::SimdPartialOrd;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};
use std::time::Duration;

fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    _c: &mut Criterion,
    name: &str,
    len: usize,
) where
    T: SimdElement + std::cmp::PartialEq + Debug + std::cmp::PartialOrd,
    Simd<T, SIMD_LEN>: SimdPartialOrd<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let v1 = vec![T::default(); len];
    let v2 = vec![T::default(); len];
    assert_eq!(v1, v2);

    let mut group = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1));
    group.bench_function(&format!("SIMD is_sorted {} {}", name, len), |b| {
        b.iter(|| black_box(black_box(&v1).iter().is_sorted_simd()))
    });
    group.bench_function(&format!("Scalar is_sorted {} {}", name, len), |b| {
        b.iter(|| black_box(v1.iter().is_sorted()))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    for n in (0..200).map(|x| x * 10) {
        benchmark_contains::<u8>(c, "u8", n);
        benchmark_contains::<i8>(c, "i8", n);
        benchmark_contains::<u16>(c, "u16", n);
        benchmark_contains::<i16>(c, "i16", n);
        benchmark_contains::<u32>(c, "u32", n);
        benchmark_contains::<i32>(c, "i32", n);
        benchmark_contains::<u64>(c, "u64", n);
        benchmark_contains::<i64>(c, "i64", n);
        benchmark_contains::<isize>(c, "isize", n);
        benchmark_contains::<usize>(c, "usize", n);
        benchmark_contains::<f32>(c, "f32", n);
        benchmark_contains::<f64>(c, "f64", n);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
