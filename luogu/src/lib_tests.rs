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
#[test]
fn big_natural() {
    assert_eq!(vec![] as Vec<u8>, *BigNatural::from(0x0000000000000000).0);
    assert_eq!(vec![0x12], *BigNatural::from(0x0000000000000012).0);
    assert_eq!(
        vec![0x12, 0x00, 0x34, 0x00, 0x56],
        *BigNatural::from(0x0000005600340012_u128).0
    );
    assert_eq!(
        vec![0x32, 0x54, 0x76, 0x98, 0x78, 0x56, 0x34, 0x12],
        *BigNatural::from(0x1234567898765432_u128).0
    );
    assert_eq!(
        vec![0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe],
        *BigNatural::from(0xfedcba9876543210_u128).0
    );
    assert_eq!(
        vec![0x10, 0x32, 0x54, 0x76, 0x98, 0xba],
        *BigNatural::from(vec![0xba98_u16, 0x7654, 0x3210]).0
    );
    assert_eq!(
        vec![1, 2, 3, 4, 5],
        *BigNatural::from(vec![5_u8, 4, 3, 2, 1]).0,
    );

    assert!(BigNatural::from(0x100001) != BigNatural::from(0x00));
    assert!(BigNatural::from(0x100001) != BigNatural::from(0x101101));
    assert!(BigNatural::from(0x103400) != BigNatural::from(0x100034));
    assert!(BigNatural::from(0x63f8a9) == BigNatural::from(0x63f8a9));

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
        ($u1:literal, $u2:literal, $ops:ident) => {
            let t = tcalc!($u1, $u2);
            assert_eq!(
                t.0.clone().$ops(t.1.clone()),
                BigNatural::from(u128::$ops($u1, $u2))
            );
            assert_eq!(t, tcalc!($u1, $u2));
        };
        ($u1:expr, $u2:expr) => {
            (BigNatural::from($u1 as u128), BigNatural::from($u2 as u128))
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
}
