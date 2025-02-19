use ojan_luogu::*;

pub fn main() {
    let mut iner = input::new();
    let mut vec = iner.line().parse_to_iter().collect::<Vec<u8>>();
    vec.sort();
    vec.into_iter().for_each(|n| print!("{n} "));
}
