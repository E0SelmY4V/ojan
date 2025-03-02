use ojan::*;

pub fn main() {
	let mut iner = input::new();
	let mut n: u8 = iner.get();
	let mut t: u8 = 1;
	for _ in 0..n {
		for _ in 0..n {
			print!("{t:02}");
			t += 1;
		}
		print!("\n");
		n -= 1;
	}
}
