//! simulation.rs
//! Monthly yearly and total simulation
use crate::config::constant::*;
use crate::domain::purchase::*;
use crate::domain::scenario::*;
use crate::domain::statement::*;
use crate::service::utility::*;

/// Simulate monthly statement
pub fn simulate_monthly(purchase: &Purchase) -> Vec<MonthlyStatementRow> {
    let mut statement = Vec::with_capacity(360);

    // 1. Extract Tool Configuration
    let cash_opt = purchase.cash().copied();
    let mortgage_opt = purchase.mortgage().copied();
    let loc_opt = purchase.loc().copied();

    // 2. State Tracking Variables
    let mut cash_balance = cash_opt
        .as_ref()
        .map_or(0.0, |c| DEFAULT_STARTING_CASH - c.amount);
    let mut mortgage_balance = mortgage_opt.as_ref().map_or(0.0, |m| m.amount);
    let mut loc_balance = loc_opt.as_ref().map_or(0.0, |l| l.amount);

    if mortgage_balance <= 0.0 && loc_balance <= 0.0 {
        return Vec::new();
    }

    let mut monthly_property_tax = purchase.house.monthly_property_tax();
    let mut monthly_insurance = purchase.house.monthly_insurance();
    let mut monthly_hoa = purchase.house.monthly_hoa;

    // 3. PMT
    let mortgage_pmt = mortgage_opt
        .as_ref()
        .map_or(0.0, |m| calculate_mortgage_pmt(m.amount, m.rate, m.term));

    // 4. Monthly Simulation Loop
    for month in 1..=360 {
        // Cash Statement
        let cash_statement = cash_opt.as_ref().map(|c| {
            let cash_compound = calculate_monthly_compound(cash_balance, c.rate);
            cash_balance = cash_compound.total;
            CashStatement {
                cash_now: cash_balance,
                cash_interest: cash_compound.interest,
            }
        });

        // Mortgage Statement
        let mortgage_statement = mortgage_opt.as_ref().and_then(|m| {
            if mortgage_balance <= 0.0 {
                return None;
            }

            let interest_paid = calculate_monthly_compound(mortgage_balance, m.rate)
                .interest
                .min(mortgage_pmt);
            let principal_paid = (mortgage_pmt - interest_paid).min(mortgage_balance);
            let extra_payment = purchase
                .mortgage_repay
                .get(&month)
                .copied()
                .unwrap_or(0.0)
                .min((mortgage_balance - principal_paid).max(0.0));
            let total_principal_paid = principal_paid + extra_payment;
            mortgage_balance = clamp_zero(mortgage_balance - total_principal_paid);

            Some(MortgageStatement {
                monthly_payment: mortgage_pmt,
                principal_paid,
                interest_paid,
                extra_payment,
                remaining_balance: mortgage_balance,
            })
        });

        // Loc Statement
        let loc_statement = loc_opt.as_ref().and_then(|l| {
            if loc_balance <= 0.0 {
                return None;
            }
            let monthly_payment = calculate_monthly_compound(loc_balance, l.rate).interest;
            let extra_payment = purchase
                .loc_repay
                .get(&month)
                .copied()
                .unwrap_or(0.0)
                .min(loc_balance);
            loc_balance = clamp_zero(loc_balance - extra_payment);

            Some(LocStatement {
                monthly_payment,
                extra_payment,
                remaining_balance: loc_balance,
            })
        });

        // House Statement
        if month > 1 && (month - 1) % 12 == 0 {
            monthly_property_tax *= 1.0 + DEFAULT_TAX_GROWTH_RATE;
            monthly_insurance *= 1.0 + DEFAULT_INSURANCE_GROWTH_RATE;
            monthly_hoa *= 1.0 + DEFAULT_HOA_GROWTH_RATE;
        }

        let house_statement = HouseStatement {
            monthly_property_tax,
            monthly_insurance,
            monthly_hoa,
        };

        // Aggregate
        let total_debt_paid = mortgage_statement.as_ref().map_or(0.0, |m| {
            (m.principal_paid + m.interest_paid).min(m.monthly_payment)
        }) + loc_statement.as_ref().map_or(0.0, |l| l.monthly_payment);
        let total_extra_payment = mortgage_statement.as_ref().map_or(0.0, |m| m.extra_payment)
            + loc_statement.as_ref().map_or(0.0, |l| l.extra_payment);
        let total_holding_cost = monthly_property_tax + monthly_insurance + monthly_hoa;
        let total_paid = total_debt_paid + total_extra_payment + total_holding_cost
            - cash_statement.as_ref().map_or(0.0, |c| c.cash_interest);
        let total_remaining_balance = clamp_zero(
            mortgage_statement
                .as_ref()
                .map_or(0.0, |m| m.remaining_balance)
                + loc_statement.as_ref().map_or(0.0, |l| l.remaining_balance),
        );

        statement.push(MonthlyStatementRow {
            month,
            cash: cash_statement,
            mortgage: mortgage_statement,
            loc: loc_statement,
            house: house_statement,
            total_debt_paid,
            total_extra_payment,
            total_holding_cost,
            total_paid,
            total_remaining_balance,
        });

        // End
        if total_remaining_balance <= 0.0 {
            break;
        }
    }

    statement
}

/// Aggregate monthly statement into yearly statement
pub fn aggregate_yearly(statement: &[MonthlyStatementRow]) -> Vec<YearlyStatementRow> {
    statement
        .chunks(12)
        .enumerate()
        .map(|(year_id, chunk)| {
            let year = (year_id + 1) as u32;

            let mut annual_cash_interest = 0.0;
            let mut annual_mortgage_interest = 0.0;
            let mut annual_loc_interest = 0.0;
            let mut annual_debt_paid = 0.0;
            let mut annual_extra_payment = 0.0;
            let mut annual_holding_cost = 0.0;
            let mut annual_paid_unadjusted = 0.0;

            let mut mortgage_balance_sum = 0.0;
            let mut mortgage_balance_count = 0usize;

            for row in chunk {
                if let Some(c) = &row.cash {
                    annual_cash_interest += c.cash_interest;
                }
                if let Some(m) = &row.mortgage {
                    annual_mortgage_interest += m.interest_paid;
                    mortgage_balance_sum += m.principal_paid + m.remaining_balance + m.extra_payment;
                    mortgage_balance_count += 1;
                }
                if let Some(l) = &row.loc {
                    annual_loc_interest += l.monthly_payment;
                }
                annual_debt_paid += row.total_debt_paid;
                annual_extra_payment += row.total_extra_payment;
                annual_holding_cost += row.total_holding_cost;
                annual_paid_unadjusted += row.total_paid;
            }

            // Annual Tax Savings
            let annual_tax_savings = if annual_mortgage_interest > 0.0
                && mortgage_balance_count > 0
            {
                let average_mortgage_balance: f64 =
                    mortgage_balance_sum / mortgage_balance_count as f64;
                let eligible_ratio = if average_mortgage_balance > IRS_MORTGAGE_LIMIT {
                    IRS_MORTGAGE_LIMIT / average_mortgage_balance
                } else {
                    1.0
                };
                let deductible_interest = annual_mortgage_interest * eligible_ratio;
                deductible_interest * DEFAULT_MARGINAL_TAX_RATE
            } else {
                0.0
            };

            // Last Row
            let last_row = chunk.last().expect("Chunk contains at least 1 month");

            YearlyStatementRow {
                year,
                annual_cash_interest,
                annual_interest_paid: annual_mortgage_interest + annual_loc_interest,
                annual_debt_paid,
                annual_tax_savings,
                annual_extra_payment,
                annual_holding_cost,
                annual_paid: annual_paid_unadjusted - annual_tax_savings,
                ending_remaining_balance: last_row.total_remaining_balance,
            }
        })
        .collect()
}

/// Aggregate yearly into total metrics
pub fn compute_metrics(yearly_statement: &[YearlyStatementRow]) -> TotalStatement {
    let total_cash_interest: f64 = yearly_statement
        .iter()
        .map(|r| r.annual_cash_interest)
        .sum();
    let total_holding_cost: f64 = yearly_statement.iter().map(|r| r.annual_holding_cost).sum();
    let total_interest_paid: f64 = yearly_statement
        .iter()
        .map(|r| r.annual_interest_paid)
        .sum();
    let total_tax_savings: f64 = yearly_statement.iter().map(|r| r.annual_tax_savings).sum();
    let total_paid: f64 = yearly_statement.iter().map(|r| r.annual_paid).sum();
    TotalStatement {
        total_cash_interest,
        total_holding_cost,
        total_interest_paid,
        total_tax_savings,
        total_paid,
    }
}

/// Create scenario
pub fn create_scenario(purchase: Purchase) -> Scenario {
    let monthly_statement = simulate_monthly(&purchase);
    let yearly_statement = aggregate_yearly(&monthly_statement);
    let total_statement = compute_metrics(&yearly_statement);

    Scenario {
        purchase,
        monthly_statement,
        yearly_statement,
        total_statement,
    }
}

/// Calculates mortgage monthly payment (pmt)
fn calculate_mortgage_pmt(principal: f64, rate: f64, year: u32) -> f64 {
    if principal <= 0.0 || year == 0 {
        return 0.0;
    }

    let total_payments = (year * 12) as i32;

    if rate <= 0.0 {
        return principal / total_payments as f64;
    }

    let monthly_rate = (rate * 0.01) / 12.0;
    let factor = (1.0 + monthly_rate).powi(total_payments);
    principal * (monthly_rate * factor) / (factor - 1.0)
}

/// Helper struct
struct Compound {
    pub interest: f64,
    pub total: f64,
}

/// Calculates 1-month interest compounding on a given balance and annual interest rate.
fn calculate_monthly_compound(amount: f64, annual_rate: f64) -> Compound {
    if amount <= 0.0 || annual_rate <= 0.0 {
        return Compound {
            interest: 0.0,
            total: amount.max(0.0),
        };
    }

    let monthly_rate = (annual_rate * 0.01) / 12.0;
    let interest = amount * monthly_rate;

    Compound {
        interest,
        total: amount + interest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::house::House;
    use std::collections::BTreeMap;

    #[test]
    fn test_calculate_mortgage_pmt_standard() {
        let pmt = calculate_mortgage_pmt(800_000.0, 6.5, 30);
        assert!((pmt - 5056.54).abs() < 0.1);
    }

    #[test]
    fn test_calculate_mortgage_pmt_zero_rate() {
        let pmt = calculate_mortgage_pmt(800_000.0, 0.0, 30);
        assert_eq!(pmt, 800_000.0 / 360.0);
    }

    #[test]
    fn test_calculate_mortgage_pmt_zero_principal_or_term() {
        assert_eq!(calculate_mortgage_pmt(0.0, 6.5, 30), 0.0);
        assert_eq!(calculate_mortgage_pmt(800_000.0, 6.5, 0), 0.0);
    }

    #[test]
    fn test_calculate_monthly_compound() {
        let c1 = calculate_monthly_compound(100_000.0, 6.0);
        assert_eq!(c1.interest, 500.0);
        assert_eq!(c1.total, 100_500.0);

        let c2 = calculate_monthly_compound(100_000.0, 0.0);
        assert_eq!(c2.interest, 0.0);
        assert_eq!(c2.total, 100_000.0);

        let c3 = calculate_monthly_compound(-50_000.0, 6.0);
        assert_eq!(c3.interest, 0.0);
        assert_eq!(c3.total, 0.0);
    }

    #[test]
    fn test_simulate_monthly_empty_tools() {
        let purchase = Purchase {
            name: "Empty Tools Purchase".to_string(),
            house: House {
                purchase_price: 500_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 1_200.0,
                monthly_hoa: 50.0,
            },
            tools: vec![],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        let result = simulate_monthly(&purchase);
        assert!(result.is_empty());
    }
}
