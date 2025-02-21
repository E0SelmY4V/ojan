use ojan_luogu::*;

pub fn main() {
	let mut iner = input::new();
	let mut n: usize = iner.get();
	let mut day: usize = 1;
	while n != 1 {
		n /= 2;
		day += 1;
	}
	print!("{day}");
}
