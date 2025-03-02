use ojan::*;

fn is_leap_year(y: usize) -> usize {
    ((y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)) as usize
}

pub fn main() {
    let mut iner = input::new();
    let mut line_iner = iner.line();
    let y: usize = line_iner.parse();
    let m: usize = line_iner.parse();
    let d = match m {
        2 => 28 + is_leap_year(y),
        _ => 30 + (m % 2) ^ ((m > 7) as usize),
    };
    print!("{d}");
}
