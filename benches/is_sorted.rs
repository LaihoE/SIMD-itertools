#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd_itertools::{IsSortedSimd, SIMD_LEN};
use std::{
    fmt::Debug,
    simd::{prelude::SimdPartialOrd, Mask, Simd, SimdElement},
};

fn benchmark_is_sorted<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + Debug + std::cmp::PartialOrd,
    Simd<T, SIMD_LEN>: SimdPartialOrd<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    let mut group = c.benchmark_group(format!("is-sorted-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().is_sorted_simd()))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().is_sorted()))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_is_sorted::<u8>(c);
    benchmark_is_sorted::<i8>(c);
    benchmark_is_sorted::<u16>(c);
    benchmark_is_sorted::<i16>(c);
    benchmark_is_sorted::<u32>(c);
    benchmark_is_sorted::<i32>(c);
    benchmark_is_sorted::<u64>(c);
    benchmark_is_sorted::<i64>(c);
    benchmark_is_sorted::<isize>(c);
    benchmark_is_sorted::<usize>(c);
    benchmark_is_sorted::<f32>(c);
    benchmark_is_sorted::<f64>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
