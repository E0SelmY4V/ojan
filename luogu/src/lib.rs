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

#[derive(Debug)]
pub enum BigNatural<T> {
    NonZero(Rc<Vec<T>>),
    Zero,
}
impl<T: Integer> BigNatural<T> {
    pub fn new() -> Self {
        BigNatural::Zero
    }
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
impl<T: Integer> BigNatural<T> {
    pub fn to_num<S: Integer>(&self) -> S {
        match self {
            Self::NonZero(r) => {
                let bytes: Vec<u8> = r
                    .iter()
                    .map(|n| n.to_le_bytes_vec())
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
            (Self::NonZero(mut a), Self::NonZero(mut b)) => {
                if a.len() > b.len() {
                    swap(&mut a, &mut b);
                }
                let mut result = b.to_vec();
                Self::add_algo(a.iter_deref(), &mut result);
                Self::NonZero(Rc::new(result))
            }
            (Self::Zero, n) => n,
            (n, Self::Zero) => n,
        }
    }
}
impl<T: Integer> Sub for BigNatural<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::NonZero(a), Self::NonZero(b)) => {
                let mut a = a.to_vec();
                let success = Self::sub_impl(&mut a, b.iter_deref());
                assert!(success, "Sub overflow");
                Self::wrap_zero(a)
            }
            (n, Self::Zero) => n,
            (Self::Zero, Self::NonZero(_)) => panic!("Sub overflow"),
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
impl<T: Ord> BigNatural<T> {
    fn ord_impl(a: &[T], b: &[T]) -> Ordering {
        match a.len().cmp(&b.len()) {
            Ordering::Equal => a.iter().rev().cmp(b.iter().rev()),
            n => n,
        }
    }
}
impl<T: Ord> Ord for BigNatural<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::NonZero(a), Self::NonZero(b)) => Self::ord_impl(&a, &b),
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
            (Self::NonZero(mut a), Self::NonZero(mut b)) => {
                if a.len() > b.len() {
                    swap(&mut a, &mut b);
                }
                Self::NonZero(Rc::new(Self::mul_impl(&a, &b)))
            }
            _ => Self::Zero,
        }
    }
}
impl<T: Integer> BigNatural<T> {
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
    fn div_mod_impl<const M: char>(a: &Rc<Vec<T>>, b: &Rc<Vec<T>>) -> (Self, Self) {
        if a.len() < b.len() {
            (
                Self::Zero,
                match M {
                    'd' => Self::Zero,
                    _ => Self::NonZero(Rc::clone(a)),
                },
            )
        } else {
            match M {
                'm' => {
                    let mut m = a.to_vec();
                    Self::div_impl(&mut m, b);
                    (Self::Zero, Self::wrap_empty(m))
                }
                'd' => (
                    Self::wrap_empty(Self::div_impl(&mut a.to_vec(), b)),
                    Self::Zero,
                ),
                _ => {
                    let mut m = a.to_vec();
                    let r = Self::div_impl(&mut m, b);
                    (Self::wrap_empty(r), Self::wrap_empty(m))
                }
            }
        }
    }
    fn div_pack<const M: char>(&self, rhs: &Self) -> (Self, Self) {
        match (self, rhs) {
            (Self::NonZero(a), Self::NonZero(b)) => Self::div_mod_impl::<M>(a, b),
            (_, Self::Zero) => panic!("Div 0"),
            (Self::Zero, _) => (Self::Zero, Self::Zero),
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
            Self::NonZero(r_ori) => Self::wrap_empty(
                Self::shr_in_size_iter(&r_ori[(rhs / T::BITS)..], rhs % T::BITS).collect(),
            ),
            n => n,
        }
    }
}
impl<T: Integer> Shl<usize> for BigNatural<T> {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        match self {
            Self::NonZero(r_ori) => Self::NonZero(Rc::new(
                repeat_n(T::ZERO, rhs / T::BITS)
                    .chain(Self::shl_in_size_iter(&r_ori, rhs % T::BITS))
                    .collect(),
            )),
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
impl<T: Integer> BigNatural<T> {
    fn fmt_impl(n: &Rc<Vec<T>>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dis_list = Vec::with_capacity(size_of::<T>() * n.len() * 3 / 10 + 1);
        let ten = Self::from(10_u8);
        let mut me = Self::NonZero(Rc::clone(n));
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
        write!(f, "{}", String::from_iter(dis_list.into_iter().rev()))?;
        Ok(())
    }
}
impl<T: Integer> Display for BigNatural<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonZero(n) => {
                Self::fmt_impl(n, f)?;
            }
            Self::Zero => {
                write!(f, "0")?;
            }
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
/*
pub enum BigInteger<T> {
    NonZero(bool, Rc<Vec<T>>),
    Zero,
}
impl<T> BigInteger<T> {
    pub fn new() -> Self {
        BigInteger::Zero
    }
}
impl<T> Clone for BigInteger<T> {
    fn clone(&self) -> Self {
        match self {
            Self::NonZero(s, r) => Self::NonZero(*s, Rc::clone(r)),
            Self::Zero => Self::Zero,
        }
    }
}
macro_rules! impl_big_integer_from {
    (u, $t:ty) => {
        impl<T: Integer> From<$t> for BigInteger<T> {
            fn from(value: $t) -> Self {
                match <BigNatural<T>>::from(value) {
                    BigNatural::NonZero(value) => Self::NonZero(false, value),
                    BigNatural::Zero => Self::Zero,
                }
            }
        }
    };
    (i, $t:ty) => {
        impl<T: Integer> From<$t> for BigInteger<T> {
            fn from(mut value: $t) -> Self {
                let sign = value < 0;
                value &= <$t>::NOTSIGN_COVER;
                match <BigNatural<T>>::from(value) {
                    BigNatural::NonZero(value) => Self::NonZero(sign, value),
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
        match (self, other) {
            (Self::NonZero(sign_a, a), Self::NonZero(sign_b, b)) => match (*sign_a, *sign_b) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                (true, _) => BigNatural::ord_impl(&b, &a),
                (false, _) => BigNatural::ord_impl(&a, &b),
            },
            (Self::Zero, Self::NonZero(sign, _)) => {
                if *sign {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Self::NonZero(sign, _), Self::Zero) => {
                if *sign {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            _ => Ordering::Equal,
        }
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
        if let Self::NonZero(sign, n) = self {
            if *sign {
                write!(f, "-")?;
            }
            BigNatural::fmt_impl(n, f)?;
        } else {
            write!(f, "0")?;
        }
        Ok(())
    }
}
impl<T: Integer> Add for BigInteger<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::NonZero(sign_a, mut a), Self::NonZero(sign_b, mut b)) => {
                if sign_a ^ sign_b {
                    let overflow = a < b;
                    if overflow {
                        swap(&mut a, &mut b);
                    }
                    match BigNatural::NonZero(a) - BigNatural::NonZero(b) {
                        BigNatural::NonZero(r) => Self::NonZero(overflow ^ sign_a, r),
                        BigNatural::Zero => Self::Zero,
                    }
                } else {
                    match BigNatural::NonZero(a) + BigNatural::NonZero(b) {
                        BigNatural::NonZero(r) => Self::NonZero(sign_a, r),
                        BigNatural::Zero => panic!("Add wrong"),
                    }
                }
            }
            (Self::Zero, n) => n,
            (n, Self::Zero) => n,
        }
    }
}
impl<T> Neg for BigInteger<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::NonZero(sign, r) => Self::NonZero(!sign, r),
            n => n,
        }
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
        match (self, rhs) {
            (Self::NonZero(sign_a, a), Self::NonZero(sign_b, b)) => {
                match BigNatural::NonZero(a) * BigNatural::NonZero(b) {
                    BigNatural::NonZero(r) => Self::NonZero(sign_a ^ sign_b, r),
                    BigNatural::Zero => panic!("Mul wrong"),
                }
            }
            _ => Self::Zero,
        }
    }
}
impl<T: Integer> BigInteger<T> {
    fn div_pack<const M: char>(&self, rhs: &Self) -> (BigInteger<T>, BigInteger<T>) {
        match (self, rhs) {
            (Self::NonZero(sign_a, a), Self::NonZero(sign_b, b)) => {
                let (dived, moded) = BigNatural::div_mod_impl::<M>(a, b);
                (
                    if M == 'm' {
                        Self::Zero
                    } else {
                        match dived {
                            BigNatural::NonZero(r) => Self::NonZero(!(*sign_a ^ *sign_b), r),
                            BigNatural::Zero => Self::Zero,
                        }
                    },
                    if M == 'd' {
                        Self::Zero
                    } else {
                        match moded {
                            BigNatural::NonZero(r) => Self::NonZero(*sign_a, r),
                            BigNatural::Zero => Self::Zero,
                        }
                    },
                )
            }
            (_, Self::Zero) => panic!("Div 0"),
            (Self::Zero, _) => (Self::Zero, Self::Zero),
        }
    }
}
impl<T: Integer> Div for BigInteger<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::div_pack::<'d'>(&self, &rhs).0
    }
}
impl<T: Integer> Rem for BigInteger<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::div_pack::<'d'>(&self, &rhs).1
    }
}
*/
#[cfg(test)]
mod lib_tests;
