pub fn format_significantly_f64(value: f64, figures: usize) -> String {
    let digits = match value.abs() {
        v if v >= 1. => figures.saturating_sub(value.abs().log10() as usize + 1),
        v if v == 0. => figures - 1,
        v  => (- v.log10().floor()) as usize - 1 + figures,
    };
    format!("{:.*}", digits, value)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_significantly() {
        assert_eq!("1.000", format_significantly_f64(1., 4));
        assert_eq!("12.2300", format_significantly_f64(12.23, 6));
        assert_eq!("123.000", format_significantly_f64(123., 6));
        assert_eq!("12345", format_significantly_f64(12345., 4));
        assert_eq!("0.1235", format_significantly_f64(0.123456, 4));
        assert_eq!("0.1234", format_significantly_f64(0.123446, 4));
        assert_eq!("0.00034500", format_significantly_f64(0.000345, 5));

        assert_eq!("0.000", format_significantly_f64(0., 4));

        assert_eq!("-1.000", format_significantly_f64(-1., 4));
        assert_eq!("-12.2300", format_significantly_f64(-12.23, 6));
        assert_eq!("-123.000", format_significantly_f64(-123., 6));
        assert_eq!("-12345", format_significantly_f64(-12345., 4));
        assert_eq!("-0.1235", format_significantly_f64(-0.123456, 4));
        assert_eq!("-0.1234", format_significantly_f64(-0.123446, 4));
        assert_eq!("-0.00034500", format_significantly_f64(-0.000345, 5));
    }
}
