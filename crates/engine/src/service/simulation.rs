//! simulation.rs
//! Monthly yearly and total simulation
use crate::config::constant::*;
use crate::domain::purchase::*;
use crate::domain::scenario::*;
use crate::domain::statement::*;
use crate::service::utility::*;

/// Simulate monthly statement
pub fn simulate_monthly(purchase: &Purchase) -> Vec<MonthlyStatementRow> {
    if purchase.tools.is_empty() {
        return Vec::new();
    }

    let mut statement = Vec::with_capacity(361);

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

    // 0. Month 0
    let cash_0 = cash_opt.map(|_| CashStatement {
        cash_now: cash_balance,
        cash_interest: 0.0,
    });
    let mortgage_0 = mortgage_opt.map(|_| MortgageStatement {
        monthly_payment: 0.0,
        principal_paid: 0.0,
        interest_paid: 0.0,
        extra_payment: 0.0,
        remaining_balance: mortgage_balance,
    });
    let loc_0 = loc_opt.map(|_| LocStatement {
        monthly_payment: 0.0,
        extra_payment: 0.0,
        remaining_balance: loc_balance,
    });
    let house_0 = HouseStatement {
        monthly_property_tax: 0.0,
        monthly_insurance: 0.0,
        monthly_hoa: 0.0,
    };
    statement.push(MonthlyStatementRow {
        month: 0,
        cash: cash_0,
        mortgage: mortgage_0,
        loc: loc_0,
        house: house_0,
        total_debt_paid: 0.0,
        total_extra_payment: 0.0,
        total_holding_cost: 0.0,
        total_paid: cash_opt.map_or(0.0, |c| c.amount),
        total_remaining_balance: mortgage_balance + loc_balance,
    });

    if mortgage_balance <= 0.0 && loc_balance <= 0.0 {
        return statement;
    }

    let mut monthly_property_tax = purchase.house.monthly_property_tax();
    let mut monthly_insurance = purchase.house.monthly_insurance();
    let mut monthly_hoa = purchase.house.monthly_hoa;

    // 3. PMT
    let mortgage_pmt = mortgage_opt
        .as_ref()
        .map_or(0.0, |m| calculate_mortgage_pmt(m.amount, m.rate, m.term));

    let monthly_cash_rate = cash_opt.map_or(0.0, |c| (c.rate * 0.01) / 12.0);
    let monthly_mortgage_rate = mortgage_opt.map_or(0.0, |m| (m.rate * 0.01) / 12.0);
    let monthly_loc_rate = loc_opt.map_or(0.0, |l| (l.rate * 0.01) / 12.0);

    // 4. Monthly Simulation Loop
    for month in 1..=360 {
        // Cash Statement
        let cash_statement = cash_opt.as_ref().map(|_| {
            let cash_compound = calculate_monthly_compound(cash_balance, monthly_cash_rate);
            cash_balance = cash_compound.total;
            CashStatement {
                cash_now: cash_balance,
                cash_interest: cash_compound.interest,
            }
        });

        // Mortgage Statement
        let mortgage_statement = mortgage_opt.as_ref().and_then(|_| {
            if mortgage_balance <= 0.0 {
                return None;
            }

            let interest_paid = calculate_monthly_compound(mortgage_balance, monthly_mortgage_rate)
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
        let loc_statement = loc_opt.as_ref().and_then(|_| {
            if loc_balance <= 0.0 {
                return None;
            }
            let monthly_payment =
                calculate_monthly_compound(loc_balance, monthly_loc_rate).interest;
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

    statement.shrink_to_fit();
    statement
}

/// Aggregate monthly statement into yearly statement
pub fn aggregate_yearly(statement: &[MonthlyStatementRow]) -> Vec<YearlyStatementRow> {
    if statement.is_empty() {
        return Vec::new();
    }

    let mut chunks: Vec<&[MonthlyStatementRow]> = Vec::new();

    if statement[0].month == 0 {
        let y1_len = 13.min(statement.len());
        chunks.push(&statement[0..y1_len]);

        let remaining = &statement[y1_len..];
        for chunk in remaining.chunks(12) {
            chunks.push(chunk);
        }
    } else {
        for chunk in statement.chunks(12) {
            chunks.push(chunk);
        }
    }

    chunks
        .into_iter()
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
                    if row.month > 0 {
                        mortgage_balance_sum +=
                            m.principal_paid + m.remaining_balance + m.extra_payment;
                        mortgage_balance_count += 1;
                    }
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
            let annual_tax_savings = if annual_mortgage_interest > 0.0 && mortgage_balance_count > 0
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
    let mut total_cash_interest = 0.0;
    let mut total_holding_cost = 0.0;
    let mut total_interest_paid = 0.0;
    let mut total_tax_savings = 0.0;
    let mut total_paid = 0.0;

    for row in yearly_statement {
        total_cash_interest += row.annual_cash_interest;
        total_holding_cost += row.annual_holding_cost;
        total_interest_paid += row.annual_interest_paid;
        total_tax_savings += row.annual_tax_savings;
        total_paid += row.annual_paid;
    }

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

/// Calculates 1-month interest compounding on a given balance and pre-computed monthly rate.
fn calculate_monthly_compound(amount: f64, monthly_rate: f64) -> Compound {
    if amount <= 0.0 || monthly_rate <= 0.0 {
        return Compound {
            interest: 0.0,
            total: amount.max(0.0),
        };
    }

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
        let monthly_rate = (6.0 * 0.01) / 12.0;
        let c1 = calculate_monthly_compound(100_000.0, monthly_rate);
        assert_eq!(c1.interest, 500.0);
        assert_eq!(c1.total, 100_500.0);

        let c2 = calculate_monthly_compound(100_000.0, 0.0);
        assert_eq!(c2.interest, 0.0);
        assert_eq!(c2.total, 100_000.0);

        let c3 = calculate_monthly_compound(-50_000.0, monthly_rate);
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

    #[test]
    fn test_simulate_monthly_month_0_and_yearly_aggregation() {
        let purchase = Purchase {
            name: "Standard Scenario".to_string(),
            house: House {
                purchase_price: 1_000_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 2_400.0,
                monthly_hoa: 100.0,
            },
            tools: vec![
                crate::domain::tool::Tool::Cash(crate::domain::tool::Cash {
                    amount: 200_000.0,
                    rate: 4.0,
                }),
                crate::domain::tool::Tool::Mortgage(crate::domain::tool::Mortgage {
                    amount: 800_000.0,
                    rate: 6.0,
                    term: 30,
                }),
            ],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        let monthly = simulate_monthly(&purchase);
        assert_eq!(monthly.len(), 361);
        assert_eq!(monthly[0].month, 0);
        assert_eq!(monthly[0].total_paid, 200_000.0);
        assert_eq!(monthly[1].month, 1);
        assert_eq!(monthly[360].month, 360);

        let yearly = aggregate_yearly(&monthly);
        assert_eq!(yearly.len(), 30);
        assert_eq!(yearly[0].year, 1);
        // Year 1 includes month 0 down payment (200_000)
        assert!(yearly[0].annual_paid > 200_000.0);
    }
}
