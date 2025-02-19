use ojan_luogu::*;

pub fn main() {
    let mut iner = input::new();
    let mut lines: Vec<u64> = iner.line().parse_to_iter().take(3).collect();
    lines.sort();
    let a = lines[0];
    let c = lines[2];
    let gcd = a.gcd(c);
    print!("{}/{}", a / gcd, c / gcd);
}
