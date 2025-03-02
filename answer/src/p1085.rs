use ojan::*;

struct Day {
    pub hours: u8,
    pub weekday: u8,
}
impl Day {
    pub fn power(&self) -> u8 {
        self.hours * 10 - self.weekday + 7
    }
}

pub fn main() {
    let mut iner = input::new();
    let unhappiest_day = (1..8)
        .map(|weekday| Day {
            hours: iner.line().parse_to_iter::<u8>().take(2).sum(),
            weekday,
        })
        .max_by(|a, b| a.power().cmp(&b.power()))
        .unwrap();
	let day = match unhappiest_day {
		Day { hours, weekday } if hours > 8 => weekday,
		_ => 0,
	};
    print!("{day}");
}
