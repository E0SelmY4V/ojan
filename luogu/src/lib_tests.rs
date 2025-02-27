use crate::*;

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
impl BigNatural<u8> {
    fn get_vec(&self) -> Vec<u8> {
        match self {
            Self::NonZero(n) => (*n.clone()).clone(),
            Self::Zero => vec![],
        }
    }
}
#[test]
fn big_natural() {
    assert_eq!(
        vec![] as Vec<u8>,
        BigNatural::from(0x0000000000000000).get_vec()
    );
    assert_eq!(vec![0x12], BigNatural::from(0x0000000000000012).get_vec());
    assert_eq!(
        vec![0x12, 0x00, 0x34, 0x00, 0x56],
        BigNatural::from(0x0000005600340012_u128).get_vec()
    );
    assert_eq!(
        vec![0x32, 0x54, 0x76, 0x98, 0x78, 0x56, 0x34, 0x12],
        BigNatural::from(0x1234567898765432_u128).get_vec()
    );
    assert_eq!(
        vec![0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe],
        BigNatural::from(0xfedcba9876543210_u128).get_vec()
    );
    assert_eq!(
        vec![0x10, 0x32, 0x54, 0x76, 0x98, 0xba],
        BigNatural::from(vec![0xba98_u16, 0x7654, 0x3210]).get_vec()
    );
    assert_eq!(
        vec![1, 2, 3, 4, 5],
        BigNatural::from(vec![5_u8, 4, 3, 2, 1]).get_vec(),
    );

    macro_rules! tcmp {
        ($a:literal  == $b:literal ) => {
            tcmp!($a, eq, $b);
        };
        ($a:literal  < $b:literal ) => {
            tcmp!($a, le, $b);
        };
        ($a:literal  > $b:literal ) => {
            tcmp!($a, ge, $b);
        };
        ($a:literal != $b:literal ) => {
            tcmp!($a, ne, $b);
        };
        ($a:literal, $op:ident, $b:literal) => {
            tcmp!($a, $op, $b, u8);
            tcmp!($a, $op, $b, u16);
            tcmp!($a, $op, $b, u32);
            tcmp!($a, $op, $b, u64);
            tcmp!($a, $op, $b, u128);
        };
        ($a:literal, $op:ident, $b:literal, $t:ty) => {
            assert!(&<BigNatural<$t>>::from($a).$op(&<BigNatural<$t>>::from($b)));
        };
    }
    tcmp!(0x100001 != 0x00);
    tcmp!(0x100001 != 0x101101);
    tcmp!(0x103400 != 0x100034);
    tcmp!(0x63f8a9 == 0x63f8a9);
    tcmp!(0 == 0);
    tcmp!(0xff00 > 0xff);
    tcmp!(0x1234 < 0x2234);
    tcmp!(0x1234 < 0x1235);

    macro_rules! tfmt {
        ($a:literal) => {
            tfmt!($a, u8);
            tfmt!($a, u16);
            tfmt!($a, u32);
            tfmt!($a, u64);
            tfmt!($a, u128);
        };
        ($num:literal, $t:ty) => {
            assert_eq!(
                stringify!($num),
                format!("{}", <BigNatural<$t>>::from($num as u128))
            );
        };
    }

    tfmt!(0);
    tfmt!(1);
    tfmt!(12345678);
    tfmt!(8902367478);
    tfmt!(998244353);
    tfmt!(981678389900);

    macro_rules! tcalc {
        ($u1:literal + $u2:literal) => {
            tcalc!($u1, $u2, add);
        };
        ($u1:literal - $u2:literal) => {
            tcalc!($u1, $u2, sub);
        };
        ($u1:literal * $u2:literal) => {
            tcalc!($u1, $u2, mul);
        };
        ($u1:literal >> $u2:literal) => {
            tcalc!($u1, $u2, shr);
        };
        ($u1:literal << $u2:literal) => {
            tcalc!($u1, $u2, shl);
        };
        ($u1:literal / $u2:literal) => {
            tcalc!($u1, $u2, div);
        };
        ($u1:literal % $u2:literal) => {
            tcalc!($u1, $u2, rem);
        };
        ($u1:literal, $u2:literal, $ops:ident) => {
            tcalc!($u1, $u2, $ops, u8);
            tcalc!($u1, $u2, $ops, u16);
            tcalc!($u1, $u2, $ops, u32);
            tcalc!($u1, $u2, $ops, u64);
            tcalc!($u1, $u2, $ops, u128);
        };
        ($u1:literal, $u2:literal, $ops:ident, $t:ty) => {
            let t = tcalc!($u1, $u2, $t);
            assert_eq!(
                t.0.clone().$ops(t.1.clone()),
                <BigNatural<$t>>::from(u128::$ops($u1, $u2))
            );
            assert_eq!(t, tcalc!($u1, $u2, $t));
        };
        ($u1:expr, $u2:expr, $t:ty) => {
            (
                <BigNatural<$t>>::from($u1 as u128),
                <BigNatural<$t>>::from($u2 as u128),
            )
        };
    }

    tcalc!(0x01 + 0x02);
    tcalc!(0x02 + 0x01);
    tcalc!(0xff + 0x01);
    tcalc!(0xffff + 0x01);
    tcalc!(0xff0fff + 0x01);
    tcalc!(1892363 + 8583674);
    tcalc!(993828300 + 1);

    tcalc!(0 - 0);
    tcalc!(0x10000000 - 1);
    tcalc!(0x100000000 - 1);
    tcalc!(267478328917 - 7283719283);
    tcalc!(998244353 - 998244353);
    tcalc!(0x6642ffaa - 0x564dafcc);
    tcalc!(0xf543a - 0xff);

    tcalc!(0 * 0);
    tcalc!(0 * 912638932);
    tcalc!(981637783 * 0);
    tcalc!(998244353 * 1);
    tcalc!(1 * 998244353);
    tcalc!(2 * 3);
    tcalc!(30 * 2);
    tcalc!(14 * 11);
    tcalc!(921823 * 18239912);
    tcalc!(18239912 * 921823);
    tcalc!(92738287 * 1923);

    tcalc!(0x6199afbca993 >> 0);
    tcalc!(0x2817afbdbb >> 1);
    tcalc!(0x0194abbc71baba >> 2);
    tcalc!(0x828dbbaff191ccccccc >> 3);
    tcalc!(0x91837abcd8f1 >> 19);

    tcalc!(0x6199afbca993 << 0);
    tcalc!(0x2817afbdbb << 1);
    tcalc!(0x0194abbc71baba << 2);
    tcalc!(0x828dbbaff191ccccccc << 3);
    tcalc!(0x91837abcd8f1 << 19);

    tcalc!(8764346792 / 1);
    tcalc!(764306364 / 2);
    tcalc!(7651029391012 / 10392);
    tcalc!(38478382912 / 182930402);
    tcalc!(6 / 2);
    tcalc!(1 / 1);
    tcalc!(14 / 5);
    tcalc!(1882727373 / 1992838991929);
    tcalc!(29291993716637271_u128 / 998244353_u128);

    tcalc!(818939492 % 1);
    tcalc!(29291993716637271 % 998244353);
    tcalc!(81737291981189392181 % 192929910293201);
    tcalc!(9919 % 2);
    tcalc!(9199 % 1994848299210);
    tcalc!(1 % 1);
    tcalc!(2 % 2);
    tcalc!(0xff1828 % 0x18face);

    macro_rules! tinput {
        ($u1:literal) => {
            tinput!($u1, u8);
            tinput!($u1, u16);
            tinput!($u1, u32);
            tinput!($u1, u64);
            tinput!($u1, u128);
        };
        ($l:literal, $t:ty) => {
            assert_eq!(
                stringify!($l).parse::<BigNatural<$t>>().unwrap(),
                <BigNatural<$t>>::from($l)
            );
        };
    }

    tinput!(0);
    tinput!(1);
    tinput!(192_736_1);
    tinput!(817__2_32);
    tinput!(18267391___);
    tinput!(0x0);
    tinput!(0x0_000);
    tinput!(0x000);
    tinput!(0xff8163);
    tinput!(0x198_2__f_ff);
    // todo
    // tinput!(0o0);
    // tinput!(0o0000__);
    // tinput!(0o_0_00);
    // tinput!(0o23153);
    // tinput!(0o2_5_752631);
    tinput!(0b0);
    tinput!(0b0000);
    tinput!(0b000);
    tinput!(0b0010__1_001_0101011101);
    tinput!(0b1_1001);
}
