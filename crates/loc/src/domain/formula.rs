use chrono::NaiveDate;

/// Returns exact calendar days for a given year and month.
/// Handles leap years properly using calendar logic via `chrono`.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    if !(1..=12).contains(&month) {
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
    fn test_leap_years_vs_non_leap_years() {
        assert_eq!(days_in_month(2028, 2), 29); // 2028 is a leap year
        assert_eq!(days_in_month(2027, 2), 28); // 2027 is a non-leap year
    }

    #[test]
    fn test_century_leap_year_rules() {
        assert_eq!(days_in_month(2000, 2), 29); // 2000 is a century leap year (divisible by 400)
        assert_eq!(days_in_month(1900, 2), 28); // 1900 is NOT a century leap year (divisible by 100, not 400)
    }

    #[test]
    fn test_invalid_month_inputs() {
        assert_eq!(days_in_month(2026, 0), 0);
        assert_eq!(days_in_month(2026, 13), 0);
        assert_eq!(days_in_month(2026, 99), 0);
    }

    #[test]
    fn test_days_in_month_regular() {
        assert_eq!(days_in_month(2026, 8), 31);
        assert_eq!(days_in_month(2026, 9), 30);
        assert_eq!(days_in_month(2026, 12), 31);
    }

    #[test]
    fn test_daily_interest_rate() {
        let rate = daily_interest_rate(6.0);
        let expected = 0.06 / 365.0;
        assert!((rate - expected).abs() < 1e-12);
    }
}
