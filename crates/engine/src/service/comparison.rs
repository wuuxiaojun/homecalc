//! comparison.rs
//! Scenario comparison

use crate::config::constant::DEFAULT_DISCOUNT_RATE;
use crate::domain::scenario::Scenario;

/// Scenario comparison metrics
#[derive(Debug, Clone)]
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

/// Compute scenarios comparison metrics
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

/// Calculate the present value (pv)
fn calculate_scenario_pv(scenario: &Scenario) -> f64 {
    let monthly_r = DEFAULT_DISCOUNT_RATE / 12.0;
    let mut total_pv = 0.0;
    let base = 1.0 + monthly_r;

    for (month_idx, row) in scenario.monthly_statement.iter().enumerate() {
        let net_outflow = extract_monthly_outflow(scenario, month_idx);
        let m = row.month as i32;
        let discount_factor = base.powi(m);
        let discounted_outflow = net_outflow / discount_factor;
        total_pv += discounted_outflow;
    }

    total_pv
}

/// Calculates the internal rate of return (irr)
fn calculate_strategy_irr(baseline: &Scenario, alternative: &Scenario) -> Option<f64> {
    let max_len = baseline
        .monthly_statement
        .len()
        .max(alternative.monthly_statement.len());
    if max_len == 0 {
        return None;
    }

    let mut delta_cash_flows = Vec::with_capacity(max_len);

    for month_idx in 0..max_len {
        let outflow_a = extract_monthly_outflow(baseline, month_idx);
        let outflow_b = extract_monthly_outflow(alternative, month_idx);

        let delta = outflow_b - outflow_a;
        delta_cash_flows.push(delta);
    }

    solve_irr_newton_raphson(&delta_cash_flows)
}

/// Newton-Raphson solver for irr
fn solve_irr_newton_raphson(cash_flows: &[f64]) -> Option<f64> {
    let mut rate: f64 = 0.005; // Initial guess: 0.5% monthly (~6.0% annual)
    let max_iterations = 100;
    let tolerance = 1e-7;

    for _ in 0..max_iterations {
        let mut npv = 0.0;
        let mut derivative = 0.0;
        let base = 1.0 + rate;

        for (m, &flow) in cash_flows.iter().enumerate() {
            if m == 0 {
                npv += flow;
            } else {
                let factor = base.powi(m as i32);
                npv += flow / factor;
                derivative -= (m as f64) * flow / (factor * base);
            }
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

/// Extracts monthly outflow
/// Auxiliary function for irr & pv calculation
fn extract_monthly_outflow(scenario: &Scenario, month_idx: usize) -> f64 {
    let monthly_row = match scenario.monthly_statement.get(month_idx) {
        Some(row) => row,
        None => return 0.0,
    };

    let total_paid = monthly_row.total_paid;
    let current_month = monthly_row.month;

    let annual_tax_savings = if current_month > 0 && current_month % 12 == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::house::House;
    use crate::domain::purchase::Purchase;
    use crate::domain::statement::{
        HouseStatement, MonthlyStatementRow, TotalStatement, YearlyStatementRow,
    };
    use std::collections::BTreeMap;

    fn create_mock_scenario(months: u32, monthly_paid: f64, annual_tax_savings: f64) -> Scenario {
        let mut monthly_statement = Vec::new();
        monthly_statement.push(MonthlyStatementRow {
            month: 0,
            cash: None,
            mortgage: None,
            loc: None,
            house: HouseStatement {
                monthly_property_tax: 0.0,
                monthly_insurance: 0.0,
                monthly_hoa: 0.0,
            },
            total_debt_paid: 0.0,
            total_extra_payment: 0.0,
            total_holding_cost: 0.0,
            total_paid: 100_000.0,
            total_remaining_balance: 200_000.0,
        });

        for m in 1..=months {
            monthly_statement.push(MonthlyStatementRow {
                month: m,
                cash: None,
                mortgage: None,
                loc: None,
                house: HouseStatement {
                    monthly_property_tax: 200.0,
                    monthly_insurance: 50.0,
                    monthly_hoa: 10.0,
                },
                total_debt_paid: monthly_paid - 260.0,
                total_extra_payment: 0.0,
                total_holding_cost: 260.0,
                total_paid: monthly_paid,
                total_remaining_balance: if m == months { 0.0 } else { 100_000.0 },
            });
        }

        let num_years = (months + 11) / 12;
        let mut yearly_statement = Vec::new();
        for y in 1..=num_years {
            yearly_statement.push(YearlyStatementRow {
                year: y,
                annual_cash_interest: 0.0,
                annual_interest_paid: 5000.0,
                annual_debt_paid: monthly_paid * 12.0 - 3120.0,
                annual_tax_savings,
                annual_extra_payment: 0.0,
                annual_holding_cost: 3120.0,
                annual_paid: monthly_paid * 12.0 - annual_tax_savings,
                ending_remaining_balance: 0.0,
            });
        }

        Scenario {
            purchase: Purchase {
                name: "Mock Purchase".to_string(),
                house: House {
                    purchase_price: 300_000.0,
                    annual_property_tax_rate: 1.0,
                    annual_insurance: 600.0,
                    monthly_hoa: 10.0,
                },
                tools: vec![],
                mortgage_repay: BTreeMap::new(),
                loc_repay: BTreeMap::new(),
            },
            monthly_statement,
            yearly_statement,
            total_statement: TotalStatement {
                total_cash_interest: 0.0,
                total_holding_cost: 3120.0 * num_years as f64,
                total_interest_paid: 5000.0 * num_years as f64,
                total_tax_savings: annual_tax_savings * num_years as f64,
                total_paid: (monthly_paid * 12.0 - annual_tax_savings) * num_years as f64,
            },
        }
    }

    #[test]
    fn test_extract_monthly_outflow_bounds() {
        let scenario = create_mock_scenario(12, 1000.0, 0.0);
        assert_eq!(extract_monthly_outflow(&scenario, 0), 100_000.0); // Month 0
        assert_eq!(extract_monthly_outflow(&scenario, 1), 1000.0); // Month 1
        assert_eq!(extract_monthly_outflow(&scenario, 6), 1000.0); // Month 6
        assert_eq!(extract_monthly_outflow(&scenario, 12), 1000.0); // Month 12
        assert_eq!(extract_monthly_outflow(&scenario, 99), 0.0);
    }

    #[test]
    fn test_extract_monthly_outflow_annual_tax() {
        let scenario = create_mock_scenario(24, 1000.0, 200.0);
        // Month 1 (index 1) - tax savings should NOT apply
        assert_eq!(extract_monthly_outflow(&scenario, 1), 1000.0);
        // Month 12 (index 12) - tax savings (200.0) SHOULD apply
        assert_eq!(extract_monthly_outflow(&scenario, 12), 800.0);
        // Month 24 (index 24) - tax savings SHOULD apply
        assert_eq!(extract_monthly_outflow(&scenario, 24), 800.0);
    }

    #[test]
    fn test_solve_irr_newton_raphson() {
        // Known stream: -100 upfront, +110 in month 1 -> monthly rate r = 10%
        let cash_flows = vec![-100.0, 110.0];
        let result = solve_irr_newton_raphson(&cash_flows);
        assert!(result.is_some());
        let annual_irr = result.unwrap();
        let expected_annual = (1.10_f64).powi(12) - 1.0;
        assert!((annual_irr - expected_annual).abs() < 1e-4);

        // Non-convergent stream: all positive cash flows
        let bad_flows = vec![100.0, 100.0, 100.0];
        assert!(solve_irr_newton_raphson(&bad_flows).is_none());
    }

    #[test]
    fn test_calculate_scenario_pv() {
        let scenario = create_mock_scenario(12, 1000.0, 0.0);
        let pv = calculate_scenario_pv(&scenario);
        assert!(pv > 0.0);
        assert!(pv < 112_000.0); // Discounted total must be strictly less than nominal sum of 112,000 (100k + 12k)
    }
}
