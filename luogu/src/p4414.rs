use ojan_luogu::*;

pub fn main() {
	let mut iner = input::new();
	let mut nums: Vec<u8> = iner.line().parse_to_iter().collect();
	nums.sort();
	for alpha in iner.line().line.chars().take(3) {
		print!("{} ", nums[alpha as usize - 'A' as usize]);
	}
}
