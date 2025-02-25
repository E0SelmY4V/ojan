use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ojan_luogu::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let a = BigNatural::from(0x0923894929fafa789768_u128);
    let b = BigNatural::from(195);
    c.bench_function("Big * Small", |bencher| bencher.iter(|| {
        let m = black_box(a.clone());
        let n = black_box(b.clone());
        m * n
    }));
    c.bench_function("Small * Big", |bencher| bencher.iter(|| {
        let m = black_box(b.clone());
        let n = black_box(a.clone());
        m * n
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
