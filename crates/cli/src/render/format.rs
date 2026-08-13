//! format.rs
//! Formatting helper functions for currency, percentages, and months/years.

/// Formats a floating point value as currency (e.g., "$1,234,567.89" or "-$1,234,567.89").
pub fn format_currency(val: f64) -> String {
    if val.is_nan() {
        return "N/A".to_string();
    }

    let is_negative = val < 0.0;
    let abs_val = val.abs();

    let total_cents = (abs_val * 100.0).round() as u64;
    let dollars = total_cents / 100;
    let cents = total_cents % 100;

    let dollar_str = dollars.to_string();
    let mut formatted_dollars = String::new();
    let len = dollar_str.len();

    for (i, ch) in dollar_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted_dollars.push(',');
        }
        formatted_dollars.push(ch);
    }

    if is_negative {
        format!("-${}.{:02}", formatted_dollars, cents)
    } else {
        format!("${}.{:02}", formatted_dollars, cents)
    }
}

/// Formats a percentage value into a percent string (e.g., `6.95` -> "6.95%").
pub fn format_percent(val: f64) -> String {
    format!("{:.2}%", val)
}

/// Formats a month count into a human-readable string (e.g., 216 -> "18 Yrs, 0 Mos").
pub fn format_months(months: u32) -> String {
    let yrs = months / 12;
    let mos = months % 12;
    format!("{} Yrs, {} Mos", yrs, mos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(1234567.89), "$1,234,567.89");
        assert_eq!(format_currency(-1234567.89), "-$1,234,567.89");
        assert_eq!(format_currency(0.0), "$0.00");
    }

    #[test]
    fn test_format_percent() {
        assert_eq!(format_percent(6.95), "6.95%");
        assert_eq!(format_percent(115.84), "115.84%");
    }

    #[test]
    fn test_format_months() {
        assert_eq!(format_months(216), "18 Yrs, 0 Mos");
        assert_eq!(format_months(75), "6 Yrs, 3 Mos");
        assert_eq!(format_months(360), "30 Yrs, 0 Mos");
    }
}
