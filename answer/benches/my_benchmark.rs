use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ojan::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let a: BigNatural<u8> = "0x919939391982839494992992918188181839abbccbfbbffbaba"
        .parse()
        .unwrap();
    let b: BigNatural<u8> = "0x9198484736626515151526378383939499503abbcbbabbfbbabfb192"
        .parse()
        .unwrap();
    let d: BigNatural<u8> = "199392".parse().unwrap();
    let tail = 42;
    let mut v = vec![99; 2];
    v.append(&mut vec![tail; 3]);
    c.bench_function("Big * Small", |bencher| {
        bencher.iter(|| {
            let m = black_box(b.clone());
            let n = black_box(d.clone());
            m * n
        })
    });
    c.bench_function("Small * Big", |bencher| {
        bencher.iter(|| {
            let m = black_box(d.clone());
            let n = black_box(b.clone());
            m * n
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
