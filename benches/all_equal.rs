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

#[inline(always)]
fn trivial<T: PartialEq>(a: &[T]) -> bool {
    a.iter().all_equal()
}

fn benchmark_all_equal<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    c: &mut Criterion,
    name: &str,
) where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let len = 1000;
    let v1 = vec![T::default(); len];
    let v2 = vec![T::default(); len];

    assert_eq!(v1, v2);

    c.bench_function(&format!("SIMD all equal {}", name), |b| {
        b.iter(|| black_box(v1.iter().all_equal_simd()))
    });
    c.bench_function(&format!("trivial all equal {}", name), |b| {
        b.iter(|| trivial(black_box(&v1)))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_all_equal::<u8>(c, "u8");
    benchmark_all_equal::<i8>(c, "i8");
    benchmark_all_equal::<u16>(c, "u16");
    benchmark_all_equal::<i16>(c, "i16");
    benchmark_all_equal::<u32>(c, "u32");
    benchmark_all_equal::<i32>(c, "i32");
    benchmark_all_equal::<u64>(c, "u64");
    benchmark_all_equal::<i64>(c, "i64");
    benchmark_all_equal::<f32>(c, "f32");
    benchmark_all_equal::<f64>(c, "f64");
    benchmark_all_equal::<isize>(c, "isize");
    benchmark_all_equal::<usize>(c, "usize");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
