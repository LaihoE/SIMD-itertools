// #![feature(portable_simd)]

// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use simd_itertools::MinSimd;
// use simd_itertools::SIMD_LEN;
// use std::fmt::Debug;
// use std::simd::prelude::SimdPartialEq;
// use std::simd::Mask;
// use std::simd::{Simd, SimdElement};
// use std::time::Duration;

// fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(
//     _c: &mut Criterion,
//     name: &str,
//     len: usize,
// ) where
//     T: SimdElement + std::cmp::PartialEq + std::cmp::Ord,
//     Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
// {
//     let v1 = black_box(vec![T::default(); len]);

//     let mut group = Criterion::default()
//         .warm_up_time(Duration::from_secs(1))
//         .measurement_time(Duration::from_secs(1));
//     group.bench_function(&format!("SIMD min {} {}", name, len), |b| {
//         b.iter(|| black_box(black_box(&v1).iter().min_simd()))
//     });
//     group.bench_function(&format!("Scalar min {} {}", name, len), |b| {
//         b.iter(|| black_box(v1.iter().min()))
//     });
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     for n in (0..200).map(|x| x * 10) {
//         benchmark_contains::<u8>(c, "u8", n);
//         benchmark_contains::<i8>(c, "i8", n);
//         benchmark_contains::<u16>(c, "u16", n);
//         benchmark_contains::<i16>(c, "i16", n);
//         benchmark_contains::<u32>(c, "u32", n);
//         benchmark_contains::<i32>(c, "i32", n);
//         benchmark_contains::<u64>(c, "u64", n);
//         benchmark_contains::<i64>(c, "i64", n);
//         benchmark_contains::<isize>(c, "isize", n);
//         benchmark_contains::<usize>(c, "usize", n);
//     }
// }
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);

fn main() {}
