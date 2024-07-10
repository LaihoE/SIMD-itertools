use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::FindSimd;

#[inline(always)]
fn trivial(arr: &[i32], val: i32) -> Option<&i32> {
    arr.iter().find(|x| **x == val)
}

fn criterion_benchmark(c: &mut Criterion) {
    let len = 100000;

    let mut v = vec![55; len];
    v[len - 1] = 7;
    // v[69] = 7;
    // v[761] = 7;
    // v[2200] = 7;
    // v[4200] = 7;
    // v[5200] = 7;

    c.bench_function("SIMD find", |b| {
        b.iter(|| black_box(v.iter().find_simd(black_box(7))))
    });
    c.bench_function("trivial find", |b| {
        b.iter(|| trivial(black_box(&v), black_box(7)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
