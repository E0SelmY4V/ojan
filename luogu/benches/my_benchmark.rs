use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ojan_luogu::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let a = BigNatural::from(0x0923894929fafa789768_u128);
    let b = BigNatural::from(183_u8);
    c.bench_function("Big div", |bencher| bencher.iter(|| {
        let m = black_box(b.clone());
        let n = black_box(a.clone());
        m / n
    }));
    c.bench_function("Short div", |bencher| bencher.iter(|| {
        let m = black_box(b.clone());
        let n = black_box(183_u8);
        m.div_short(n)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
