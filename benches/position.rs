use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::PositionSimd;

#[inline(always)]
fn trivial(arr: &[i32], val: i32) -> Option<usize> {
    arr.iter().position(|x| *x == val)
}

fn criterion_benchmark(c: &mut Criterion) {
    let len = 100000;
    let mut v = vec![55; len];
    v[len - 1] = 7;

    c.bench_function("SIMD position CHUNKED", |b| {
        b.iter(|| black_box(v.iter().position_simd(black_box(7))))
    });
    c.bench_function("trivial position", |b| {
        b.iter(|| trivial(black_box(&v), black_box(7)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
