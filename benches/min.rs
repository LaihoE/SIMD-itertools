#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd_itertools::{MinSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialEq, Mask, Simd, SimdElement},
};

fn benchmark_min<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + std::cmp::Ord,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let mut group = c.benchmark_group(format!("min-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().min_simd()))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().min()))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_min::<u8>(c);
    benchmark_min::<i8>(c);
    benchmark_min::<u16>(c);
    benchmark_min::<i16>(c);
    benchmark_min::<u32>(c);
    benchmark_min::<i32>(c);
    benchmark_min::<u64>(c);
    benchmark_min::<i64>(c);
    benchmark_min::<isize>(c);
    benchmark_min::<usize>(c);
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
