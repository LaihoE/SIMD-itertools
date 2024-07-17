#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use itertools::Itertools;
use simd_itertools::{AllEqualSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialEq, Mask, Simd, SimdElement},
};

fn benchmark_all_equal<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let mut group = c.benchmark_group(format!("all-equal-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().all_equal_simd()))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().all_equal()))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_all_equal::<u8>(c);
    // benchmark_all_equal::<i8>(c);
    // benchmark_all_equal::<u16>(c);
    // benchmark_all_equal::<i16>(c, "i16", n);
    // benchmark_all_equal::<u32>(c, "u32", n);
    // benchmark_all_equal::<i32>(c, "i32", n);
    // benchmark_all_equal::<u64>(c, "u64", n);
    // benchmark_all_equal::<i64>(c, "i64", n);
    // benchmark_all_equal::<f32>(c, "f32", n);
    // benchmark_all_equal::<f64>(c, "f64", n);
    // benchmark_all_equal::<isize>(c, "isize", n);
    // benchmark_all_equal::<usize>(c, "usize", n);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
