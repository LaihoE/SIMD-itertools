#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd_itertools::{EqSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialEq, Mask, Simd, SimdElement},
};

fn benchmark_eq<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let mut group = c.benchmark_group(format!("contains-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = black_box(vec![T::default(); len]);
        let v2 = black_box(vec![T::default(); len]);

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().eq_simd(&v2.iter())))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().eq(&v2)))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_eq::<u8>(c);
    benchmark_eq::<i8>(c);
    benchmark_eq::<u16>(c);
    benchmark_eq::<i16>(c);
    benchmark_eq::<u32>(c);
    benchmark_eq::<i32>(c);
    benchmark_eq::<u64>(c);
    benchmark_eq::<i64>(c);
    benchmark_eq::<isize>(c);
    benchmark_eq::<usize>(c);
    benchmark_eq::<f32>(c);
    benchmark_eq::<f64>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
