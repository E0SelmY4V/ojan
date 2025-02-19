use ojan_luogu::*;

pub fn bmi_tip(m: f64, h: f64) -> String {
    match m / (h * h) {
        v if v < 18.5 => String::from("Underweight"),
        v if v < 24.0 => String::from("Normal"),
        v => v.format_significantly(6) + "\nOverweight",
    }
}

pub fn main() {
    let mut iner = input::new();
    let mut line_iner = iner.line();
    print!("{}", bmi_tip(line_iner.parse(), line_iner.parse()));
}
