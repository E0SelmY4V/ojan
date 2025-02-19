pub trait SignificantlyFormatable {
    fn format_significantly(&self, figures: usize) -> String;
}
macro_rules! impl_format_significantly {
    ($type:ty) => {
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
}
