#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::*;

#[inline(always)]
fn trivial(a: &[i32]) -> Option<i32> {
    a.iter().copied().min()
}

fn criterion_benchmark(c: &mut Criterion) {
    let len = 100000;
    let mut v = vec![0; len];
    for x in v.iter_mut() {
        *x = rand::random()
    }
    c.bench_function("trivial MAX", |b| b.iter(|| trivial(black_box(&v))));
    c.bench_function("SIMD MAX", |b| b.iter(|| black_box(v.iter().max_simd())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
