use crate::config::constants::*;
use crate::config::utility::*;
use crate::domain::scenario::*;
use crate::domain::statement::*;
use crate::domain::tool::*;
use crate::service::formula::*;

// Simulate Monthly Statement Row
pub fn simulate_monthly(scenario: &Scenario) -> Vec<MonthlyStatementRow> {
    let mut schedule = Vec::with_capacity(360);

    // 1. Extract Tool Configuration
    let mut cash_opt = None;
    let mut mortgage_opt = None;
    let mut loc_opt = None;

    for tool in &scenario.tools {
        match tool {
            Tool::Cash(c) => cash_opt = Some(c.clone()),
            Tool::Mortgage(m) => mortgage_opt = Some(m.clone()),
            Tool::Loc(l) => loc_opt = Some(l.clone()),
        }
    }

    // 2. State Tracking Variables
    let mut cash_balance = cash_opt
        .as_ref()
        .map_or(0.0, |c| DEFAULT_STARTING_CASH - c.amount);
    let mut mortgage_balance = mortgage_opt.as_ref().map_or(0.0, |m| m.amount);
    let mut loc_balance = loc_opt.as_ref().map_or(0.0, |l| l.amount);
    let mut monthly_property_tax =
        scenario.house.purchase_price * scenario.house.annual_property_tax_rate * 0.01 / 12.0;
    let mut monthly_insurance = scenario.house.annual_insurance / 12.0;
    let mut monthly_hoa = scenario.house.monthly_hoa;

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
                interest_earned: cash_compound.interest,
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
            let extra_payment = scenario
                .mortgage_repay
                .get(&month)
                .copied()
                .unwrap_or(0.0)
                .min(mortgage_balance - principal_paid);
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
            let extra_payment = scenario
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
            - cash_statement.as_ref().map_or(0.0, |c| c.interest_earned);
        let total_remaining_balance = clamp_zero(
            mortgage_statement
                .as_ref()
                .map_or(0.0, |m| m.remaining_balance)
                + loc_statement.as_ref().map_or(0.0, |l| l.remaining_balance),
        );

        schedule.push(MonthlyStatementRow {
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

    schedule
}

pub fn aggregate_yearly(schedule: &[MonthlyStatementRow]) -> Vec<YearlyStatementRow> {
    schedule
        .chunks(12)
        .enumerate()
        .map(|(year_id, chunk)| {
            let year = (year_id + 1) as u32;

            let mut annual_cash_interest = 0.0;
            let mut annual_mortgage_interest = 0.0;
            let mut annual_debt_paid = 0.0;
            let mut annual_extra_payment = 0.0;
            let mut annual_holding_cost = 0.0;
            let mut annual_paid_unadjusted = 0.0;

            let mut monthly_mortgage_balance = Vec::with_capacity(chunk.len());

            for row in chunk {
                if let Some(c) = &row.cash {
                    annual_cash_interest += c.interest_earned;
                }
                if let Some(m) = &row.mortgage {
                    annual_mortgage_interest += m.interest_paid;
                    monthly_mortgage_balance
                        .push(m.principal_paid + m.remaining_balance + m.extra_payment);
                }
                annual_debt_paid += row.total_debt_paid;
                annual_extra_payment += row.total_extra_payment;
                annual_holding_cost += row.total_holding_cost;
                annual_paid_unadjusted += row.total_paid;
            }

            // Annual Tax Savings
            let annual_tax_savings = if annual_mortgage_interest > 0.0
                && !monthly_mortgage_balance.is_empty()
            {
                let average_mortgage_balance: f64 = monthly_mortgage_balance.iter().sum::<f64>()
                    / monthly_mortgage_balance.len() as f64;
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
