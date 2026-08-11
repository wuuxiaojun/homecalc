use crate::domain::scenario::Scenario;
use crate::service::formula::solve_irr_newton_raphson;

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
