#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd_itertools::FilterSimd;
use std::{
    fmt::Debug,
    simd::{cmp::SimdPartialOrd, Mask, Simd, SimdElement},
};

fn benchmark_filter<'a, T: 'static + Copy + PartialEq + Default + Debug>(c: &mut Criterion)
where
    T: SimdElement + std::cmp::PartialEq + TryFrom<i32> + Debug + PartialOrd,
    Simd<T, 8>: SimdPartialOrd<Mask = Mask<T::Mask, 8>>,
    <T as TryFrom<i32>>::Error: Debug,
{
    let mut group = c.benchmark_group(format!("filter-{}", std::any::type_name::<T>()));
    let mut len = 1;

    while len < (1 << 11) {
        let v1 = vec![T::default(); len];
        let needle: T = 55.try_into().unwrap();

        group.throughput(Throughput::Elements(len as u64));

        group.bench_function(BenchmarkId::new("SIMD", len), |b| {
            b.iter(|| black_box(v1.iter().filter_simd_lt(needle)))
        });
        group.bench_function(BenchmarkId::new("Scalar", len), |b| {
            b.iter(|| black_box(v1.iter().filter(|x| **x < needle)))
        });

        len *= 10;
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_filter::<u8>(c);
    benchmark_filter::<u16>(c);
    benchmark_filter::<u32>(c);
    benchmark_filter::<u64>(c);
    benchmark_filter::<u8>(c);
    benchmark_filter::<i8>(c);
    benchmark_filter::<u16>(c);
    benchmark_filter::<i16>(c);
    benchmark_filter::<i32>(c);
    benchmark_filter::<u64>(c);
    benchmark_filter::<i64>(c);
    benchmark_filter::<isize>(c);
    benchmark_filter::<usize>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
