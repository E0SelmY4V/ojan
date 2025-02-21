//! # OJan 库
//!
//! 这个库是 E0SelmY4V 自己用来在各大在线题库上写
//! 不带其他任何 crates 的比赛代码用的。
//!
//! 因为这个库在不断更新，
//! 考虑到正在看这堆文字的你可能也对这个库感兴趣，
//! [这里是这个库的网址](https://github.com/E0SelmY4V/ojan)
//!

use std::{
    fmt::Display,
    iter::repeat,
    num::ParseIntError,
    ops::{Add, Mul, Sub},
    rc::Rc,
    str::FromStr,
};

pub mod input {
    use std::{
        collections::VecDeque,
        fmt::Debug,
        io::{self, Stdin},
        iter,
        str::{FromStr, SplitWhitespace},
    };
    pub struct Iner {
        stdin: Stdin,
        cached: VecDeque<String>,
        line_last: String,
    }
    pub fn new() -> Iner {
        Iner {
            stdin: io::stdin(),
            cached: VecDeque::new(),
            line_last: String::new(),
        }
    }
    impl Iner {
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
        pub fn line(&mut self) -> LineIner {
            self.line_last = self.read_line();
            let splited = self.line_last.split_whitespace();
            LineIner {
                splited,
                line: &self.line_last,
            }
        }
        pub fn line_iter<T>(&mut self) -> impl Iterator<Item = T> + '_
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            self.line().parse_to_iter()
        }
    }
    pub struct LineIner<'a> {
        splited: SplitWhitespace<'a>,
        pub line: &'a str,
    }
    impl<'a> LineIner<'a> {
        #[deprecated(note = "use `iner.get()` instead")]
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

#[derive(Debug)]
pub struct BigNatural(Rc<Vec<u8>>);
impl BigNatural {
    pub fn new() -> Self {
        Self(Rc::new(Vec::new()))
    }
    fn add_impl(a: &Vec<u8>, mut b: Vec<u8>, pre: bool) -> (Vec<u8>, bool) {
        assert!(a.len() <= b.len());
        let mut next = pre;
        for index in 0..a.len() {
            let (ab, n1) = a[index].overflowing_add(b[index]);
            let (end, n2) = ab.overflowing_add(next as u8);
            b[index] = end;
            next = n1 || n2;
        }
        let mut end;
        for index in a.len()..b.len() {
            if !next {
                break;
            }
            (end, next) = b[index].overflowing_add(1);
            b[index] = end;
        }
        (b, next)
    }
    fn add_algo(a: &Vec<u8>, b: Vec<u8>) -> Vec<u8> {
        let (mut result, next) = Self::add_impl(a, b, false);
        if next {
            result.push(1);
        }
        result
    }
    fn binary_mul_add(level: u8, results: &mut impl Iterator<Item = Vec<u8>>) -> Option<Vec<u8>> {
        if level == 0 {
            results.next()
        } else {
            if let Some(a) = Self::binary_mul_add(level - 1, results) {
                Some(if let Some(b) = Self::binary_mul_add(level - 1, results) {
                    Self::shl_add(level - 1, &a, b)
                } else {
                    a
                })
            } else {
                None
            }
        }
    }
    fn shl_add(pos: u8, a: &Vec<u8>, b: Vec<u8>) -> Vec<u8> {
        let mut b_filled: Vec<u8> = vec![0; 1 << pos];
        b_filled.extend(b.into_iter());
        Self::add_algo(a, b_filled)
    }
    fn clear_zero(vec: &mut Vec<u8>) {
        while Some(&0) == vec.last() {
            vec.pop();
        }
    }
    fn pop_zero(vec: &mut Vec<u8>) {
        if vec.last() == Some(&0) {
            vec.pop();
        }
    }
}
macro_rules! impl_big_natural_from {
    ($type:ty, long) => {
        impl From<$type> for BigNatural {
            fn from(mut num: $type) -> Self {
                let mut num_vec = vec![];
                while num != 0 {
                    num_vec.push(num as u8);
                    num = num >> 8_usize;
                }
                Self(Rc::new(num_vec))
            }
        }
        impl From<Vec<$type>> for BigNatural {
            fn from(vec: Vec<$type>) -> Self {
                let mut result = vec
                    .into_iter()
                    .map(BigNatural::from)
                    .map(|n| n.0.to_vec())
                    .rev()
                    .collect::<Vec<Vec<u8>>>()
                    .concat();
                Self::clear_zero(&mut result);
                Self(Rc::new(result))
            }
        }
    };
    ($type:ty, short) => {
        impl From<$type> for BigNatural {
            fn from(num: $type) -> Self {
                Self(Rc::new(vec![num as u8]))
            }
        }
        impl From<Vec<$type>> for BigNatural {
            fn from(vec: Vec<$type>) -> Self {
                let mut result = vec.into_iter().map(|n| n as u8).rev().collect();
                Self::clear_zero(&mut result);
                Self(Rc::new(result))
            }
        }
    };
}
impl_big_natural_from!(u8, short);
impl_big_natural_from!(u16, long);
impl_big_natural_from!(u32, long);
impl_big_natural_from!(u64, long);
impl_big_natural_from!(u128, long);
impl_big_natural_from!(i8, short);
impl_big_natural_from!(i16, long);
impl_big_natural_from!(i32, long);
impl_big_natural_from!(i64, long);
impl_big_natural_from!(i128, long);
impl From<BigNatural> for Vec<u8> {
    fn from(big_natural: BigNatural) -> Self {
        let mut inner = big_natural.0.to_vec();
        inner.reverse();
        inner
    }
}
impl Clone for BigNatural {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl Add for BigNatural {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(Rc::new(if self.0.len() < rhs.0.len() {
            Self::add_algo(&self.0, rhs.0.to_vec())
        } else {
            Self::add_algo(&rhs.0, self.0.to_vec())
        }))
    }
}
impl Sub for BigNatural {
    type Output = Self;
    fn sub(self, b: Self) -> Self::Output {
        assert!(self.0.len() >= b.0.len());
        let rhs: Vec<u8> =
            b.0.iter()
                .map(|&n| !n)
                .chain(repeat(0xff).take(self.0.len() - b.0.len()))
                .collect();
        let (mut result, _) = Self::add_impl(&self.0, rhs, true);
        Self::clear_zero(&mut result);
        Self(Rc::new(result))
    }
}
impl Eq for BigNatural {}
impl PartialEq for BigNatural {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for index in 0..self.0.len() {
            if self.0[index] != other.0[index] {
                return false;
            }
        }
        true
    }
}
impl Mul for BigNatural {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.0.len() == 0 {
            return self;
        }
        let mut results = rhs.0.iter().map(|&b| {
            let mut pre = 0;
            let mut line: Vec<u8> = self
                .0
                .iter()
                .chain(repeat(&0).take(1))
                .map(|&a| {
                    let r = (a as u16) * (b as u16);
                    let n = r + pre as u16;
                    pre = (n >> 8) as u8;
                    n as u8
                })
                .collect();
            Self::pop_zero(&mut line);
            line
        });
        let mut level = 0;
        let mut result = results.next().unwrap_or(vec![]);
        while let Some(r) = Self::binary_mul_add(level, &mut results) {
            result = Self::shl_add(level, &result, r);
            level += 1;
        }
        Self(Rc::new(result))
    }
}
impl BigNatural {
    pub fn div_short(&self, diver: u8) -> (Self, u8) {
        assert!(diver != 0);
        let diver = diver as u16;
        let mut result = vec![];
        let dived = self.0.iter().rev().fold(0, |pre, &now| {
            let n = ((pre as u16) << 8) + now as u16;
            result.push((n / diver) as u8);
            (n % diver) as u8
        });
        result.reverse();
        Self::pop_zero(&mut result);
        (Self(Rc::new(result)), dived)
    }
}
impl Display for BigNatural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut me = self.clone();
        let mut dived;
        let mut dis_list = vec![];
        while me.0.len() != 0 {
            (me, dived) = me.div_short(10);
            dis_list.push((dived + '0' as u8) as char);
        }
        if dis_list.is_empty() {
            dis_list.push('0');
        }
        write!(f, "{}", String::from_iter(dis_list.into_iter().rev()))?;
        Ok(())
    }
}
#[derive(Debug)]
pub enum BigNaturalParseErr {
    NoInput,
    FormatWrong,
    ParseInt,
    WrongChar,
}
impl From<ParseIntError> for BigNaturalParseErr {
    fn from(_: ParseIntError) -> Self {
        BigNaturalParseErr::ParseInt
    }
}
impl FromStr for BigNatural {
    type Err = BigNaturalParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(BigNaturalParseErr::NoInput);
        }
        if s.chars().nth(0).unwrap() == '0' {
            if s.len() == 1 {
                return Ok(BigNatural::new());
            }
            if s.len() < 3 {
                return Err(BigNaturalParseErr::FormatWrong);
            }
            let (radix, chunk_size, cap) = match s.chars().nth(1).unwrap() {
                'x' | 'X' => (16, 2, (s.len() - 2) / 2),
                // 'o' | 'O' => (8, s.len() - 2),
                'b' | 'B' => (2, 8, s.len() - 2),
                _ => return Err(BigNaturalParseErr::FormatWrong),
            };
            let mut input = Vec::with_capacity(cap);
            for num in s[2..]
                .chars()
                .filter(|&n| n != '_')
                .collect::<Vec<char>>()
                .rchunks(chunk_size)
                .map(|chunk| String::from_iter(chunk))
                .map(|s| u8::from_str_radix(&s, radix))
                .rev()
            {
                input.push(num?);
            }
            Ok(BigNatural::from(input))
        } else {
            let mut result = BigNatural::new();
            let mut power = BigNatural::from(1_u8);
            let ten = BigNatural::from(10_u8);
            let codes: Vec<BigNatural> = (0..=9_u8).map(BigNatural::from).collect();
            for c in s.chars().rev() {
                if c == '_' {
                    continue;
                }
                if ('0'..='9').contains(&c) {
                    result = result + codes[c as usize - '0' as usize].clone() * power.clone();
                    power = power * ten.clone();
                } else {
                    return Err(BigNaturalParseErr::WrongChar);
                }
            }
            Ok(BigNatural::from(result))
        }
    }
}

#[cfg(test)]
mod lib_tests;
