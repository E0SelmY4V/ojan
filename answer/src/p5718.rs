use ojan::*;

pub fn main() {
	let mut iner = input::new();
	iner.read_line();
	let min = iner.line_iter::<usize>().min().unwrap();
	print!("{min}");
}
