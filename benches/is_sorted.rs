#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::IsSortedSimd;

#[inline(always)]
fn trivial(a: &[i32]) -> bool {
    a.is_sorted()
}

fn criterion_benchmark(c: &mut Criterion) {
    let len = 100000;
    let mut v = vec![0; len];
    for x in v.iter_mut() {
        *x = rand::random()
    }
    if true {
        v.sort();
    }
    c.bench_function("SIMD is_sorted", |b| {
        b.iter(|| black_box(v.iter().is_sorted_simd()))
    });
    c.bench_function("trivial is_sorted", |b| b.iter(|| trivial(black_box(&v))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
