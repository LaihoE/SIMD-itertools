#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd_itertools::{MaxSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialEq, Mask, Simd, SimdElement},
};

fn benchmark_max<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + std::cmp::Ord,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let mut group = c.benchmark_group(format!("max-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().max_simd()))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().max()))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    for n in (0..200).map(|x| x * 10) {
        benchmark_max::<u8>(c);
        benchmark_max::<i8>(c);
        benchmark_max::<u16>(c);
        benchmark_max::<i16>(c);
        benchmark_max::<u32>(c);
        benchmark_max::<i32>(c);
        benchmark_max::<u64>(c);
        benchmark_max::<i64>(c);
        benchmark_max::<isize>(c);
        benchmark_max::<usize>(c);
    }
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
