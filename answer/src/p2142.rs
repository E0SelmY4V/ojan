use ojan::*;

pub fn main() {
    let mut iter = input::new();
    let a: BigInteger<usize> = iter.get();
    let b: BigInteger<usize> = iter.get();
    print!("{}", a - b);
}

