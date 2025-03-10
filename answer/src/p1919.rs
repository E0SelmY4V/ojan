use ojan::*;

pub fn main() {
    let mut iter = input::new();
    let a: BigNatural<u128> = iter.get();
    let b: BigNatural<u128> = iter.get();
    print!("{}", a * b);
}

