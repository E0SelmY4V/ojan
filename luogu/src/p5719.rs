use ojan_luogu::*;

type Num = u32;

pub fn main() {
    let mut iner = input::new();
    let n: Num = iner.get();
    let k: Num = iner.get();
    let mut t = [(0, 0); 2];
    for i in 1..(n + 1) {
        let idx = (i % k != 0) as usize;
        t[idx].0 += 1;
        t[idx].1 += i;
    }
    t.iter()
        .map(|(counter, sum)| *sum as f64 / *counter as f64)
        .for_each(|n| print!("{n:.1} "));
}
