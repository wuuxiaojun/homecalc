use serde::{Deserialize, Serialize};
use crate::domain::loc::LocEngine;

/// Metrics extracted from an LOC Engine scenario for comparison analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocMetrics {
    pub initial_draw: f64,
    pub annual_rate: f64,
    pub annual_tax_and_insurance: f64,
    pub total_lump_sum_paid: f64,
    pub lump_sum_event_count: usize,
    pub total_interest_paid: f64,
    pub total_tax_and_insurance_paid: f64,
    pub total_lifetime_outflow: f64,
    pub balance_at_year_5: f64,
    pub equity_at_year_5: f64,
}

/// Comparison report between two LOC scenarios (Option B minus Option A).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocComparisonReport {
    pub option_a: LocMetrics,
    pub option_b: LocMetrics,
    pub delta_lump_sum_paid: f64,
    pub delta_total_interest: f64,
    pub delta_lifetime_outflow: f64,
    pub delta_year5_balance: f64,
    pub delta_year5_equity: f64,
}

/// Extracts quantitative financial metrics from a `LocEngine`.
pub fn extract_loc_metrics(engine: &LocEngine) -> LocMetrics {
    let initial_draw = engine.initial_draw;
    let annual_rate = engine.annual_rate;
    let annual_tax_and_insurance = engine.annual_tax_and_insurance();

    let total_lump_sum_paid: f64 = engine.schedule.iter().map(|s| s.extra_principal_paid).sum();
    let lump_sum_event_count = engine
        .schedule
        .iter()
        .filter(|s| s.extra_principal_paid > 0.0)
        .count();

    let total_interest_paid: f64 = engine.schedule.iter().map(|s| s.interest_billed).sum();
    let total_tax_and_insurance_paid: f64 =
        engine.schedule.iter().map(|s| s.tax_and_insurance).sum();
    let total_lifetime_outflow: f64 = engine.schedule.iter().map(|s| s.total_outflow).sum();

    // Month 60 (Year 5) balance calculation
    let balance_at_year_5 = if engine.schedule.is_empty() {
        initial_draw
    } else if engine.schedule.len() >= 60 {
        engine.schedule[59].end_balance
    } else {
        engine.schedule.last().map(|s| s.end_balance).unwrap_or(0.0)
    };

    let equity_at_year_5 = (initial_draw - balance_at_year_5).max(0.0);

    LocMetrics {
        initial_draw,
        annual_rate,
        annual_tax_and_insurance,
        total_lump_sum_paid,
        lump_sum_event_count,
        total_interest_paid,
        total_tax_and_insurance_paid,
        total_lifetime_outflow,
        balance_at_year_5,
        equity_at_year_5,
    }
}

/// Compares two LOC scenario engines and computes delta metrics (Option B - Option A).
pub fn compare_loc_scenarios(option_a: &LocEngine, option_b: &LocEngine) -> LocComparisonReport {
    let a_metrics = extract_loc_metrics(option_a);
    let b_metrics = extract_loc_metrics(option_b);

    let delta_lump_sum_paid = b_metrics.total_lump_sum_paid - a_metrics.total_lump_sum_paid;
    let delta_total_interest = b_metrics.total_interest_paid - a_metrics.total_interest_paid;
    let delta_lifetime_outflow = b_metrics.total_lifetime_outflow - a_metrics.total_lifetime_outflow;
    let delta_year5_balance = b_metrics.balance_at_year_5 - a_metrics.balance_at_year_5;
    let delta_year5_equity = b_metrics.equity_at_year_5 - a_metrics.equity_at_year_5;

    LocComparisonReport {
        option_a: a_metrics,
        option_b: b_metrics,
        delta_lump_sum_paid,
        delta_total_interest,
        delta_lifetime_outflow,
        delta_year5_balance,
        delta_year5_equity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_compare_identical_scenarios() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let engine_a = LocEngine::new("Identical A", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();
        let engine_b = LocEngine::new("Identical B", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();

        let report = compare_loc_scenarios(&engine_a, &engine_b);

        assert_eq!(report.delta_lump_sum_paid, 0.0);
        assert_eq!(report.delta_total_interest, 0.0);
        assert_eq!(report.delta_lifetime_outflow, 0.0);
        assert_eq!(report.delta_year5_balance, 0.0);
        assert_eq!(report.delta_year5_equity, 0.0);
    }

    #[test]
    fn test_early_payoff_comparison() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        // Option A: Interest-only baseline (360 months)
        let engine_a = LocEngine::new("Interest Only", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();

        // Option B: Paid off in Month 12 ($125,000 extra principal per month)
        let mut engine_b = LocEngine::new("Payoff Month 12", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();
        engine_b.set_recurring_extra_payment(125_000.0);

        assert_eq!(engine_b.schedule.len(), 12);
        assert_eq!(engine_b.schedule.last().unwrap().end_balance, 0.0);

        let report = compare_loc_scenarios(&engine_a, &engine_b);

        // Verify Year 5 balance metrics handle short schedules gracefully without out-of-bounds indexing
        assert_eq!(report.option_b.balance_at_year_5, 0.0);
        assert_eq!(report.option_b.equity_at_year_5, 1_500_000.0);

        assert_eq!(report.option_a.balance_at_year_5, 1_500_000.0);
        assert_eq!(report.option_a.equity_at_year_5, 0.0);

        assert_eq!(report.delta_year5_balance, -1_500_000.0);
        assert_eq!(report.delta_year5_equity, 1_500_000.0);
        assert!(report.delta_total_interest < 0.0);
    }

    #[test]
    fn test_extract_loc_metrics() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let engine = LocEngine::new("Test LOC", start_date, 1_000_000.0, 5.0, 1.0, 2400.0).unwrap();
        let metrics = extract_loc_metrics(&engine);

        assert_eq!(metrics.initial_draw, 1_000_000.0);
        assert_eq!(metrics.annual_rate, 5.0);
        assert_eq!(metrics.annual_tax_and_insurance, 10_000.0 + 2400.0);
        assert_eq!(metrics.total_lump_sum_paid, 0.0);
        assert_eq!(metrics.lump_sum_event_count, 0);
        assert_eq!(metrics.balance_at_year_5, 1_000_000.0);
        assert_eq!(metrics.equity_at_year_5, 0.0);
    }

    #[test]
    fn test_compare_loc_scenarios_delta_math() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let engine_a = LocEngine::new("Baseline", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();

        let mut engine_b = LocEngine::new("Accelerated", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();
        engine_b.add_extra_payment(1, 100_000.0);

        let report = compare_loc_scenarios(&engine_a, &engine_b);

        assert_eq!(report.delta_lump_sum_paid, 100_000.0);
        assert!(report.delta_total_interest < 0.0, "Interest delta should be negative for extra payment");
        assert_eq!(report.delta_year5_balance, -100_000.0);
        assert_eq!(report.delta_year5_equity, 100_000.0);
    }

    #[test]
    fn test_print_comparison_report_runs_cleanly() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let engine_a = LocEngine::new("Option A Baseline", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();
        let mut engine_b = LocEngine::new("Option B Lump Sum", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();
        engine_b.add_extra_payment(6, 200_000.0);

        let report = compare_loc_scenarios(&engine_a, &engine_b);
        crate::ui::display::print_loc_comparison_report(&report, &engine_a.name, &engine_b.name);
    }
}
