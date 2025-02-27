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
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, Mul, MulAssign, Not, Rem,
        Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
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

pub trait Integer:
    Copy
    + Clone
    + Not<Output = Self>
    + Mul<Output = Self>
    + MulAssign
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
{
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn to_le_bytes_vec(self) -> Vec<u8>;
    fn from_le_bytes_ref(value: &[u8]) -> Self;
    const ONE: Self;
    const ZERO: Self;
    const MAX: Self;
    const HALF_COVER: Self;
    const SIZE: usize;
    const BIT_SIZE: usize = size_of::<Self>() * 8;
    const HALF_BIT_SIZE: usize = Self::BIT_SIZE / 2;
}
/*
impl Integer for u8 {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
    fn to_le_bytes_vec(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn from_le_bytes_ref(value: &[u8]) -> Self {
        Self::from_le_bytes(<[u8; size_of::<u8>()]>::try_from(value).unwrap())
    }
    const SIZE: usize = size_of::<u8>();
    const ONE: Self = 1;
    const MAX: Self = u8::MAX;
    const ZERO: Self = 0;
    const HALF_COVER: Self = Self::MAX >> Self::HALF_BIT_SIZE;
}
*/
macro_rules! impl_integer {
    ($u8:ty) => {
        impl Integer for $u8 {
            fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                self.overflowing_add(rhs)
            }
            fn to_le_bytes_vec(self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
            fn from_le_bytes_ref(value: &[u8]) -> Self {
                Self::from_le_bytes(<[u8; size_of::<$u8>()]>::try_from(value).unwrap())
            }
            const SIZE: usize = size_of::<$u8>();
            const ONE: Self = 1;
            const MAX: Self = <$u8>::MAX;
            const ZERO: Self = 0;
            const HALF_COVER: Self = Self::MAX >> Self::HALF_BIT_SIZE;
        }
    };
}
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);

#[derive(Debug)]
pub enum BigNatural<T> {
    NonZero(Rc<Vec<T>>),
    Zero,
}
impl<T: Integer> BigNatural<T> {
    pub fn new() -> Self {
        BigNatural::Zero
    }
    fn add_impl(a: &[T], b: &mut Vec<T>, pre: bool) -> bool {
        assert!(a.len() <= b.len());
        let mut next = a
            .iter()
            .zip(b.iter_mut())
            .fold(pre, |mut next, (&a_d, b_p)| {
                if next {
                    (*b_p, next) = (*b_p).overflowing_add(T::ONE);
                }
                let n1;
                (*b_p, n1) = (*b_p).overflowing_add(a_d);
                next | n1
            });
        for b_p in b.iter_mut().skip(a.len()) {
            if !next {
                break;
            }
            (*b_p, next) = (*b_p).overflowing_add(T::ONE);
        }
        next
    }
    fn add_algo(a: &[T], b: &mut Vec<T>) {
        if Self::add_impl(a, b, false) {
            b.push(T::ONE);
        }
    }
    fn clear_zero(vec: &mut Vec<T>) {
        while Some(&(T::ZERO)) == vec.last() {
            vec.pop();
        }
    }
    fn pop_zero(vec: &mut Vec<T>) {
        if vec.last() == Some(&(T::ZERO)) {
            vec.pop();
        }
    }
    fn sub_impl(a: &mut Vec<T>, b: &[T]) -> bool {
        a.iter_mut().for_each(|p| *p = !*p);
        let success = Self::add_impl(b, a, false);
        a.iter_mut().for_each(|p| *p = !*p);
        Self::clear_zero(a);
        !success
    }
    fn wrap_zero(mut result: Vec<T>) -> Self {
        Self::clear_zero(&mut result);
        if result.is_empty() {
            Self::Zero
        } else {
            result.shrink_to_fit();
            Self::NonZero(Rc::new(result))
        }
    }
    fn wrap_empty(result: Vec<T>) -> Self {
        if result.is_empty() {
            Self::Zero
        } else {
            Self::NonZero(Rc::new(result))
        }
    }
    fn mul_impl(a: &[T], b: &[T]) -> Vec<T> {
        let mut results = repeat(())
            .map(|_| vec![T::ZERO; b.len() + 1])
            .zip(a.iter())
            .map(|(mut line, &a_num)| {
                b.iter()
                    .chain(repeat_n(&(T::ZERO), 1))
                    .zip(line.iter_mut())
                    .fold(T::ZERO, |pre, (&b_num, line_p)| {
                        let mut a_low = a_num & T::HALF_COVER;
                        let a_high = a_num >> T::HALF_BIT_SIZE;
                        let mut b_low = b_num & T::HALF_COVER;
                        let b_high = b_num >> T::HALF_BIT_SIZE;
                        let t0 = a_low * b_low;
                        a_low *= b_high;
                        b_low = a_high * b_low + (t0 >> T::HALF_BIT_SIZE) + (a_low & T::HALF_COVER);
                        let j;
                        (*line_p, j) = ((b_low << T::HALF_BIT_SIZE) | (t0 & T::HALF_COVER))
                            .overflowing_add(pre);
                        a_high * b_high
                            + (a_low >> T::HALF_BIT_SIZE)
                            + (b_low >> T::HALF_BIT_SIZE)
                            + if j { T::ONE } else { T::ZERO }
                    });
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
    fn binary_mul_add(level: u8, results: &mut impl Iterator<Item = Vec<T>>) -> Option<Vec<T>> {
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
    fn shl_add(level: u8, a: &mut Vec<T>, b: &mut Vec<T>) {
        let pos = (1 << level) as usize;
        Self::add_algo(&a[pos..], b);
        a.truncate(pos);
        a.append(b);
    }
}
struct BigNaturalBase(Vec<u8>);
impl<T: Integer> From<T> for BigNaturalBase {
    fn from(value: T) -> Self {
        Self(value.to_le_bytes_vec())
    }
}
impl<T: Integer> From<Vec<T>> for BigNaturalBase {
    fn from(value: Vec<T>) -> Self {
        Self(
            value
                .into_iter()
                .rev()
                .map(|v| v.to_le_bytes_vec())
                .flatten()
                .collect(),
        )
    }
}
impl<T: Integer> From<BigNaturalBase> for BigNatural<T> {
    fn from(value: BigNaturalBase) -> Self {
        let BigNaturalBase(mut vec) = value;
        vec.extend(repeat_n(0, T::SIZE - (vec.len() % T::SIZE)));
        let result = vec
            .chunks_exact(T::SIZE)
            .map(|n| T::from_le_bytes_ref(n))
            .collect();
        Self::wrap_zero(result)
    }
}
impl<T: Integer, F: Integer> From<F> for BigNatural<T> {
    fn from(value: F) -> Self {
        BigNatural::from(BigNaturalBase::from(value))
    }
}
impl<T: Integer, F: Integer> From<Vec<F>> for BigNatural<T> {
    fn from(value: Vec<F>) -> Self {
        BigNatural::from(BigNaturalBase::from(value))
    }
}
impl<T: Integer> BigNatural<T> {
    pub fn to_num<S: Integer>(&self) -> S {
        match self {
            Self::NonZero(r) => {
                let bytes: Vec<u8> = r
                    .iter()
                    .map(|&n| n.to_le_bytes_vec())
                    .into_iter()
                    .flatten()
                    .chain(repeat(0))
                    .take(S::SIZE)
                    .collect();
                S::from_le_bytes_ref(&bytes)
            }
            Self::Zero => S::ZERO,
        }
    }
}
impl<T> Clone for BigNatural<T> {
    fn clone(&self) -> Self {
        match self {
            Self::NonZero(n) => Self::NonZero(Rc::clone(n)),
            _ => Self::Zero,
        }
    }
}
impl<T: Integer> Add for BigNatural<T> {
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
impl<T: Integer> Sub for BigNatural<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::NonZero(b) => match self {
                Self::NonZero(a) => {
                    let mut a = a.to_vec();
                    let success = Self::sub_impl(&mut a, &b);
                    assert!(success, "Sub overflow");
                    Self::wrap_zero(a)
                }
                Self::Zero => Self::Zero,
            },
            Self::Zero => self,
        }
    }
}
impl<T: Ord> Eq for BigNatural<T> {}
impl<T: Ord> PartialEq for BigNatural<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl<T: Ord> PartialOrd for BigNatural<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord> Ord for BigNatural<T> {
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
impl<T: Integer> Mul for BigNatural<T> {
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
impl<T: Integer> BigNatural<T> {
    fn shr_in_size(r: &mut Vec<T>, rhs: usize) {
        let les = T::BIT_SIZE - rhs;
        r.iter_mut().rev().fold(T::ZERO, |pre, now| {
            let next = *now << les;
            *now = pre | (*now >> rhs);
            next
        });
        Self::pop_zero(r);
    }
    fn shl_in_size(r: &mut Vec<T>, rhs: usize, na: usize) {
        let les = T::BIT_SIZE - rhs;
        let next = r.iter_mut().skip(na).fold(T::ZERO, |pre, now| {
            let next = *now >> les;
            *now = pre | (*now << rhs);
            next
        });
        if next != T::ZERO {
            r.push(next);
        }
    }
    fn div_impl(a: &mut Vec<T>, b: &mut Vec<T>) -> Vec<T> {
        let dlen = a.len() - b.len() + 1;
        let mut result = vec![T::ZERO; dlen];
        for (pos, p) in result.iter_mut().enumerate().rev() {
            b.insert(0, T::ZERO);
            for _ in 0..T::BIT_SIZE {
                Self::shr_in_size(b, 1);
                *p <<= 1;
                if b.len() + pos > a.len() {
                    continue;
                }
                let mut subed = Vec::from(&a[pos..]);
                if Self::sub_impl(&mut subed, &b) {
                    a.splice(pos.., subed);
                    *p |= T::ONE;
                }
            }
        }
        Self::pop_zero(&mut result);
        result
    }
    fn div_pack<const M: char>(&self, rhs: &Self) -> (Self, Self) {
        match rhs {
            Self::NonZero(b) => match self {
                Self::NonZero(a) => {
                    if a.len() < b.len() {
                        (
                            Self::Zero,
                            match M {
                                'd' => Self::Zero,
                                _ => Self::NonZero(a.clone()),
                            },
                        )
                    } else {
                        match M {
                            'm' => {
                                let mut m = a.to_vec();
                                Self::div_impl(&mut m, &mut b.to_vec());
                                (Self::Zero, Self::wrap_empty(m))
                            }
                            'd' => (
                                Self::wrap_empty(Self::div_impl(&mut a.to_vec(), &mut b.to_vec())),
                                Self::Zero,
                            ),
                            _ => {
                                let mut m = a.to_vec();
                                let r = Self::div_impl(&mut m, &mut b.to_vec());
                                (Self::wrap_empty(r), Self::wrap_empty(m))
                            }
                        }
                    }
                }
                Self::Zero => (Self::Zero, Self::Zero),
            },
            Self::Zero => panic!("Div 0"),
        }
    }
    pub fn div_mod(&self, rhs: &Self) -> (Self, Self) {
        self.div_pack::<'b'>(rhs)
    }
}
impl<T: Integer> Shr<usize> for BigNatural<T> {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        match self {
            Self::NonZero(r_ori) => {
                let mut result = r_ori.to_vec();
                result.drain(0..(rhs / T::BIT_SIZE));
                let m = rhs % T::BIT_SIZE;
                if m != 0 {
                    Self::shr_in_size(&mut result, m);
                }
                Self::wrap_empty(result)
            }
            n => n,
        }
    }
}
impl<T: Integer> Shl<usize> for BigNatural<T> {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        match self {
            Self::NonZero(r_ori) => {
                let na = rhs / T::BIT_SIZE;
                let mut r = Vec::with_capacity(na + r_ori.len() + 1);
                r.extend(repeat_n(T::ZERO, na));
                r.extend_from_slice(&r_ori);
                let m = rhs % T::BIT_SIZE;
                if m != 0 {
                    Self::shl_in_size(&mut r, m, na);
                }
                Self::NonZero(Rc::new(r))
            }
            n => n,
        }
    }
}
impl<T: Integer> Shr for BigNatural<T> {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        match &rhs {
            Self::NonZero(_) => self >> rhs.to_num::<usize>(),
            Self::Zero => self,
        }
    }
}
impl<T: Integer> Shl for BigNatural<T> {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        match &rhs {
            Self::NonZero(_) => self << rhs.to_num::<usize>(),
            Self::Zero => self,
        }
    }
}
impl<T: Integer> Div for BigNatural<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.div_pack::<'d'>(&rhs).0
    }
}
impl<T: Integer> Rem for BigNatural<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self.div_pack::<'m'>(&rhs).1
    }
}
impl<T: Integer> Display for BigNatural<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dis_list;
        match self {
            Self::NonZero(n) => {
                dis_list = Vec::with_capacity(
                    ((size_of::<T>() * n.len()) as f64 * 2_f64.log10()) as usize + 1,
                );
                let ten = Self::from(10_u8);
                let mut me = self.clone();
                let mut dived;
                while me != Self::Zero {
                    (me, dived) = me.div_mod(&ten);
                    dis_list.push(
                        (match &dived {
                            Self::Zero => 0,
                            Self::NonZero(_) => dived.to_num::<u8>(),
                        } + '0' as u8) as char,
                    );
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
impl<T: Integer> FromStr for BigNatural<T> {
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
            {
                input.push(num?);
            }
            Ok(BigNaturalBase(input).into())
        } else {
            let mut result = BigNatural::new();
            let ten = BigNatural::from(10_u8);
            let codes: Vec<BigNatural<T>> = (0..=9_u8).map(BigNatural::from).collect();
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
/*
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

*/
#[cfg(test)]
mod lib_tests;
