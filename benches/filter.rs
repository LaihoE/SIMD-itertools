#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::FilterSimd;
use simd_itertools::SIMD_LEN;
use std::fmt::Debug;
use std::simd::cmp::SimdPartialOrd;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};

fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    c: &mut Criterion,
    name: &str,
) where
    T: SimdElement + std::cmp::PartialEq + TryFrom<i32> + Debug + PartialOrd,
    // Simd<T, SIMD_LEN>: SimdPartialOrd<Mask = Mask<T::Mask, SIMD_LEN>>,
    Simd<T, 8>: SimdPartialOrd<Mask = Mask<T::Mask, 8>>,

    <T as TryFrom<i32>>::Error: Debug,
{
    let ns = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];
    for i in ns {
        let len = i;
        let v1 = vec![T::default(); len];
        let needle: T = 55.try_into().unwrap();

        c.bench_function(
            &format!("SIMD position {} len:{} simdl:{}", name, len, SIMD_LEN),
            |b| b.iter(|| black_box(v1.iter().filter_simd_lt(needle))),
        );
        c.bench_function(
            &format!("trivial position {} len:{} simdl:{}", name, len, SIMD_LEN),
            |b| b.iter(|| black_box(v1.iter().position(|x| *x == needle))),
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_contains::<u8>(c, "u8");
    benchmark_contains::<u16>(c, "u16");
    benchmark_contains::<u32>(c, "u32");
    benchmark_contains::<u64>(c, "u64");
    benchmark_contains::<u8>(c, "u8");
    benchmark_contains::<i8>(c, "i8");
    benchmark_contains::<u16>(c, "u16");
    benchmark_contains::<i16>(c, "i16");
    benchmark_contains::<i32>(c, "i32");
    benchmark_contains::<u64>(c, "u64");
    benchmark_contains::<i64>(c, "i64");
    benchmark_contains::<isize>(c, "isize");
    benchmark_contains::<usize>(c, "usize");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
