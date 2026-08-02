// src/analysis.rs

use crate::mortgage::Mortgage;

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonReport {
    pub baseline_months: u32,
    pub accelerated_months: u32,
    pub months_saved: u32,
    pub years_saved: f64,
    pub baseline_interest: f64,
    pub accelerated_interest: f64,
    pub interest_saved: f64,
    pub baseline_outflow: f64,
    pub accelerated_outflow: f64,
    pub total_outflow_saved: f64,
}

/// Compares two mortgage instances side-by-side (e.g., standard baseline vs. accelerated loan)
pub fn compare_mortgages(baseline: &Mortgage, accelerated: &Mortgage) -> ComparisonReport {
    let baseline_months = baseline.schedule.len() as u32;
    let accelerated_months = accelerated.schedule.len() as u32;

    let months_saved = baseline_months.saturating_sub(accelerated_months);
    let years_saved = months_saved as f64 / 12.0;

    let baseline_interest: f64 = baseline.schedule.values().map(|p| p.interest).sum();
    let accelerated_interest: f64 = accelerated.schedule.values().map(|p| p.interest).sum();
    let interest_saved = baseline_interest - accelerated_interest;

    let baseline_outflow: f64 = baseline.schedule.values().map(|p| p.total_outflow).sum();
    let accelerated_outflow: f64 = accelerated.schedule.values().map(|p| p.total_outflow).sum();
    let total_outflow_saved = baseline_outflow - accelerated_outflow;

    ComparisonReport {
        baseline_months,
        accelerated_months,
        months_saved,
        years_saved,
        baseline_interest,
        accelerated_interest,
        interest_saved,
        baseline_outflow,
        accelerated_outflow,
        total_outflow_saved,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_mortgages_savings() {
        let baseline = Mortgage::new(1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();
        let mut accelerated = Mortgage::new(1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();

        let _ = accelerated.add_extra_payment(1, 50_000.0);
        let _ = accelerated.add_extra_payment(12, 100_000.0);
        let _ = accelerated.add_extra_payment(24, 200_000.0);

        let report = compare_mortgages(&baseline, &accelerated);

        assert_eq!(report.baseline_months, 180);
        assert!(report.accelerated_months < 180);
        assert!(report.months_saved > 0);
        assert!(report.years_saved > 0.0);

        assert!(report.interest_saved > 0.0);
        assert!(report.total_outflow_saved > 0.0);

        assert_eq!(report.months_saved, 180 - report.accelerated_months);
        assert!((report.interest_saved - (report.baseline_interest - report.accelerated_interest)).abs() < 1e-6);
        assert!((report.total_outflow_saved - (report.baseline_outflow - report.accelerated_outflow)).abs() < 1e-6);
    }
}
