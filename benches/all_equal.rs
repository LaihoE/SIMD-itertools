#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use simd_itertools::AllEqualSimd;
use simd_itertools::SIMD_LEN;
use std::fmt::Debug;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::Simd;
use std::simd::SimdElement;
use std::time::Duration;

fn benchmark_all_equal<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    _c: &mut Criterion,
    name: &str,
    len: usize,
) where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let v1 = vec![T::default(); len];

    let mut group = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1));

    group.bench_function(&format!("SIMD all_equal {} {}", name, len), |b| {
        b.iter(|| black_box(v1.iter().all_equal_simd()))
    });
    group.bench_function(&format!("Scalar all_equal {} {}", name, len), |b| {
        b.iter(|| black_box(v1.iter().all_equal()))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    for n in (0..200).map(|x| x * 10) {
        benchmark_all_equal::<u8>(c, "u8", n);
        benchmark_all_equal::<i8>(c, "i8", n);
        benchmark_all_equal::<u16>(c, "u16", n);
        benchmark_all_equal::<i16>(c, "i16", n);
        benchmark_all_equal::<u32>(c, "u32", n);
        benchmark_all_equal::<i32>(c, "i32", n);
        benchmark_all_equal::<u64>(c, "u64", n);
        benchmark_all_equal::<i64>(c, "i64", n);
        benchmark_all_equal::<f32>(c, "f32", n);
        benchmark_all_equal::<f64>(c, "f64", n);
        benchmark_all_equal::<isize>(c, "isize", n);
        benchmark_all_equal::<usize>(c, "usize", n);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
