pub mod input {
    use std::{
        fmt::Debug,
        io::{self, Stdin},
        str::{FromStr, SplitWhitespace},
    };
    pub struct Iner {
        input: String,
        stdin: Stdin,
    }
    pub fn new() -> Iner {
        Iner {
            input: String::new(),
            stdin: io::stdin(),
        }
    }
    impl Iner {
        pub fn line(&mut self) -> LineIner {
            self.input.clear();
            self.stdin
                .read_line(&mut self.input)
                .expect("Can't read input!");
            let splited = self.input.split_whitespace();
            LineIner { splited }
        }
    }
    pub struct LineIner<'a> {
        pub splited: SplitWhitespace<'a>,
    }
    impl<'a> LineIner<'a> {
        pub fn parse<T>(&mut self) -> T
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            self.splited
                .next()
                .expect("No more input!")
                .parse()
                .expect("can't parse!")
        }
        pub fn parse_to_iter<T>(self) -> impl Iterator<Item = T> + 'a
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            self.splited.map(|s| s.parse().expect("can't parse!"))
        }
    }
}

pub mod util {
    pub fn format_significantly_f64(value: f64, figures: usize) -> String {
        let digits = match value {
            v if v <= 0.0 => figures,
            _ => figures.saturating_sub(value.abs().log10() as usize + 1),
        };
        format!("{:.*}", digits, value)
    }
}

use util::*;

pub fn bmi_tip(m: f64, h: f64) -> String {
    match m / (h * h){
        v if v < 18.5 => String::from("Underweight"),
        v if v < 24.0 => String::from("Normal"),
        v => format_significantly_f64(v, 6) + "\nOverweight",
    }
}

pub fn main() {
    let mut iner = input::new();
    let mut line_iner = iner.line();
    print!("{}", bmi_tip(line_iner.parse(), line_iner.parse()));
}
