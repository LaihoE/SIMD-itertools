#![feature(portable_simd)]
#![feature(is_sorted)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::MinSimd;

#[inline(always)]
fn trivial(a: &[i32]) -> Option<&i32> {
    a.iter().min()
}

fn criterion_benchmark(c: &mut Criterion) {
    let len = 100000;
    let mut v = vec![0; len];
    for x in v.iter_mut() {
        *x = rand::random()
    }
    c.bench_function("trivial MIN", |b| b.iter(|| trivial(black_box(&v))));
    c.bench_function("SIMD MIN", |b| b.iter(|| black_box(v.iter().min_simd())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
