use ojan::*;

pub fn main() {
	let mut iner = input::new();
	let apples: Vec<u8> = iner.line_iter().collect();
	let height: u8 = iner.get::<u8>() + 30;
	let sum = apples.into_iter().filter(|&n| n <= height).count();
	print!("{sum}");
}
