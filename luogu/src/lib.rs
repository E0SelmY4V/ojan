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
    ops::{Add, Mul, Sub},
    rc::Rc,
    result,
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
            if next {
                (end, next) = b[index].overflowing_add(1);
                b[index] = end;
            }
        }
        (b, next)
    }
    fn add_algo_compare(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
        if a.len() > b.len() {
            Self::add_algo(&b, a)
        } else {
            Self::add_algo(&a, b)
        }
    }
    fn add_algo(a: &Vec<u8>, b: Vec<u8>) -> Vec<u8> {
        let (mut result, next) = Self::add_impl(a, b, false);
        if next {
            result.push(1);
        }
        result
    }
    fn binary_add(level: u8, iter: &mut impl Iterator<Item = Vec<u8>>) -> Option<Vec<u8>> {
        if level == 0 {
            iter.next()
        } else {
            if let Some(mut a) = Self::binary_add(level - 1, iter) {
                if let Some(b_ori) = Self::binary_add(level - 1, iter) {
                    let mut b: Vec<u8> = vec![0; 1 << (level - 1)];
                    b.extend(b_ori.into_iter());
                    Some(Self::add_algo_compare(a, b))
                } else {
                    a.insert(0, 0);
                    Some(a)
                }
            } else {
                None
            }
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
                let result = vec
                    .into_iter()
                    .map(BigNatural::from)
                    .map(|n| n.0.to_vec())
                    .rev()
                    .collect::<Vec<Vec<u8>>>()
                    .concat();
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
                Self(Rc::new(vec.into_iter().map(|n| n as u8).rev().collect()))
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
        while Some(&0) == result.last() {
            result.pop();
        }
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
            if line.last() == Some(&0) {
                line.pop();
            }
            line
        });
        let result = results
            .rev()
            .reduce(|mut result, line| {
                result.insert(0, 0);
                Self::add_algo(&line, result)
            })
            .unwrap_or(vec![]);
        // let result = Self::binary_add((results.len() as f64).log2().ceil() as u8, &mut results)
        //     .unwrap_or(Vec::new());
        Self(Rc::new(result))
    }
}
impl BigNatural {
    pub fn div_short(&self, diver: u8) -> (Self, u8) {
        let diver = diver as u16;
        let mut result = vec![];
        let dived = self.0.iter().rev().fold(0, |pre, &now| {
            let n = ((pre as u16) << 8) + now as u16;
            result.push((n / diver) as u8);
            (n % diver) as u8
        });
        result.reverse();
        if let Some(n) = result.pop() {
            if n != 0 {
                result.push(n);
            }
        }
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
        write!(f, "{}", String::from_iter(dis_list.into_iter()))?;
        Ok(())
    }
}

#[cfg(test)]
mod lib_tests;
