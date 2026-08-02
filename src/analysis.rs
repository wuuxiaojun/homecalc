// src/analysis.rs

use crate::mortgage::Mortgage;

#[derive(Debug, Clone, PartialEq)]
pub struct MortgageMetrics {
    pub home_price: f64,
    pub down_payment: f64,
    pub loan_amount: f64,
    pub interest_rate: f64,
    pub term_years: u32,
    pub actual_payoff_months: u32,
    pub monthly_p_i: f64,
    pub monthly_escrow: f64,
    pub monthly_piti: f64,
    pub upfront_prepaids: f64,
    pub total_cash_to_close: f64, // down_payment + upfront_prepaids
    pub total_interest_paid: f64,
    pub total_lifetime_outflow: f64,
    pub equity_at_5_years: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonReport {
    pub option_a: MortgageMetrics,
    pub option_b: MortgageMetrics,

    // Deltas (Option B minus Option A)
    pub delta_cash_to_close: f64,
    pub delta_monthly_piti: f64,
    pub delta_payoff_months: i32,
    pub delta_total_interest: f64,
    pub delta_lifetime_outflow: f64,
    pub delta_5yr_equity: f64,
}

/// Helper function to extract financial and schedule metrics from a Mortgage instance
pub fn extract_metrics(mortgage: &Mortgage) -> MortgageMetrics {
    let home_price = mortgage.price;
    let down_payment = mortgage.down;
    let loan_amount = mortgage.loan;
    let interest_rate = mortgage.rate;
    let term_years = mortgage.term;
    let actual_payoff_months = mortgage.schedule.len() as u32;

    let monthly_p_i = mortgage.base_payment;
    let monthly_escrow = mortgage.monthly_escrow();
    let monthly_piti = monthly_p_i + monthly_escrow;

    let (_, _, upfront_prepaids) = mortgage.closing_prepaids();
    let total_cash_to_close = down_payment + upfront_prepaids;

    let total_interest_paid: f64 = mortgage.schedule.values().map(|p| p.interest).sum();
    let total_lifetime_outflow: f64 = mortgage.schedule.values().map(|p| p.total_outflow).sum();

    // 5-Year Equity (Month 60). Remaining balance at month 60, or 0.0 if paid off before month 60.
    let balance_at_60 = if actual_payoff_months <= 60 {
        0.0
    } else {
        mortgage.schedule.get(&60).map(|p| p.balance).unwrap_or(0.0)
    };
    let equity_at_5_years = home_price - balance_at_60;

    MortgageMetrics {
        home_price,
        down_payment,
        loan_amount,
        interest_rate,
        term_years,
        actual_payoff_months,
        monthly_p_i,
        monthly_escrow,
        monthly_piti,
        upfront_prepaids,
        total_cash_to_close,
        total_interest_paid,
        total_lifetime_outflow,
        equity_at_5_years,
    }
}

/// Compares any two mortgage instances side-by-side (Option A vs. Option B)
pub fn compare_mortgages(option_a: &Mortgage, option_b: &Mortgage) -> ComparisonReport {
    let a = extract_metrics(option_a);
    let b = extract_metrics(option_b);

    let delta_cash_to_close = b.total_cash_to_close - a.total_cash_to_close;
    let delta_monthly_piti = b.monthly_piti - a.monthly_piti;
    let delta_payoff_months = b.actual_payoff_months as i32 - a.actual_payoff_months as i32;
    let delta_total_interest = b.total_interest_paid - a.total_interest_paid;
    let delta_lifetime_outflow = b.total_lifetime_outflow - a.total_lifetime_outflow;
    let delta_5yr_equity = b.equity_at_5_years - a.equity_at_5_years;

    ComparisonReport {
        option_a: a,
        option_b: b,
        delta_cash_to_close,
        delta_monthly_piti,
        delta_payoff_months,
        delta_total_interest,
        delta_lifetime_outflow,
        delta_5yr_equity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_different_mortgage_products() {
        // Option A: 15-Year Fixed @ 5.5% with $300k down on $1.5M home
        let option_a = Mortgage::new(1_500_000.0, 300_000.0, 5.5, 15, 1.2, 3_600.0).unwrap();

        // Option B: 30-Year Fixed @ 6.25% with $150k down on $1.5M home
        let option_b = Mortgage::new(1_500_000.0, 150_000.0, 6.25, 30, 1.2, 3_600.0).unwrap();

        let report = compare_mortgages(&option_a, &option_b);

        // Option A down payment = $300k, Option B down payment = $150k
        // Option B requires less cash to close, so delta_cash_to_close < 0
        assert!(report.delta_cash_to_close < 0.0);

        // 15-Year monthly P&I is higher than 30-Year monthly P&I -> delta_monthly_piti < 0
        assert!(report.delta_monthly_piti < 0.0);

        // 30-Year loan takes 360 months vs 180 months -> delta_payoff_months = 180
        assert_eq!(report.delta_payoff_months, 180);

        // 30-Year loan accrues significantly more interest over time
        assert!(report.delta_total_interest > 0.0);
        assert!(report.delta_lifetime_outflow > 0.0);

        // 15-Year loan builds equity much faster in 5 years than 30-Year loan
        assert!(report.delta_5yr_equity < 0.0);
    }
}
