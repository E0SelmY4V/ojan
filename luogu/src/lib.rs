//! # OJan 库
//!
//! 这个库是 E0SelmY4V 自己用来在各大在线题库上写
//! 不带其他任何 crates 的比赛代码用的。
//!
//! 因为这个库在不断更新，
//! 考虑到正在看这堆文字的你可能也对这个库感兴趣，
//! [这里是这个库的网址](https://github.com/E0SelmY4V/ojan)
//!

pub mod input {
    use std::{
        collections::VecDeque,
        fmt::Debug,
        io::{self, Stdin},
        iter,
        str::{FromStr, SplitWhitespace},
    };
    pub struct Iner {
        input: String,
        stdin: Stdin,
    }
    #[deprecated]
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
            LineIner {
                splited,
                line: &self.input,
            }
        }
    }
    pub struct LineIner<'a> {
        splited: SplitWhitespace<'a>,
        pub line: &'a str,
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
    pub struct Demander {
        stdin: Stdin,
        cached: VecDeque<String>,
    }
    pub fn demand() -> Demander {
        Demander {
            stdin: io::stdin(),
            cached: VecDeque::new(),
        }
    }
    impl Demander {
        pub fn get<T>(&mut self) -> T
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            if let Some(data) = self.cached.pop_front() {
                data.parse().expect(&format!("can't parse {data}"))
            } else {
                self.cached = self
                    .read_line()
                    .split_whitespace()
                    .map(|s| String::from(s))
                    .collect();
                self.get()
            }
        }
        pub fn get_many<'a, T>(&'a mut self, num: usize) -> impl Iterator<Item = T> + 'a
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            let mut counter = 0;
            iter::from_fn(move || {
                if counter < num {
                    counter += 1;
                    Some(self.get::<T>())
                } else {
                    None
                }
            })
        }
        pub fn read_line(&self) -> String {
            let mut input_str = String::new();
            self.stdin.read_line(&mut input_str).expect("Can't read");
            input_str
        }
    }
}

pub trait SignificantlyFormatable {
    fn format_significantly(&self, figures: usize) -> String;
}
macro_rules! impl_format_significantly {
    ($type: ty) => {
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

pub trait Gcdable {
    fn gcd(&self, other: Self) -> Self;
    fn lcm(&self, other: Self) -> Self;
}
macro_rules! impl_gcd {
    ($type: ty) => {
        impl Gcdable for $type {
            fn gcd(&self, mut b: Self) -> Self {
                assert!(*self != 0 && b != 0);
                let mut a = *self;
                impl_gcd!(a, b)
            }
            fn lcm(&self, mut b: Self) -> Self {
                if *self == 0 || b == 0 {
                    0
                } else {
                    let mut a = *self;
                    (a * b) / impl_gcd!(a, b)
                }
            }
        }
    };
    ($a: ident, $b: ident) => {{
        while $b != 0 {
            let temp = $b;
            $b = $a % $b;
            $a = temp;
        }
        $a
    }};
}
impl_gcd!(u8);
impl_gcd!(u16);
impl_gcd!(u32);
impl_gcd!(u64);
impl_gcd!(u128);
impl_gcd!(i8);
impl_gcd!(i16);
impl_gcd!(i32);
impl_gcd!(i64);
impl_gcd!(i128);

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
    #[test]
    fn gcd() {
        assert_eq!(1, 3.gcd(2));
        assert_eq!(4, 16.gcd(12));
        assert_eq!(11, 77.gcd(66));
        assert_eq!(80, 800.gcd(880));
        assert_eq!(1, 163.gcd(79));

        assert_eq!(6, 2.lcm(3));
        assert_eq!(12, 4.lcm(6));
        assert_eq!(30, 10.lcm(15));
    }
}
