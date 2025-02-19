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

pub trait SignificantlyFormatable {
    fn format_significantly(&self, figures: usize) -> String;
}
macro_rules! impl_format_significantly {
    ($type:ty) => {
        impl SignificantlyFormatable for $type {
            fn format_significantly(&self, figures: usize) -> String {
                let digits = match self.abs() {
                    v if v >= 1. => figures.saturating_sub(self.abs().log10() as usize + 1),
                    v if v == 0. => figures - 1,
                    v => (-v.log10().floor()) as usize - 1 + figures,
                };
                format!("{:.*}", digits, self)
            }
        }
    };
}
impl_format_significantly!(f32);
impl_format_significantly!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_significantly() {
        assert_eq!("1.000", 1_f64.format_significantly(4));
        assert_eq!("12.2300", 12.23_f64.format_significantly(6));
        assert_eq!("123.000", 123_f64.format_significantly(6));
        assert_eq!("12345", 12345_f64.format_significantly(4));
        assert_eq!("0.1235", 0.123456_f64.format_significantly(4));
        assert_eq!("0.1234", 0.123446_f64.format_significantly(4));
        assert_eq!("0.00034500", 0.000345_f64.format_significantly(5));

        assert_eq!("0.000", 0_f64.format_significantly(4));

        assert_eq!("-1.000", (-1_f64).format_significantly(4));
        assert_eq!("-12.2300", (-12.23_f64).format_significantly(6));
        assert_eq!("-123.000", (-123_f64).format_significantly(6));
        assert_eq!("-12345", (-12345_f64).format_significantly(4));
        assert_eq!("-0.1235", (-0.123456_f64).format_significantly(4));
        assert_eq!("-0.1234", (-0.123446_f64).format_significantly(4));
        assert_eq!("-0.00034500", (-0.000345_f64).format_significantly(5));
    }
}