#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd_itertools::{PositionSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialEq, Mask, Simd, SimdElement},
};

fn benchmark_position<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + TryFrom<i32> + Debug,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<i32>>::Error: Debug,
{
    let mut group = c.benchmark_group(format!("position-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];
        let needle: T = 55.try_into().unwrap();

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().position_simd(needle)))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().position(|x| *x == needle)))
        });

        len *= 10;
    }

    group.finish();
}

fn benchmark_position_floats<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + TryFrom<f32> + Debug + std::convert::From<f32>,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
    <T as TryFrom<f32>>::Error: Debug,
{
    let mut group = c.benchmark_group(format!("position-floats-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];
        let needle: T = (55.0).try_into().unwrap();

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().position_simd(needle)))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().position(|x| *x == needle)))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_position::<u8>(c);
    benchmark_position::<u64>(c);
    benchmark_position::<u32>(c);
    benchmark_position::<u8>(c);
    benchmark_position::<i8>(c);
    benchmark_position::<u16>(c);
    benchmark_position::<i16>(c);
    benchmark_position::<i32>(c);
    benchmark_position::<u64>(c);
    benchmark_position::<i64>(c);
    benchmark_position::<isize>(c);
    benchmark_position::<usize>(c);
    benchmark_position_floats::<f32>(c);
    benchmark_position_floats::<f64>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
