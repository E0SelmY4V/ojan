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
    cmp::Ordering,
    fmt::Display,
    iter::{repeat, repeat_n},
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
pub enum BigNatural {
    NonZero(Rc<Vec<u8>>),
    Zero,
}
impl BigNatural {
    pub fn new() -> Self {
        BigNatural::Zero
    }
    fn add_impl(a: &[u8], b: &mut Vec<u8>, pre: bool) -> bool {
        assert!(a.len() <= b.len());
        let mut next = a
            .iter()
            .zip(b.iter_mut())
            .fold(pre, |mut next, (&a_d, b_p)| {
                if next {
                    (*b_p, next) = (*b_p).overflowing_add(1);
                }
                let n1;
                (*b_p, n1) = (*b_p).overflowing_add(a_d);
                next || n1
            });
        for b_p in b.iter_mut().skip(a.len()) {
            if !next {
                break;
            }
            (*b_p, next) = (*b_p).overflowing_add(1);
        }
        next
    }
    fn add_algo(a: &[u8], b: &mut Vec<u8>) {
        if Self::add_impl(a, b, false) {
            b.push(1);
        }
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
    fn sub_impl(a: &[u8], b: &mut Vec<u8>) -> bool {
        b.iter_mut().for_each(|p| *p = !*p);
        b.extend(repeat_n(u8::MAX, a.len() - b.len()));
        let success = Self::add_impl(a, b, true);
        Self::clear_zero(b);
        success
    }
    fn wrap_zero(mut result: Vec<u8>) -> Self {
        Self::clear_zero(&mut result);
        if result.is_empty() {
            Self::Zero
        } else {
            result.shrink_to_fit();
            Self::NonZero(Rc::new(result))
        }
    }
    fn mul_impl(a: &[u8], b: &[u8]) -> Vec<u8> {
        let mut results =
            repeat(())
                .map(|_| vec![0; b.len() + 1])
                .zip(a.iter())
                .map(|(mut line, &a_num)| {
                    b.iter().chain(repeat_n(&0, 1)).zip(line.iter_mut()).fold(
                        0,
                        |pre, (&b_num, line_p)| {
                            let r = (b_num as u16) * (a_num as u16);
                            let n = r + pre as u16;
                            *line_p = n as u8;
                            (n >> 8) as u8
                        },
                    );
                    Self::pop_zero(&mut line);
                    line
                });
        let mut level = 0;
        let mut result = results.next().unwrap_or_default();
        while let Some(mut r) = Self::binary_mul_add(level, &mut results) {
            Self::shl_add(level, &mut result, &mut r);
            level += 1;
        }
        result
    }
    fn binary_mul_add(level: u8, results: &mut impl Iterator<Item = Vec<u8>>) -> Option<Vec<u8>> {
        if level == 0 {
            results.next()
        } else {
            Self::binary_mul_add(level - 1, results).and_then(|mut a| {
                if let Some(mut b) = Self::binary_mul_add(level - 1, results) {
                    Self::shl_add(level - 1, &mut a, &mut b);
                }
                Some(a)
            })
        }
    }
    fn shl_add(level: u8, a: &mut Vec<u8>, b: &mut Vec<u8>) {
        let pos = (1 << level) as usize;
        Self::add_algo(&a[pos..], b);
        a.truncate(pos);
        a.extend(b.iter());
    }
}
macro_rules! impl_big_natural_from {
    (_impl, $num:ident: $type:ty) => {{
        let mut num_vec = Vec::with_capacity(size_of::<$type>());
        while $num != 0 {
            num_vec.push($num as u8);
            $num = $num >> 8;
        }
        num_vec
    }};
    (long, $type:ty) => {
        impl From<$type> for BigNatural {
            fn from(mut num: $type) -> Self {
                Self::wrap_zero(impl_big_natural_from!(_impl, num: $type))
            }
        }
        impl From<Vec<$type>> for BigNatural {
            fn from(vec: Vec<$type>) -> Self {
                Self::wrap_zero(vec
                    .into_iter()
                    .map(|mut num| impl_big_natural_from!(_impl, num: $type))
                    .rev()
                    .collect::<Vec<Vec<u8>>>()
                    .concat()
                )
            }
        }
    };
    (short, $type:ty) => {
        impl From<$type> for BigNatural {
            fn from(num: $type) -> Self {
                if num == 0 {
                    Self::Zero
                } else {
                    Self::NonZero(Rc::new(vec![num as u8]))
                }
            }
        }
        impl From<Vec<$type>> for BigNatural {
            fn from(vec: Vec<$type>) -> Self {
                Self::wrap_zero(vec.into_iter().map(|n| n as u8).rev().collect())
            }
        }
    };
}
impl_big_natural_from!(short, u8);
impl_big_natural_from!(long, u16);
impl_big_natural_from!(long, u32);
impl_big_natural_from!(long, u64);
impl_big_natural_from!(long, u128);
impl_big_natural_from!(short, i8);
impl_big_natural_from!(long, i16);
impl_big_natural_from!(long, i32);
impl_big_natural_from!(long, i64);
impl_big_natural_from!(long, i128);
impl Clone for BigNatural {
    fn clone(&self) -> Self {
        match self {
            Self::NonZero(n) => Self::NonZero(Rc::clone(n)),
            _ => Self::Zero,
        }
    }
}
impl Add for BigNatural {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::NonZero(a), Self::NonZero(b)) => {
                let (small, mut big) = if a.len() < b.len() {
                    (a, b.to_vec())
                } else {
                    (b, a.to_vec())
                };
                Self::add_algo(&small, &mut big);
                Self::NonZero(Rc::new(big))
            }
            (Self::Zero, n) => n,
            (n, Self::Zero) => n,
        }
    }
}
impl Sub for BigNatural {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::NonZero(rhs) => match self {
                Self::NonZero(a) => {
                    let mut b = rhs.to_vec();
                    let success = Self::sub_impl(&a, &mut b);
                    assert!(success, "Sub overflow");
                    Self::wrap_zero(b)
                }
                Self::Zero => Self::Zero,
            },
            Self::Zero => self,
        }
    }
}
impl Eq for BigNatural {}
impl PartialEq for BigNatural {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl PartialOrd for BigNatural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BigNatural {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::NonZero(a), Self::NonZero(b)) => match a.len().cmp(&b.len()) {
                Ordering::Equal => a.iter().rev().cmp(b.iter().rev()),
                n => n,
            },
            (Self::NonZero(_), Self::Zero) => Ordering::Greater,
            (Self::Zero, Self::NonZero(_)) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}
impl Mul for BigNatural {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::NonZero(ya), Self::NonZero(yb)) => {
                Self::NonZero(Rc::new(if ya.len() > yb.len() {
                    Self::mul_impl(&yb, &ya)
                } else {
                    Self::mul_impl(&ya, &yb)
                }))
            }
            _ => Self::Zero,
        }
    }
}
impl BigNatural {
    pub fn div_short(&self, diver: u8) -> (Self, u8) {
        assert!(diver != 0);
        match self {
            Self::NonZero(n) => {
                let diver = diver as u16;
                let mut result = vec![0; n.len()];
                let dived = n
                    .iter()
                    .rev()
                    .zip(result.iter_mut())
                    .fold(0, |pre, (&now, r_p)| {
                        let n = ((pre as u16) << 8) + now as u16;
                        *r_p = (n / diver) as u8;
                        (n % diver) as u8
                    });
                result.reverse();
                Self::pop_zero(&mut result);
                (
                    if result.is_empty() {
                        Self::Zero
                    } else {
                        Self::NonZero(Rc::new(result))
                    },
                    dived,
                )
            }
            _ => (Self::Zero, 0),
        }
    }
}
impl Display for BigNatural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dis_list;
        match self {
            Self::NonZero(n) => {
                dis_list = Vec::with_capacity(
                    ((size_of::<u8>() * n.len()) as f64 * 2_f64.log10()) as usize + 1,
                );
                let mut me = self.clone();
                let mut dived;
                while me != Self::Zero {
                    (me, dived) = me.div_short(10);
                    dis_list.push((dived + '0' as u8) as char);
                }
            }
            Self::Zero => {
                dis_list = vec!['0'];
            }
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
                'x' | 'X' => (16, 2, (s.len() - 2) / 2 + 1),
                // 'o' | 'O' => (8, s.len() - 2),
                'b' | 'B' => (2, 8, (s.len() - 2) / 8 + 1),
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
            Ok(input.into())
        } else {
            let mut result = BigNatural::new();
            let ten = BigNatural::from(10_u8);
            let codes: Vec<BigNatural> = (0..=9_u8).map(BigNatural::from).collect();
            for c in s.chars().filter(|&n| n != '_') {
                if ('0'..='9').contains(&c) {
                    result = result * ten.clone() + codes[c as usize - '0' as usize].clone();
                } else {
                    return Err(BigNaturalParseErr::WrongChar);
                }
            }
            Ok(result.into())
        }
    }
}

pub enum BigInteger {
    NonZero { sign: bool, value: Rc<Vec<u8>> },
    Zero,
}
impl BigInteger {
    pub fn new() -> Self {
        BigInteger::Zero
    }
}
macro_rules! impl_big_integer_from {
    (u, $type:ty) => {
        impl_big_integer_from!(bind, $type, |_| false);
    };
    (i, $type:ty) => {
        impl_big_integer_from!(bind, $type, |&value| value < 0);
    };
    (bind, $type:ty, $sign:expr) => {
        impl_big_integer_from!($type, $sign);
        impl_big_integer_from!(Vec<$type>, |_| false);
    };
    ($type:ty, $sign:expr) => {
        impl From<$type> for BigInteger {
            fn from(inp: $type) -> Self {
                let sign = $sign(&inp);
                let big_natural = BigNatural::from(inp);
                match big_natural {
                    BigNatural::NonZero(value) => Self::NonZero { sign, value },
                    BigNatural::Zero => Self::Zero,
                }
            }
        }
    };
}
impl_big_integer_from!(u, u8);
impl_big_integer_from!(u, u16);
impl_big_integer_from!(u, u32);
impl_big_integer_from!(u, u64);
impl_big_integer_from!(u, u128);
impl_big_integer_from!(i, i8);
impl_big_integer_from!(i, i16);
impl_big_integer_from!(i, i32);
impl_big_integer_from!(i, i64);
impl_big_integer_from!(i, i128);
impl Clone for BigInteger {
    fn clone(&self) -> Self {
        match self {
            Self::NonZero { sign, value } => Self::NonZero {
                sign: *sign,
                value: value.clone(),
            },
            Self::Zero => Self::Zero,
        }
    }
}
impl Add for BigInteger {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                Self::NonZero {
                    sign: sign_a,
                    value: value_a,
                },
                Self::NonZero {
                    sign: sign_b,
                    value: value_b,
                },
            ) => {
                if sign_a ^ sign_b {
                    let a = BigNatural::NonZero(value_a);
                    let b = BigNatural::NonZero(value_b);
                    let (small, big, sign) = if a > b {
                        (b, a, sign_a)
                    } else {
                        (a, b, sign_b)
                    };
                    Self::NonZero {
                        sign,
                        value: match big - small {
                            BigNatural::NonZero(n) => n,
                            BigNatural::Zero => panic!("Sub wrong"),
                        },
                    }
                } else {
                    Self::NonZero {
                        sign: sign_a,
                        value: match BigNatural::NonZero(value_a) + BigNatural::NonZero(value_b) {
                            BigNatural::NonZero(n) => n,
                            BigNatural::Zero => panic!("Add wrong"),
                        },
                    }
                }
            }
            (n, Self::Zero) => n,
            (Self::Zero, n) => n,
        }
    }
}

#[cfg(test)]
mod lib_tests;
