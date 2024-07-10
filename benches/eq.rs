#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::SimdEq;

#[inline(always)]
fn trivial(a: &[i32], b: &[i32]) -> bool {
    a.iter().eq(b.iter())
}
#[inline(always)]
fn trivial2(a: &[i32], b: &[i32]) -> bool {
    a == b
}
#[inline(always)]
fn eq_simd(a: &[i32], b: &[i32]) -> bool {
    a.iter().simd_eq(&b.iter())
}

fn criterion_benchmark(c: &mut Criterion) {
    let len = 100000;
    let v1 = vec![44; len];
    let v2 = vec![44; len];

    assert_eq!(v1, v2);

    let mut group = c.benchmark_group("TEST");

    group.bench_function("SIMD eq chunked", |b| {
        b.iter(|| eq_simd(black_box(&v1), black_box(&v2)))
    });
    group.bench_function("trivial eq", |b| {
        b.iter(|| trivial(black_box(&v1), black_box(&v2)))
    });
    group.bench_function("smart", |b| {
        b.iter(|| trivial2(black_box(&v1), black_box(&v2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
