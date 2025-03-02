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
    fmt::{Debug, Display},
    iter::{repeat, repeat_n, RepeatN},
    mem::swap,
    num::ParseIntError,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign
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
fn gcd_impl<T: Integer>(mut a: T, mut b: T) -> T {
        while b != T::ZERO {
            a %= b;
            swap(&mut a, &mut b);
        }
        a
}

impl<T: Integer> Gcdable for T {
    fn gcd(&self, b: Self) -> Self {
        assert!(*self != T::ZERO && b != T::ZERO);
        gcd_impl(*self, b)
    }
    fn lcm(&self, b: Self) -> Self {
        if *self == T::ZERO || b == T::ZERO {
            T::ZERO
        } else {
            let a = *self;
            (a * b) / gcd_impl(a, b)
        }
    }
}

pub trait IterDeref {
    type Item;
    fn iter_deref(&self) -> impl Iterator<Item = Self::Item>;
}
impl<T: Copy> IterDeref for [T] {
    type Item = T;
    fn iter_deref(&self) -> impl Iterator<Item = Self::Item> {
        self.iter().map(|&n| n)
    }
}

pub fn repeat_when<T: Clone>(value: T, expr: bool) -> RepeatN<T> {
    repeat_n(value, if expr { 1 } else { 0 })
}

pub trait Integer:
    Copy
    + Clone
    + Not<Output = Self>
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
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
    + Debug
    + Display
    + Default
    + 'static
{
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn to_le_bytes_vec(self) -> Vec<u8>;
    fn from_le_bytes_ref(value: &[u8]) -> Self;
    fn leading_zeros(self) -> u32;
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    const ONE: Self;
    const ZERO: Self;
    const MAX: Self;
    const HALF_COVER: Self;
    const SIZE: usize = size_of::<Self>();
    const BITS: usize = size_of::<Self>() * 8;
    const HALF_BIT_SIZE: usize = Self::BITS / 2;
    const NOTSIGN_COVER: Self;
}
macro_rules! impl_integer {
    (i, $t:ty) => {
        impl_integer!($t);
    };
    (u, $t:ty) => {
        impl_integer!($t);
    };
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
            fn leading_zeros(self) -> u32 {
                self.leading_zeros()
            }
            fn checked_shr(self, rhs: u32) -> Option<Self> {
                self.checked_shr(rhs)
            }
            fn checked_shl(self, rhs: u32) -> Option<Self> {
                self.checked_shl(rhs)
            }
            const ONE: Self = 1;
            const MAX: Self = <$u8>::MAX;
            const ZERO: Self = 0;
            const HALF_COVER: Self = Self::MAX >> Self::HALF_BIT_SIZE;
            const NOTSIGN_COVER: Self = <$u8>::MAX >> 1;
        }
    };
}
impl_integer!(u, u8);
impl_integer!(u, u16);
impl_integer!(u, u32);
impl_integer!(u, u64);
impl_integer!(u, u128);
impl_integer!(u, usize);
impl_integer!(i, i8);
impl_integer!(i, i16);
impl_integer!(i, i32);
impl_integer!(i, i64);
impl_integer!(i, i128);
impl_integer!(i, isize);

#[derive(Debug, Clone)]
pub struct BigNatural<T>(Rc<Vec<T>>);
impl<T> BigNatural<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self(Rc::new(vec))
    }
}
impl<T: Integer> BigNatural<T> {
    fn add_impl<'c>(
        a: impl Iterator<Item = T>,
        mut b: impl Iterator<Item = &'c mut T>,
        pre: bool,
    ) -> bool
    where
        T: 'c,
    {
        let mut next = a.zip(&mut b).fold(pre, |mut next, (a_d, b_p)| {
            if next {
                (*b_p, next) = (*b_p).overflowing_add(T::ONE);
            }
            let n1;
            (*b_p, n1) = (*b_p).overflowing_add(a_d);
            next | n1
        });
        for b_p in b {
            if !next {
                break;
            }
            (*b_p, next) = (*b_p).overflowing_add(T::ONE);
        }
        next
    }
    fn add_algo(a: impl Iterator<Item = T>, b: &mut Vec<T>) {
        if Self::add_impl(a, b.iter_mut(), false) {
            b.push(T::ONE);
        }
    }
    fn clear_value(vec: &mut Vec<T>, value: T) {
        while vec.last().is_some_and(|&n| n == value) {
            vec.pop();
        }
    }
    fn pop_zero(vec: &mut Vec<T>) {
        if vec.last().is_some_and(|&n| n == T::ZERO) {
            vec.pop();
        }
    }
    fn sub_impl(a: &mut Vec<T>, b: impl Iterator<Item = T>) -> bool {
        a.iter_mut().for_each(|p| *p = !*p);
        let success = !Self::add_impl(b, a.iter_mut(), false);
        Self::clear_value(a, T::MAX);
        a.iter_mut().for_each(|p| *p = !*p);
        success
    }
    fn wrap_zero(mut result: Vec<T>) -> Self {
        Self::clear_value(&mut result, T::ZERO);
        result.shrink_to_fit();
        Self::new(result)
    }
    fn mul_impl(a: &[T], b: &[T]) -> Vec<T> {
        let mut result = vec![T::ZERO; a.len() + b.len()];
        for (pos, results) in a
            .iter()
            .map(|&a_num| {
                let mut pre = T::ZERO;
                b.iter_deref()
                    .chain(repeat_n(T::ZERO, 1))
                    .map(move |b_num| {
                        let mut a_low = a_num & T::HALF_COVER;
                        let a_high = a_num >> T::HALF_BIT_SIZE;
                        let mut b_low = b_num & T::HALF_COVER;
                        let b_high = b_num >> T::HALF_BIT_SIZE;
                        let t0 = a_low * b_low;
                        a_low *= b_high;
                        b_low = a_high * b_low + (t0 >> T::HALF_BIT_SIZE) + (a_low & T::HALF_COVER);
                        let (r, j) = ((b_low << T::HALF_BIT_SIZE) | (t0 & T::HALF_COVER))
                            .overflowing_add(pre);
                        pre = a_high * b_high
                            + (a_low >> T::HALF_BIT_SIZE)
                            + (b_low >> T::HALF_BIT_SIZE)
                            + if j { T::ONE } else { T::ZERO };
                        r
                    })
            })
            .enumerate()
        {
            Self::add_impl(results, result.iter_mut().skip(pos), false);
        }
        Self::pop_zero(&mut result);
        result
    }
    pub fn to_num<S: Integer>(&self) -> S {
        let bytes: Vec<u8> = self
            .0
            .iter()
            .map(|n| n.to_le_bytes_vec())
            .flatten()
            .chain(repeat(0))
            .take(S::SIZE)
            .collect();
        S::from_le_bytes_ref(&bytes)
    }
    fn shr_in_size_iter<'a>(r: &'a [T], rhs: usize) -> impl Iterator<Item = T> + 'a {
        let les = (T::BITS - rhs) as u32;
        let tail = r.last().copied().unwrap_or_default() >> rhs;
        r.windows(2)
            .map(move |s| s[1].checked_shl(les).unwrap_or_default() | (s[0] >> rhs))
            .chain(repeat_when(tail, tail != T::ZERO))
    }
    fn shl_in_size_iter<'a>(r: &'a [T], rhs: usize) -> impl Iterator<Item = T> + 'a {
        let les = (T::BITS - rhs) as u32;
        let tail = r
            .last()
            .and_then(|n| n.checked_shr(les))
            .unwrap_or_default();
        repeat_when(r.first().copied().unwrap_or_default() << rhs, r.len() > 0)
            .chain(
                r.windows(2)
                    .map(move |s| s[0].checked_shr(les).unwrap_or_default() | (s[1] << rhs)),
            )
            .chain(repeat_when(tail, tail != T::ZERO))
    }
    fn div_impl_check(dis: usize, a: &mut Vec<T>, b: &[T], result: &mut [T]) -> bool {
        let na = dis / T::BITS;
        let m = dis % T::BITS;
        let mut subed = Vec::from(&a[na..]);
        let success = Self::sub_impl(&mut subed, Self::shl_in_size_iter(b, m));
        if success {
            a.splice(na.., subed);
            result[na] |= T::ONE << m;
        }
        success
    }
    fn div_impl(a: &mut Vec<T>, b: &[T]) -> Vec<T> {
        let dlen = a.len() - b.len() + 1;
        let bblen = b.len() * T::BITS - b.last().unwrap().leading_zeros() as usize;
        let mut result = vec![T::ZERO; dlen];
        while let Some(dis) = a
            .last()
            .and_then(|n| (a.len() * T::BITS - n.leading_zeros() as usize).checked_sub(bblen))
        {
            if !Self::div_impl_check(dis, a, b, &mut result) {
                if let Some(dis) = dis.checked_sub(1) {
                    Self::div_impl_check(dis, a, b, &mut result);
                } else {
                    break;
                }
            }
        }
        Self::pop_zero(&mut result);
        result
    }
    fn div_mod_pack<const M: char>(a: &Self, b: &Self) -> (Self, Self) {
        let Self(a) = a;
        let Self(b) = b;
        if a.len() < b.len() {
            let d = Self::default();
            (
                d.clone(),
                match M {
                    'd' => d.clone(),
                    _ => Self(Rc::clone(a)),
                },
            )
        } else {
            match M {
                'm' => {
                    let mut m = a.to_vec();
                    Self::div_impl(&mut m, b);
                    (Self::default(), Self::new(m))
                }
                'd' => (
                    Self::new(Self::div_impl(&mut a.to_vec(), b)),
                    Self::default(),
                ),
                _ => {
                    let mut m = a.to_vec();
                    let r = Self::div_impl(&mut m, b);
                    (Self::new(r), Self::new(m))
                }
            }
        }
    }
    pub fn div_mod(&self, rhs: &Self) -> (Self, Self) {
        Self::div_mod_pack::<'b'>(self, rhs)
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
            .chunks(T::SIZE)
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
impl<T> Default for BigNatural<T> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
impl<T: Integer> Add for BigNatural<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Self(mut a) = self;
        let Self(mut b) = rhs;
        if a.len() > b.len() {
            swap(&mut a, &mut b);
        }
        let mut result = b.to_vec();
        Self::add_algo(a.iter_deref(), &mut result);
        Self(Rc::new(result))
    }
}
impl<T: Integer> Sub for BigNatural<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut a = self.0.to_vec();
        let success = Self::sub_impl(&mut a, rhs.0.iter_deref());
        assert!(success, "Sub overflow");
        Self::wrap_zero(a)
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
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => self.0.iter().rev().cmp(other.0.iter().rev()),
            n => n,
        }
    }
}
impl<T: Integer> Mul for BigNatural<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let Self(mut a) = self;
        let Self(mut b) = rhs;
        if a.is_empty() || b.is_empty() {
            Self::default()
        } else {
            if a.len() > b.len() {
                swap(&mut a, &mut b);
            }
            Self(Rc::new(Self::mul_impl(&a, &b)))
        }
    }
}
impl<T: Integer> BigNatural<T> {}
impl<T: Integer> Shr<usize> for BigNatural<T> {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        let na = rhs / T::BITS;
        if na >= self.0.len() {
            Self::default()
        } else {
            Self::new(Self::shr_in_size_iter(&self.0[na..], rhs % T::BITS).collect())
        }
    }
}
impl<T: Integer> Shl<usize> for BigNatural<T> {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        if self.0.is_empty() {
            self
        } else {
            Self::new(
                repeat_n(T::ZERO, rhs / T::BITS)
                    .chain(Self::shl_in_size_iter(&self.0, rhs % T::BITS))
                    .collect(),
            )
        }
    }
}
impl<T: Integer> Shr for BigNatural<T> {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        self >> rhs.to_num::<usize>()
    }
}
impl<T: Integer> Shl for BigNatural<T> {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        self << rhs.to_num::<usize>()
    }
}
impl<T: Integer> Div for BigNatural<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::div_mod_pack::<'d'>(&self, &rhs).0
    }
}
impl<T: Integer> Rem for BigNatural<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::div_mod_pack::<'m'>(&self, &rhs).1
    }
}
impl<T: Integer> Display for BigNatural<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "0")?;
        } else {
            let mut dis_list = Vec::with_capacity(size_of::<T>() * self.0.len() * 3 / 10 + 1);
            let ten = Self::from(10_u8);
            let mut me = self.clone();
            let mut dived;
            while !me.0.is_empty() {
                (me, dived) = me.div_mod(&ten);
                dis_list.push((dived.to_num::<u8>() + '0' as u8) as char);
            }
            write!(f, "{}", String::from_iter(dis_list.into_iter().rev()))?;
        }
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
        if s.chars().nth(0).is_some_and(|n| n == '0') {
            if s.len() == 1 {
                return Ok(Self::default());
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
            let mut result = Self::default();
            let ten = Self::from(10_u8);
            let codes: Vec<BigNatural<T>> = (0..=9_u8).map(Self::from).collect();
            for c in s.chars().filter(|&n| n != '_') {
                if c.is_digit(10) {
                    result = codes[c as usize - '0' as usize].clone() + ten.clone() * result;
                } else {
                    return Err(BigNaturalParseErr::WrongChar);
                }
            }
            Ok(result)
        }
    }
}

#[derive(Clone, Debug)]
pub struct BigInteger<T>(bool, BigNatural<T>);
impl<T> BigInteger<T> {
    pub fn new(sign: bool, big_natural: BigNatural<T>) -> Self {
        Self(sign, big_natural)
    }
    fn check_sign(&self) -> bool {
        self.0 && !self.1 .0.is_empty()
    }
}
macro_rules! impl_big_integer_from {
    (u, $t:ty) => {
        impl<T: Integer> From<$t> for BigInteger<T> {
            fn from(value: $t) -> Self {
                Self(false, <BigNatural<T>>::from(value))
            }
        }
    };
    (i, $t:ty) => {
        impl<T: Integer> From<$t> for BigInteger<T> {
            fn from(mut value: $t) -> Self {
                let sign = value < 0;
                if sign {
                    value = -value;
                }
                Self(sign, <BigNatural<T>>::from(value))
            }
        }
    };
}
impl_big_integer_from!(u, u8);
impl_big_integer_from!(u, u16);
impl_big_integer_from!(u, u32);
impl_big_integer_from!(u, u64);
impl_big_integer_from!(u, u128);
impl_big_integer_from!(u, usize);
impl_big_integer_from!(i, i8);
impl_big_integer_from!(i, i16);
impl_big_integer_from!(i, i32);
impl_big_integer_from!(i, i64);
impl_big_integer_from!(i, i128);
impl_big_integer_from!(i, isize);
impl<T: Ord> PartialOrd for BigInteger<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord> Ord for BigInteger<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let sign_a = self.check_sign();
        other.check_sign().cmp(&sign_a).then_with(|| {
            if sign_a {
                other.1.cmp(&self.1)
            } else {
                self.1.cmp(&other.1)
            }
        })
    }
}
impl<T: Ord> PartialEq for BigInteger<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl<T: Ord> Eq for BigInteger<T> {}
impl<T: Integer> Display for BigInteger<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.check_sign() {
            write!(f, "-")?;
        }
        Display::fmt(&self.1, f)?;
        Ok(())
    }
}
impl<T: Integer> Add for BigInteger<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Self(sign_a, mut a) = self;
        let Self(sign_b, mut b) = rhs;
        if sign_a ^ sign_b {
            let overflow = a < b;
            if overflow {
                swap(&mut a, &mut b);
            }
            Self(overflow ^ sign_a, a - b)
        } else {
            Self(sign_a, a + b)
        }
    }
}
impl<T> Neg for BigInteger<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(!self.0, self.1)
    }
}
impl<T: Integer> Sub for BigInteger<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl<T: Integer> Mul for BigInteger<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let Self(sign_a, a) = self;
        let Self(sign_b, b) = rhs;
        Self(sign_a ^ sign_b, a * b)
    }
}
impl<T: Integer> Div for BigInteger<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let Self(sign_a, a) = self;
        let Self(sign_b, b) = rhs;
        Self(sign_a ^ sign_b, a / b)
    }
}
impl<T: Integer> Rem for BigInteger<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let Self(sign_a, a) = self;
        let Self(_, b) = rhs;
        Self(sign_a, a % b)
    }
}
impl<T: Integer> BigInteger<T> {
    pub fn div_mod(&self, rhs: &Self) -> (Self, Self) {
        let Self(sign_a, a) = self;
        let Self(sign_b, b) = rhs;
        let (dived, moded) = a.div_mod(b);
        (Self(*sign_a ^ *sign_b, dived), Self(*sign_a, moded))
    }
}
impl<T: Integer> FromStr for BigInteger<T> {
    type Err = BigNaturalParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign = s.chars().nth(0).is_some_and(|n| n == '-');
        Ok(Self(sign, (if sign { &s[1..] } else { s }).parse()?))
    }
}

#[cfg(test)]
mod lib_tests;
