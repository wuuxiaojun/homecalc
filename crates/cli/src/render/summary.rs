//! summary.rs
//! Renders basic purchase information, property details, financial tools, and extra repayment schedules.

use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use engine::domain::scenario::Scenario;
use engine::domain::tool::Tool;

use super::format::format_currency;

/// Renders a summary table of basic information stored directly in `Purchase`.
pub fn render_summary(scenario: &Scenario) {
    let purchase = &scenario.purchase;
    let house = &purchase.house;

    println!("\n================================================================================");
    println!(" 📋 SCENARIO SUMMARY: {}", purchase.name);
    println!("================================================================================");

    // 1. Property Details Table
    let mut house_table = Table::new();
    house_table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Property Parameter", "Value"]);

    house_table.add_row(vec![
        "🏡 Purchase Price",
        &format_currency(house.purchase_price),
    ]);
    house_table.add_row(vec![
        "🏛️  Annual Property Tax Rate",
        &format!("{:.2}%", house.annual_property_tax_rate),
    ]);
    house_table.add_row(vec![
        "🛡️  Annual Insurance",
        &format_currency(house.annual_insurance),
    ]);
    house_table.add_row(vec!["🏢 Monthly HOA", &format_currency(house.monthly_hoa)]);

    println!("\n🏡 Property Details");
    println!("{house_table}");

    // 2. Financial Tools Table
    let mut tools_table = Table::new();
    tools_table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Type", "Principal", "Interest", "Term"]);

    if let Some(c) = purchase.cash() {
        tools_table.add_row(vec![
            "💵 Cash",
            &format_currency(c.amount),
            &format!("{:.2}%", c.rate),
            "N/A",
        ]);
    }

    if let Some(m) = purchase.mortgage() {
        tools_table.add_row(vec![
            "🏦 Mortgage",
            &format_currency(m.amount),
            &format!("{:.2}%", m.rate),
            &format!("{} Years", m.term),
        ]);
    }

    if let Some(l) = purchase.loc() {
        tools_table.add_row(vec![
            "💳 Line of Credit (LOC)",
            &format_currency(l.amount),
            &format!("{:.2}%", l.rate),
            "N/A",
        ]);
    }

    println!("\n💳 Financial Tools");
    println!("{tools_table}");

    // 3. Extra Repayments Table
    if !purchase.mortgage_repay.is_empty() || !purchase.loc_repay.is_empty() {
        let mut repay_table = Table::new();
        repay_table
            .load_style(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Loan Type", "Month Number", "Extra Payment Amount"]);

        for (month, amt) in &purchase.mortgage_repay {
            repay_table.add_row(vec!["🏦 Mortgage", &month.to_string(), &format_currency(*amt)]);
        }
        for (month, amt) in &purchase.loc_repay {
            repay_table.add_row(vec!["💳 LOC", &month.to_string(), &format_currency(*amt)]);
        }

        println!("\n⚡ Scheduled Extra Principal Repayments");
        println!("{repay_table}");
    }
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
    fn test_render_purchase_summary() {
        let purchase = Purchase {
            name: "Test Info Scenario".to_string(),
            house: House {
                purchase_price: 1_200_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 3_000.0,
                monthly_hoa: 150.0,
            },
            tools: vec![
                Tool::Cash(Cash {
                    amount: 200_000.0,
                    rate: 4.0,
                }),
                Tool::Mortgage(Mortgage {
                    amount: 1_000_000.0,
                    rate: 6.0,
                    term: 30,
                }),
            ],
            mortgage_repay: BTreeMap::from([(12, 50_000.0)]),
            loc_repay: BTreeMap::new(),
        };

        let scenario = create_scenario(purchase);
        render_summary(&scenario);
    }
}
