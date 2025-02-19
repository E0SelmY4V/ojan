pub fn format_significantly_f64(value: f64, figures: usize) -> String {
    let digits = match value {
        v if v <= 0.0 => figures,
        _ => figures.saturating_sub(value.abs().log10() as usize + 1),
    };
    format!("{:.*}", digits, value)
}
