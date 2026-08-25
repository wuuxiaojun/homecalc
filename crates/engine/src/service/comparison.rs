//! comparison.rs
//! Scenario comparison and differential analytics

use crate::config::constant::DEFAULT_DISCOUNT_RATE;
use crate::domain::house::House;
use crate::domain::scenario::Scenario;
use crate::service::utility::clamp_zero;
use serde::{Deserialize, Serialize};

/// Scenario comparison metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub irr: Option<f64>,
}

/// Checks if all property attributes match between two scenarios for valid IRR strategy evaluation
pub fn house_matches(h1: &House, h2: &House) -> bool {
    (h1.purchase_price - h2.purchase_price).abs() < 1e-4
        && (h1.annual_property_tax_rate - h2.annual_property_tax_rate).abs() < 1e-4
        && (h1.annual_insurance - h2.annual_insurance).abs() < 1e-4
        && (h1.monthly_hoa - h2.monthly_hoa).abs() < 1e-4
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
    let delta_extra_payment = clamp_zero(alternative_extra_payment - baseline_extra_payment);

    let baseline_interest_paid = baseline.total_statement.total_interest_paid;
    let alternative_interest_paid = alternative.total_statement.total_interest_paid;
    let delta_interest_paid = clamp_zero(alternative_interest_paid - baseline_interest_paid);

    // 3. Inflows
    let baseline_cash_interest = baseline.total_statement.total_cash_interest;
    let alternative_cash_interest = alternative.total_statement.total_cash_interest;
    let delta_cash_interest = clamp_zero(alternative_cash_interest - baseline_cash_interest);

    let baseline_tax_savings = baseline.total_statement.total_tax_savings;
    let alternative_tax_savings = alternative.total_statement.total_tax_savings;
    let delta_tax_savings = clamp_zero(alternative_tax_savings - baseline_tax_savings);

    // 4. Aggregate
    let baseline_gross_paid = baseline.total_statement.total_paid;
    let alternative_gross_paid = alternative.total_statement.total_paid;
    let delta_gross_paid = clamp_zero(alternative_gross_paid - baseline_gross_paid);

    // 5. Analytics
    let baseline_pv = calculate_scenario_pv(baseline);
    let alternative_pv = calculate_scenario_pv(alternative);
    let delta_pv = clamp_zero(alternative_pv - baseline_pv);
    let irr = calculate_strategy_irr(baseline, alternative);

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

/// Calculate the present value (pv) of scenario cash outflows
pub fn calculate_scenario_pv(scenario: &Scenario) -> f64 {
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

    clamp_zero(total_pv)
}

/// Computes net debt service outflow (debt payment + extra principal - tax deductions) for a scenario at a given month
fn get_net_debt_outflow(scenario: &Scenario, month_idx: usize) -> f64 {
    let row = match scenario.monthly_statement.get(month_idx) {
        Some(r) => r,
        None => return 0.0,
    };

    if row.month == 0 {
        return row.total_paid; // Initial capital / down payment at month 0
    }

    let debt_service = row.total_debt_paid + row.total_extra_payment;
    let current_month = row.month;
    let annual_tax_savings = if current_month > 0
        && (current_month % 12 == 0 || month_idx == scenario.monthly_statement.len() - 1)
    {
        let year_idx = ((current_month - 1) / 12) as usize;
        scenario
            .yearly_statement
            .get(year_idx)
            .map_or(0.0, |y| y.annual_tax_savings)
    } else {
        0.0
    };

    (debt_service - annual_tax_savings).max(0.0)
}

/// Calculates the internal rate of return (IRR) on the incremental investment of Alternative (B) over Baseline (A)
pub fn calculate_strategy_irr(baseline: &Scenario, alternative: &Scenario) -> Option<f64> {
    // 1. House Parity Guard: If property parameters differ, IRR is non-comparable
    if !house_matches(&baseline.purchase.house, &alternative.purchase.house) {
        return None;
    }

    let len_a = baseline.monthly_statement.len();
    let len_b = alternative.monthly_statement.len();
    let max_len = len_a.max(len_b);
    if max_len <= 1 {
        return None;
    }

    let mut delta_cash_flows = Vec::with_capacity(max_len);

    for month_idx in 0..max_len {
        let net_a = get_net_debt_outflow(baseline, month_idx);
        let net_b = get_net_debt_outflow(alternative, month_idx);

        // Incremental cash flow: delta_C_t = -(C_{B,t} - C_{A,t}) = C_{A,t} - C_{B,t}
        let delta = net_a - net_b;
        delta_cash_flows.push(delta);
    }

    // 2. Terminal equity difference at final comparison horizon T
    let last_idx = max_len - 1;
    let ending_debt_a = baseline
        .monthly_statement
        .get(last_idx)
        .map_or(0.0, |r| r.total_remaining_balance);
    let ending_debt_b = alternative
        .monthly_statement
        .get(last_idx)
        .map_or(0.0, |r| r.total_remaining_balance);

    // Terminal net equity difference: (NetEquity_B - NetEquity_A) = (Debt_A - Debt_B)
    let terminal_equity_diff = ending_debt_a - ending_debt_b;
    delta_cash_flows[last_idx] += terminal_equity_diff;

    solve_irr_hybrid(&delta_cash_flows)
}

/// Robust bounded Hybrid Newton-Raphson / Bisection solver over monthly rate bracket [-0.99, 1.0]
pub fn solve_irr_hybrid(cash_flows: &[f64]) -> Option<f64> {
    // Check if there is at least one positive and one negative cash flow (sign change)
    let has_positive = cash_flows.iter().any(|&f| f > 1e-4);
    let has_negative = cash_flows.iter().any(|&f| f < -1e-4);
    if !has_positive || !has_negative {
        return None;
    }

    let min_rate = -0.99;
    let max_rate = 1.00;
    let tolerance = 1e-7;

    // Evaluates Net Present Value at a monthly rate r
    let npv_at = |r: f64| -> f64 {
        let base = 1.0 + r;
        if base <= 0.0 {
            return f64::NAN;
        }
        cash_flows.iter().enumerate().fold(0.0, |acc, (m, &flow)| {
            if m == 0 {
                acc + flow
            } else {
                acc + flow / base.powi(m as i32)
            }
        })
    };

    // Evaluates derivative d(NPV)/dr at monthly rate r
    let dnpv_at = |r: f64| -> f64 {
        let base = 1.0 + r;
        if base <= 0.0 {
            return f64::NAN;
        }
        cash_flows.iter().enumerate().fold(0.0, |acc, (m, &flow)| {
            if m == 0 {
                acc
            } else {
                let factor = base.powi(m as i32);
                acc - (m as f64) * flow / (factor * base)
            }
        })
    };

    // 1. Multi-start bounded Newton-Raphson (evaluating standard positive financial returns first)
    let initial_guesses: [f64; 12] = [
        0.005, 0.01, 0.02, 0.04, 0.06, 0.08, 0.10, 0.001, 0.0, 0.20, -0.005, -0.01,
    ];

    for &initial_rate in &initial_guesses {
        let mut rate = initial_rate;
        for _ in 0..60 {
            if rate < min_rate || rate > max_rate || rate.is_nan() {
                break;
            }

            let npv = npv_at(rate);
            if npv.abs() < tolerance {
                let annual_irr = (1.0 + rate).powi(12) - 1.0;
                if annual_irr.is_finite() {
                    return Some(annual_irr);
                }
            }

            let derivative = dnpv_at(rate);
            if derivative.abs() < 1e-12 || derivative.is_nan() {
                break;
            }

            let step = npv / derivative;
            let next_rate = rate - step;

            // Reject steps outside bounds
            if next_rate < min_rate || next_rate > max_rate || next_rate.is_nan() {
                break;
            }

            rate = next_rate;
        }
    }

    // 2. Bisection search across positive rates first, then negative rates
    let grid_positive: [f64; 13] = [
        0.0, 0.002, 0.005, 0.01, 0.02, 0.04, 0.06, 0.08, 0.10, 0.15, 0.25, 0.50, 1.00,
    ];

    for w in grid_positive.windows(2) {
        let r_low = w[0];
        let r_high = w[1];
        let npv_low = npv_at(r_low);
        let npv_high = npv_at(r_high);

        if npv_low.is_nan() || npv_high.is_nan() {
            continue;
        }

        if npv_low * npv_high <= 0.0 {
            let mut a = r_low;
            let mut b = r_high;
            for _ in 0..80 {
                let mid: f64 = (a + b) * 0.5;
                let npv_mid = npv_at(mid);
                if npv_mid.abs() < tolerance || (b - a).abs() < 1e-9 {
                    let annual_irr = (1.0 + mid).powi(12) - 1.0;
                    if annual_irr.is_finite() {
                        return Some(annual_irr);
                    }
                }
                if npv_at(a) * npv_mid <= 0.0 {
                    b = mid;
                } else {
                    a = mid;
                }
            }
            let root: f64 = (a + b) * 0.5;
            let annual_irr = (1.0 + root).powi(12) - 1.0;
            if annual_irr.is_finite() {
                return Some(annual_irr);
            }
        }
    }

    let grid_negative: [f64; 9] = [
        0.0, -0.01, -0.02, -0.05, -0.10, -0.20, -0.40, -0.70, -0.99,
    ];

    for w in grid_negative.windows(2) {
        let r_low: f64 = w[0].min(w[1]);
        let r_high: f64 = w[0].max(w[1]);
        let npv_low = npv_at(r_low);
        let npv_high = npv_at(r_high);

        if npv_low.is_nan() || npv_high.is_nan() {
            continue;
        }

        if npv_low * npv_high <= 0.0 {
            let mut a = r_low;
            let mut b = r_high;
            for _ in 0..80 {
                let mid: f64 = (a + b) * 0.5;
                let npv_mid = npv_at(mid);
                if npv_mid.abs() < tolerance || (b - a).abs() < 1e-9 {
                    let annual_irr = (1.0 + mid).powi(12) - 1.0;
                    if annual_irr.is_finite() {
                        return Some(annual_irr);
                    }
                }
                if npv_at(a) * npv_mid <= 0.0 {
                    b = mid;
                } else {
                    a = mid;
                }
            }
            let root: f64 = (a + b) * 0.5;
            let annual_irr = (1.0 + root).powi(12) - 1.0;
            if annual_irr.is_finite() {
                return Some(annual_irr);
            }
        }
    }


    None
}


/// Backwards-compatible alias for solve_irr_hybrid
pub fn solve_irr_newton_raphson(cash_flows: &[f64]) -> Option<f64> {
    solve_irr_hybrid(cash_flows)
}

/// Extracts monthly outflow for PV and IRR calculations
pub fn extract_monthly_outflow(scenario: &Scenario, month_idx: usize) -> f64 {
    let monthly_row = match scenario.monthly_statement.get(month_idx) {
        Some(row) => row,
        None => return 0.0,
    };

    let total_paid = monthly_row.total_paid;
    let current_month = monthly_row.month;

    // Tax savings applied on annual boundary (month % 12 == 0) or on the final payoff month
    let annual_tax_savings = if current_month > 0
        && (current_month % 12 == 0 || month_idx == scenario.monthly_statement.len() - 1)
    {
        let year_idx = ((current_month - 1) / 12) as usize;
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
    use crate::domain::tool::{Cash, Loc, Mortgage, Tool};
    use crate::service::simulation::create_scenario;
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

        let num_years = months.div_ceil(12);
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
    fn test_house_matches_parity_guard() {
        let h1 = House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.25,
            annual_insurance: 2_400.0,
            monthly_hoa: 150.0,
        };

        // Identical house
        let h2 = h1.clone();
        assert!(house_matches(&h1, &h2));

        // Different purchase price
        let mut h_diff_price = h1.clone();
        h_diff_price.purchase_price = 1_050_000.0;
        assert!(!house_matches(&h1, &h_diff_price));

        // Different property tax rate
        let mut h_diff_tax = h1.clone();
        h_diff_tax.annual_property_tax_rate = 1.30;
        assert!(!house_matches(&h1, &h_diff_tax));

        // Different insurance
        let mut h_diff_ins = h1.clone();
        h_diff_ins.annual_insurance = 3_000.0;
        assert!(!house_matches(&h1, &h_diff_ins));

        // Different HOA
        let mut h_diff_hoa = h1.clone();
        h_diff_hoa.monthly_hoa = 200.0;
        assert!(!house_matches(&h1, &h_diff_hoa));
    }

    #[test]
    fn test_house_parity_guard_blocks_irr_on_different_homes() {
        let p1 = Purchase {
            name: "Home A".to_string(),
            house: House {
                purchase_price: 800_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 2_000.0,
                monthly_hoa: 50.0,
            },
            tools: vec![
                Tool::Cash(Cash {
                    amount: 160_000.0,
                    rate: 4.0,
                }),
                Tool::Mortgage(Mortgage {
                    amount: 640_000.0,
                    rate: 6.5,
                    term: 30,
                }),
            ],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        let p2 = Purchase {
            name: "Home B (Higher Price)".to_string(),
            house: House {
                purchase_price: 900_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 2_000.0,
                monthly_hoa: 50.0,
            },
            tools: vec![
                Tool::Cash(Cash {
                    amount: 180_000.0,
                    rate: 4.0,
                }),
                Tool::Mortgage(Mortgage {
                    amount: 720_000.0,
                    rate: 6.5,
                    term: 30,
                }),
            ],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        let s1 = create_scenario(p1);
        let s2 = create_scenario(p2);

        let comparison = compare_scenarios(&s1, &s2);
        assert_eq!(comparison.irr, None, "IRR must be None when houses differ");
    }

    #[test]
    fn test_prepayment_strategy_irr_identical_house() {
        let house = House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.25,
            annual_insurance: 2_400.0,
            monthly_hoa: 100.0,
        };

        // Scenario A: Standard 30yr Mortgage ($800k @ 6.0%)
        let purchase_a = Purchase {
            name: "Baseline 30yr".to_string(),
            house: house.clone(),
            tools: vec![
                Tool::Cash(Cash {
                    amount: 200_000.0,
                    rate: 4.0,
                }),
                Tool::Mortgage(Mortgage {
                    amount: 800_000.0,
                    rate: 6.0,
                    term: 30,
                }),
            ],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        // Scenario B: Same house, same mortgage, but with $500/mo extra prepayment
        let mut mortgage_repay = BTreeMap::new();
        for m in 1..=360 {
            mortgage_repay.insert(m, 500.0);
        }

        let purchase_b = Purchase {
            name: "Prepayment Strategy".to_string(),
            house,
            tools: vec![
                Tool::Cash(Cash {
                    amount: 200_000.0,
                    rate: 4.0,
                }),
                Tool::Mortgage(Mortgage {
                    amount: 800_000.0,
                    rate: 6.0,
                    term: 30,
                }),
            ],
            mortgage_repay,
            loc_repay: BTreeMap::new(),
        };

        let scenario_a = create_scenario(purchase_a);
        let scenario_b = create_scenario(purchase_b);

        let comparison = compare_scenarios(&scenario_a, &scenario_b);

        assert!(comparison.irr.is_some(), "Expected IRR to be Some for prepayment strategy");
        let irr = comparison.irr.unwrap();
        // Return on mortgage prepayment is directly anchored around the mortgage note rate (6.0%)
        assert!(irr > 0.04 && irr < 0.08, "Expected IRR near 6%, got {}", irr);
        assert!(comparison.months_saved > 0);
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
    fn test_extract_monthly_outflow_non_annual_payoff_boundary() {
        // 15-month scenario: tax savings should apply at Month 12 AND Month 15 (final month)
        let scenario = create_mock_scenario(15, 1000.0, 300.0);
        // Month 12: Year 1 tax savings applied
        assert_eq!(extract_monthly_outflow(&scenario, 12), 700.0);
        // Month 14: Mid-year month, no tax savings applied
        assert_eq!(extract_monthly_outflow(&scenario, 14), 1000.0);
        // Month 15: Final month of 15-month scenario, Year 2 tax savings applied
        assert_eq!(extract_monthly_outflow(&scenario, 15), 700.0);
    }

    #[test]
    fn test_solve_irr_hybrid() {
        // Known stream: -100 upfront, +110 in month 1 -> monthly rate r = 10%
        let cash_flows = vec![-100.0, 110.0];
        let result = solve_irr_hybrid(&cash_flows);
        assert!(result.is_some());
        let annual_irr = result.unwrap();
        let expected_annual = (1.10_f64).powi(12) - 1.0;
        assert!((annual_irr - expected_annual).abs() < 1e-4);

        // Non-convergent stream: all positive cash flows
        let bad_flows = vec![100.0, 100.0, 100.0];
        assert!(solve_irr_hybrid(&bad_flows).is_none());

        // Negative monthly return stream: -100 upfront, +50 in month 1 -> -50% monthly
        let neg_flows = vec![-100.0, 50.0];
        let neg_res = solve_irr_hybrid(&neg_flows);
        assert!(neg_res.is_some());
        let expected_neg_annual = (0.50_f64).powi(12) - 1.0;
        assert!((neg_res.unwrap() - expected_neg_annual).abs() < 1e-4);
    }

    #[test]
    fn test_calculate_scenario_pv() {
        let scenario = create_mock_scenario(12, 1000.0, 0.0);
        let pv = calculate_scenario_pv(&scenario);
        assert!(pv > 0.0);
        assert!(pv < 112_000.0); // Discounted total must be strictly less than nominal sum of 112,000 (100k + 12k)
    }

    #[test]
    fn test_irr_user_scenario() {
        // Baseline: 1M cash down, 700k mortgage (30 yr @ 6%)
        let purchase_a = Purchase {
            name: "Baseline Mortgage".to_string(),
            house: House {
                purchase_price: 1_700_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 3_000.0,
                monthly_hoa: 100.0,
            },
            tools: vec![
                Tool::Cash(Cash {
                    amount: 1_000_000.0,
                    rate: 4.0,
                }),
                Tool::Mortgage(Mortgage {
                    amount: 700_000.0,
                    rate: 6.0,
                    term: 30,
                }),
            ],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        // Alternative: 1.7M LOC, 5-year payoff ($28,333.33/mo extra)
        let mut loc_repay = BTreeMap::new();
        for m in 1..=60 {
            loc_repay.insert(m, 1_700_000.0 / 60.0);
        }

        let purchase_b = Purchase {
            name: "Alternative LOC 5yr".to_string(),
            house: House {
                purchase_price: 1_700_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 3_000.0,
                monthly_hoa: 100.0,
            },
            tools: vec![Tool::Loc(Loc {
                amount: 1_700_000.0,
                rate: 6.0,
            })],
            mortgage_repay: BTreeMap::new(),
            loc_repay,
        };

        let scenario_a = create_scenario(purchase_a);
        let scenario_b = create_scenario(purchase_b);

        let comparison = compare_scenarios(&scenario_a, &scenario_b);
        assert!(comparison.irr.is_some());
        assert!(comparison.months_saved > 0);
        assert!(comparison.baseline_pv > 0.0);
        assert!(comparison.alternative_pv > 0.0);
    }
}
