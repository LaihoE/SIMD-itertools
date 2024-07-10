#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use simd_itertools::ContainsSimd;
use simd_itertools::SIMD_LEN;
use std::simd::prelude::SimdPartialEq;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};

#[inline(always)]
fn trivial<T>(arr: &[T], val: T) -> bool
where
    T: std::cmp::PartialEq,
{
    arr.iter().contains(&val)
}
#[inline(always)]
fn simd<T>(arr: &[T], val: T) -> bool
where
    T: std::cmp::PartialEq,
    T: SimdElement + std::cmp::PartialEq,
    Simd<T, SIMD_LEN>: SimdPartialEq<Mask = Mask<T::Mask, SIMD_LEN>>,
{
    arr.iter().contains_simd(val)
}

fn criterion_benchmark(c: &mut Criterion) {
    let len = 1000;
    let mut v = vec![55_u8; len];
    v[len - 1] = 7;

    c.bench_function("SIMD contains", |b| {
        b.iter(|| simd(black_box(&v), black_box(7)))
    });
    c.bench_function("trivial contains", |b| {
        b.iter(|| trivial(black_box(&v), black_box(7)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
