use chrono::NaiveDate;

/// Returns exact calendar days for a given year and month.
/// Handles leap years properly using calendar logic via `chrono`.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    if month < 1 || month > 12 {
        return 0;
    }
    let current = NaiveDate::from_ymd_opt(year, month, 1);
    let next = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };

    match (current, next) {
        (Some(c), Some(n)) => (n - c).num_days() as u32,
        _ => 0,
    }
}

/// Returns the daily simple interest rate for a given annual percentage rate (APR).
/// Formula: (annual_rate_pct / 100.0) / 365.0
pub fn daily_interest_rate(annual_rate_pct: f64) -> f64 {
    (annual_rate_pct / 100.0) / 365.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_in_month_regular_and_leap() {
        assert_eq!(days_in_month(2026, 8), 31);
        assert_eq!(days_in_month(2026, 9), 30);
        assert_eq!(days_in_month(2027, 2), 28);
        assert_eq!(days_in_month(2028, 2), 29); // leap year
        assert_eq!(days_in_month(2000, 2), 29); // leap year century rule
        assert_eq!(days_in_month(1900, 2), 28); // non-leap year century rule
        assert_eq!(days_in_month(2026, 12), 31);
        assert_eq!(days_in_month(2026, 0), 0);
        assert_eq!(days_in_month(2026, 13), 0);
    }

    #[test]
    fn test_daily_interest_rate() {
        let rate = daily_interest_rate(6.0);
        let expected = 0.06 / 365.0;
        assert!((rate - expected).abs() < 1e-12);
    }
}
