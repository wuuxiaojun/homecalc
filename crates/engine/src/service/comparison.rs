use crate::{config::constant::DEFAULT_DISCOUNT_RATE, domain::scenario::Scenario};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScenarioComparison {
    // 1. Timeline
    pub baseline_payoff_month: u32,
    pub alternative_payoff_month: u32,
    pub months_saved: i32,

    // 2. Outflows
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

    // 5. Analytics
    pub baseline_pv: f64,
    pub alternative_pv: f64,
    pub delta_pv: f64,
    pub irr: f64,
}

pub fn compare_scenarios(baseline: &Scenario, alternative: &Scenario) -> ScenarioComparison {
    // 1. Timeline
    let baseline_payoff_month = baseline.monthly_statement.last().map_or(0, |r| r.month);
    let alternative_payoff_month = alternative.monthly_statement.last().map_or(0, |r| r.month);
    let months_saved = baseline_payoff_month as i32 - alternative_payoff_month as i32;

    // 2. Outflows
    let baseline_extra_payment: f64 = baseline
        .yearly_statement
        .iter()
        .map(|y| y.annual_extra_payment)
        .sum();
    let alternative_extra_payment: f64 = alternative
        .yearly_statement
        .iter()
        .map(|y| y.annual_extra_payment)
        .sum();
    let delta_extra_payment = alternative_extra_payment - baseline_extra_payment;

    let baseline_interest_paid = baseline.total_statement.total_interest_paid;
    let alternative_interest_paid = alternative.total_statement.total_interest_paid;
    let delta_interest_paid = alternative_interest_paid - baseline_interest_paid;

    // 3. Inflows
    let baseline_cash_interest = baseline.total_statement.total_cash_interest;
    let alternative_cash_interest = alternative.total_statement.total_cash_interest;
    let delta_cash_interest = alternative_cash_interest - baseline_cash_interest;

    let baseline_tax_savings = baseline.total_statement.total_tax_savings;
    let alternative_tax_savings = alternative.total_statement.total_tax_savings;
    let delta_tax_savings = alternative_tax_savings - baseline_tax_savings;

    // 4. Aggregate
    let baseline_gross_paid = baseline.total_statement.total_paid;
    let alternative_gross_paid = alternative.total_statement.total_paid;
    let delta_gross_paid = alternative_gross_paid - baseline_gross_paid;

    // 5. Analytics
    let baseline_pv = calculate_scenario_pv(baseline);
    let alternative_pv = calculate_scenario_pv(alternative);
    let delta_pv = alternative_pv - baseline_pv;

    let irr = calculate_strategy_irr(baseline, alternative).unwrap_or(0.0);

    ScenarioComparison {
        baseline_payoff_month,
        alternative_payoff_month,
        months_saved,
        baseline_extra_payment,
        alternative_extra_payment,
        delta_extra_payment,
        baseline_interest_paid,
        alternative_interest_paid,
        delta_interest_paid,
        baseline_cash_interest,
        alternative_cash_interest,
        delta_cash_interest,
        baseline_tax_savings,
        alternative_tax_savings,
        delta_tax_savings,
        baseline_gross_paid,
        alternative_gross_paid,
        delta_gross_paid,
        baseline_pv,
        alternative_pv,
        delta_pv,
        irr,
    }
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
pub fn calculate_scenario_pv(scenario: &Scenario) -> f64 {
    let monthly_r = DEFAULT_DISCOUNT_RATE / 12.0;

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
