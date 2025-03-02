use ojan::*;

pub fn main() {
	let mut iner = input::new();
	let mut nums: Vec<u8> = iner.line_iter().collect();
	nums.sort();
	for alpha in iner.read_line().chars().take(3) {
		print!("{} ", nums[alpha as usize - 'A' as usize]);
	}
}
