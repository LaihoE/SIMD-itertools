#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::MinSimd;
use simd_itertools::SIMD_LEN;
use std::fmt::Debug;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};

fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    c: &mut Criterion,
    name: &str,
) where
    T: SimdElement + std::cmp::PartialEq + std::cmp::Ord,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let len = 10000;
    let v1 = black_box(vec![T::default(); len]);

    c.bench_function(&format!("SIMD max {}", name), |b| {
        b.iter(|| black_box(black_box(&v1).iter().min_simd()))
    });
    c.bench_function(&format!("trivial max {}", name), |b| {
        b.iter(|| black_box(v1.iter().min()))
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
    // benchmark_contains::<f32>(c, "f32");
    // benchmark_contains::<f64>(c, "f64");
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
