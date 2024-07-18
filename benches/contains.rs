#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use itertools::Itertools;
use simd_itertools::{ContainsSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialEq, Mask, Simd, SimdElement},
};

fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + TryFrom<i32> + Debug,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<i32>>::Error: Debug,
{
    let mut group = c.benchmark_group(format!("contains-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];
        let needle: T = 55.try_into().unwrap();

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().contains_simd(&needle)))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().contains(&needle)))
        });

        len *= 10;
    }

    group.finish();
}

fn benchmark_contains_floats<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + TryFrom<f32> + Debug + std::convert::From<f32>,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<f32>>::Error: Debug,
{
    let mut group = c.benchmark_group(format!("contains-float-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];
        let v2 = vec![T::default(); len];
        let needle: T = 55.0.try_into().unwrap();
        assert_eq!(v1, v2);

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().contains_simd(&needle)))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().contains(&needle)))
        });
    }
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_contains::<u8>(c);
    benchmark_contains::<i8>(c);
    benchmark_contains::<u16>(c);
    benchmark_contains::<i16>(c);
    benchmark_contains::<u32>(c);
    benchmark_contains::<i32>(c);
    benchmark_contains::<u64>(c);
    benchmark_contains::<i64>(c);
    benchmark_contains::<isize>(c);
    benchmark_contains::<usize>(c);
    benchmark_contains_floats::<f32>(c);
    benchmark_contains_floats::<f64>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
