//! format.rs
//! Formatting helper functions for currency, percentages, and months/years.

/// Formats a floating point value as currency (e.g., "$1,234,567.89" or "-$1,234,567.89").
pub fn format_currency(val: f64) -> String {
    if val.is_nan() || val.is_infinite() {
        return "N/A".to_string();
    }

    let abs_val = val.abs();
    let total_cents = (abs_val * 100.0).round() as u64;
    let is_negative = val < 0.0 && total_cents > 0;
    let dollars = total_cents / 100;
    let cents = total_cents % 100;

    let mut buf = [0u8; 32];
    let mut cursor = buf.len();

    // Write cents: .XX
    cursor -= 1;
    buf[cursor] = b'0' + (cents % 10) as u8;
    cursor -= 1;
    buf[cursor] = b'0' + (cents / 10) as u8;
    cursor -= 1;
    buf[cursor] = b'.';

    // Write dollars with thousand separator commas
    let mut d = dollars;
    let mut digit_count = 0;

    if d == 0 {
        cursor -= 1;
        buf[cursor] = b'0';
    } else {
        while d > 0 {
            if digit_count > 0 && digit_count % 3 == 0 {
                cursor -= 1;
                buf[cursor] = b',';
            }
            cursor -= 1;
            buf[cursor] = b'0' + (d % 10) as u8;
            d /= 10;
            digit_count += 1;
        }
    }

    cursor -= 1;
    buf[cursor] = b'$';

    if is_negative {
        cursor -= 1;
        buf[cursor] = b'-';
    }

    let s = std::str::from_utf8(&buf[cursor..]).unwrap_or("$0.00");
    s.to_string()
}

/// Formats a percentage value into a percent string (e.g., `6.95` -> "6.95%").
pub fn format_percent(val: f64) -> String {
    if val.is_nan() || val.is_infinite() {
        return "N/A".to_string();
    }
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
        assert_eq!(format_currency(-0.0), "$0.00");
        assert_eq!(format_currency(-0.001), "$0.00");
        assert_eq!(format_currency(-0.006), "-$0.01");
        assert_eq!(format_currency(0.05), "$0.05");
        assert_eq!(format_currency(10_000_000.0), "$10,000,000.00");
        assert_eq!(format_currency(f64::NAN), "N/A");
        assert_eq!(format_currency(f64::INFINITY), "N/A");
        assert_eq!(format_currency(f64::NEG_INFINITY), "N/A");
    }

    #[test]
    fn test_format_percent() {
        assert_eq!(format_percent(6.95), "6.95%");
        assert_eq!(format_percent(115.84), "115.84%");
        assert_eq!(format_percent(0.0), "0.00%");
        assert_eq!(format_percent(f64::NAN), "N/A");
        assert_eq!(format_percent(f64::INFINITY), "N/A");
    }

    #[test]
    fn test_format_months() {
        assert_eq!(format_months(0), "0 Yrs, 0 Mos");
        assert_eq!(format_months(6), "0 Yrs, 6 Mos");
        assert_eq!(format_months(12), "1 Yrs, 0 Mos");
        assert_eq!(format_months(75), "6 Yrs, 3 Mos");
        assert_eq!(format_months(216), "18 Yrs, 0 Mos");
        assert_eq!(format_months(360), "30 Yrs, 0 Mos");
    }
}
