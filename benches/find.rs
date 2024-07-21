#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd_itertools::{FindSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialEq, Mask, Simd, SimdElement},
};

fn benchmark_find<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + TryFrom<i32> + Debug,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<i32>>::Error: Debug,
{
    let mut group = c.benchmark_group(format!("find-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];
        let needle: T = 55.try_into().unwrap();

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().find_simd(needle)))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().find(|x| **x == needle)))
        });

        len *= 10;
    }

    group.finish();
}

fn benchmark_find_float<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + TryFrom<f32> + Debug + std::convert::From<f32>,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<f32>>::Error: Debug,
{
    let mut group = c.benchmark_group(format!("find-float-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];
        let needle: T = (55.0).try_into().unwrap();

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().find_simd(needle)))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().find(|x| **x == needle)))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_find::<u8>(c);
    benchmark_find::<i8>(c);
    benchmark_find::<u16>(c);
    benchmark_find::<i16>(c);
    benchmark_find::<u32>(c);
    benchmark_find::<i32>(c);
    benchmark_find::<u64>(c);
    benchmark_find::<i64>(c);
    benchmark_find::<isize>(c);
    benchmark_find::<usize>(c);
    benchmark_find_float::<f32>(c);
    benchmark_find_float::<f64>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
