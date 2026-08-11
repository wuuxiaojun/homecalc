use crate::domain::scenario::Scenario;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScenarioComparison {
    // 1. Timeline
    pub baseline_payoff_month: u32,
    pub alternative_payoff_month: u32,
    pub months_saved: i32,

    // 2. Outflows
    pub baseline_monthly_payment: f64,
    pub alternative_monthly_payment: f64,
    pub delta_monthly_payment: f64,
    pub baseline_extra_payment: f64,
    pub alternative_extra_payment: f64,
    pub delta_extra_payment: f64,
    pub baseline_interest_paid: f64,
    pub alternative_interest_paid: f64,
    pub delta_interest_paid: f64,

    // 3. Inflows
    pub baseline_cash_interest: f64,
    pub alternative_cash_interest: f64,
    pub delta_cash_interest: f64,
    pub baseline_tax_savings: f64,
    pub alternative_tax_savings: f64,
    pub delta_tax_savings: f64,

    // 4. Aggregate
    pub baseline_gross_paid: f64,
    pub alternative_gross_paid: f64,
    pub delta_gross_paid: f64,

    // 5. Internal Rate of Return
    pub irr: f64,
    pub pv: f64,
}

pub fn extract_monthly_outflow(scenario: &Scenario, month_idx: usize) -> f64 {
    let monthly_row = match scenario.monthly_statement.get(month_idx) {
        Some(row) => row,
        None => return 0.0,
    };

    let total_paid = monthly_row.total_paid;
    let current_month = (month_idx + 1) as u32;

    let annual_tax_savings = if current_month % 12 == 0 {
        let year_idx = ((current_month / 12) - 1) as usize;
        scenario
            .yearly_statement
            .get(year_idx)
            .map_or(0.0, |y| y.annual_tax_savings)
    } else {
        0.0
    };
    (total_paid - annual_tax_savings).max(0.0)
}

pub fn calculate_strategy_irr(baseline: &Scenario, alternative: &Scenario) -> Option<f64> {
    let last_month_a = baseline.monthly_statement.last().map_or(0, |r| r.month) as usize;
    let last_month_b = alternative.monthly_statement.last().map_or(0, |r| r.month) as usize;

    let max_months = last_month_a.max(last_month_b);
    if max_months == 0 {
        return None;
    }

    let mut delta_cash_flows = Vec::with_capacity(max_months);

    for month_idx in 0..max_months {
        let outflow_a = extract_monthly_outflow(baseline, month_idx);
        let outflow_b = extract_monthly_outflow(alternative, month_idx);

        let delta = outflow_b - outflow_a;
        delta_cash_flows.push(delta);
    }

    solve_irr_newton_raphson(&delta_cash_flows)
}

/// Newton-Raphson solver for finding the monthly root and converting to annualized IRR.
fn solve_irr_newton_raphson(cash_flows: &[f64]) -> Option<f64> {
    let mut rate: f64 = 0.005; // Initial guess: 0.5% monthly (~6.0% annual)
    let max_iterations = 100;
    let tolerance = 1e-7;

    for _ in 0..max_iterations {
        let mut npv = 0.0;
        let mut derivative = 0.0;

        for (idx, &flow) in cash_flows.iter().enumerate() {
            let m = (idx + 1) as f64;
            let factor = (1.0 + rate).powf(m);

            npv += flow / factor;
            derivative -= m * flow / (factor * (1.0 + rate));
        }

        if npv.abs() < tolerance {
            // Convert monthly compounding rate to effective annualized IRR
            let annual_irr = (1.0 + rate).powi(12) - 1.0;
            return Some(annual_irr);
        }

        if derivative.abs() < 1e-10 {
            return None; // Avoid division by zero
        }

        rate -= npv / derivative;
    }

    None // Failed to converge
}

/// Calculates the Present Value (PV) of a Scenario's net cash outflows
/// discounted at a given annual discount rate (e.g., 0.066 for 6.6%).
pub fn calculate_scenario_pv(scenario: &Scenario, discount_rate_annual: f64) -> f64 {
    let monthly_r = discount_rate_annual / 12.0;

    // Number of active months in the schedule
    let total_months = scenario.monthly_statement.len();

    let mut total_pv = 0.0;

    for month_idx in 0..total_months {
        // 1-based month index for discounting exponent
        let m = (month_idx + 1) as i32;

        // Extract net outflow using the same logic as irr.rs (total_paid - cash_yield - annual_tax_shield)
        let net_outflow = extract_monthly_outflow(scenario, month_idx);

        // Discount formula: Outflow / (1 + r)^m
        let discounted_outflow = net_outflow / (1.0 + monthly_r).powi(m);

        total_pv += discounted_outflow;
    }

    total_pv
}
