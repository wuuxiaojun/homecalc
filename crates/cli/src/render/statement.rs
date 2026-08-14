//! statement.rs
//! Renders monthly statement schedule, yearly summary statement, and lifetime total statement tables.

use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use engine::domain::scenario::Scenario;

use super::format::format_currency;

/// Renders monthly schedule table, yearly summary table, and lifetime total metrics summary.
pub fn render_statement(scenario: &Scenario) {
    let monthly = &scenario.monthly_statement;
    let yearly = &scenario.yearly_statement;
    let total = &scenario.total_statement;

    // 1. Monthly Schedule Table
    println!("\n================================================================================");
    println!(
        " MONTHLY STATEMENT SCHEDULE (Total Active Months: {})",
        monthly.len()
    );
    println!("================================================================================");

    let mut monthly_table = Table::new();
    monthly_table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Month",
            "Cash Bal",
            "Mortg PMT",
            "Mortg Extra",
            "Mortg Bal",
            "LOC PMT",
            "LOC Extra",
            "LOC Bal",
            "Holding Cost",
            "Total Paid",
            "Total Rem Bal",
        ]);

    let total_len = monthly.len();
    let mut indices_to_print = Vec::new();

    for (i, row) in monthly.iter().enumerate() {
        let is_first_6 = i < 6;
        let is_last_3 = i >= total_len.saturating_sub(3);
        let has_extra = row.total_extra_payment > 0.0;

        if is_first_6 || is_last_3 || has_extra {
            indices_to_print.push(i);
        }
    }

    let mut last_printed: Option<usize> = None;

    for &idx in &indices_to_print {
        if let Some(prev) = last_printed {
            if idx > prev + 1 {
                let skipped = idx - prev - 1;
                monthly_table.add_row(vec![
                    format!("... [{} mo skipped]", skipped),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                    "...".into(),
                ]);
            }
        }
        last_printed = Some(idx);

        let row = &monthly[idx];
        let cash_bal = row.cash.as_ref().map_or(0.0, |c| c.cash_now);
        let mortg_pmt = row.mortgage.as_ref().map_or(0.0, |m| {
            (m.principal_paid + m.interest_paid).min(m.monthly_payment)
        });
        let mortg_extra = row.mortgage.as_ref().map_or(0.0, |m| m.extra_payment);
        let mortg_bal = row.mortgage.as_ref().map_or(0.0, |m| m.remaining_balance);

        let loc_pmt = row.loc.as_ref().map_or(0.0, |l| l.monthly_payment);
        let loc_extra = row.loc.as_ref().map_or(0.0, |l| l.extra_payment);
        let loc_bal = row.loc.as_ref().map_or(0.0, |l| l.remaining_balance);

        monthly_table.add_row(vec![
            row.month.to_string(),
            format_currency(cash_bal),
            format_currency(mortg_pmt),
            format_currency(mortg_extra),
            format_currency(mortg_bal),
            format_currency(loc_pmt),
            format_currency(loc_extra),
            format_currency(loc_bal),
            format_currency(row.total_holding_cost),
            format_currency(row.total_paid),
            format_currency(row.total_remaining_balance),
        ]);
    }

    println!("{monthly_table}");

    // 2. Yearly Summary Table
    println!("\n================================================================================");
    println!(" YEARLY SUMMARY STATEMENT");
    println!("================================================================================");

    let mut yearly_table = Table::new();
    yearly_table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Year",
            "Cash Yield",
            "Interest Paid",
            "Debt Paid",
            "Extra Paid",
            "Holding Cost",
            "Tax Savings",
            "Net Annual Paid",
            "Ending Rem Bal",
        ]);

    for y in yearly {
        yearly_table.add_row(vec![
            y.year.to_string(),
            format_currency(y.annual_cash_interest),
            format_currency(y.annual_interest_paid),
            format_currency(y.annual_debt_paid),
            format_currency(y.annual_extra_payment),
            format_currency(y.annual_holding_cost),
            format_currency(y.annual_tax_savings),
            format_currency(y.annual_paid),
            format_currency(y.ending_remaining_balance),
        ]);
    }

    println!("{yearly_table}");

    // 3.  Total Summary Table
    println!("\n--- TOTAL STATEMENT SUMMARY ---");
    let mut total_table = Table::new();
    total_table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Lifetime Summary Metric", "Total Amount"]);

    total_table.add_row(vec![
        "Total Cash Yield Earned",
        &format_currency(total.total_cash_interest),
    ]);
    total_table.add_row(vec![
        "Total Interest Paid",
        &format_currency(total.total_interest_paid),
    ]);
    total_table.add_row(vec![
        "Total Holding Cost Paid",
        &format_currency(total.total_holding_cost),
    ]);
    total_table.add_row(vec![
        "Total Tax Savings Realized",
        &format_currency(total.total_tax_savings),
    ]);
    total_table.add_row(vec![
        "Total Net Cash Paid Out-of-Pocket",
        &format_currency(total.total_paid),
    ]);

    println!("{total_table}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::domain::house::House;
    use engine::domain::purchase::Purchase;
    use engine::domain::tool::{Cash, Mortgage, Tool};
    use engine::service::simulation::create_scenario;
    use std::collections::BTreeMap;

    #[test]
    fn test_render_statement() {
        let purchase = Purchase {
            name: "Test Statement Scenario".to_string(),
            house: House {
                purchase_price: 1_000_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 2_400.0,
                monthly_hoa: 100.0,
            },
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

        let scenario = create_scenario(purchase);
        render_statement(&scenario);
    }
}
