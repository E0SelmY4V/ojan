use ojan_luogu::*;

pub fn main() {
	let mut demander = input::demand();
	let mut nums: Vec<u8> = demander.get_many(3).collect();
	nums.sort();
	for alpha in demander.read_line().chars().take(3) {
		print!("{} ", nums[alpha as usize - 'A' as usize]);
	}
}
