use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ojan::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let a: BigNatural<u8> = "3".repeat(999).parse().unwrap();
    let b: BigNatural<u8> = "8".repeat(9999).parse().unwrap();
    let d: BigNatural<u8> = "199392".parse().unwrap();
    let tail = 42;
    let mut v = vec![99; 2];
    v.append(&mut vec![tail; 3]);
    c.bench_function("crazy div", |bencher| {
        bencher.iter(|| {
            let m = black_box(b.clone());
            let n = black_box(d.clone());
            m / n
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
