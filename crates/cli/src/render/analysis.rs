//! analysis.rs
//! Formats and displays single scenario analysis metrics (`ScenarioAnalysis`).

use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use engine::service::analysis::ScenarioAnalysis;

use super::format::{format_currency, format_months, format_percent};

/// Formats and displays single-scenario analysis metrics cleanly.
pub fn render_analysis(analysis: &ScenarioAnalysis) {
    println!("\n================================================================================");
    println!(" 🔍 SCENARIO ANALYSIS");
    println!("================================================================================");

    let mut table = Table::new();
    table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Analysis Metric", "Value"]);

    table.add_row(vec![
        "⏱️  Payoff Timeline",
        &format!(
            "{} (Month {})",
            format_months(analysis.payoff_month),
            analysis.payoff_month
        ),
    ]);
    table.add_row(vec![
        "💳 Effective Monthly Outlay",
        &format_currency(analysis.effective_monthly_cost),
    ]);
    table.add_row(vec![
        "🗑️  Waste Ratio",
        &format_percent(analysis.waste_ratio * 100.0),
    ]);
    table.add_row(vec![
        "🧾 Tax Savings Ratio",
        &format_percent(analysis.tax_savings_ratio * 100.0),
    ]);

    println!("{table}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::domain::house::House;
    use engine::domain::purchase::Purchase;
    use engine::domain::tool::{Cash, Mortgage, Tool};
    use engine::service::analysis::analyze_scenario;
    use engine::service::simulation::create_scenario;
    use std::collections::BTreeMap;

    #[test]
    fn test_render_single_analysis() {
        let purchase = Purchase {
            name: "Analysis Test Scenario".to_string(),
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
        let analysis = analyze_scenario(&scenario);
        render_analysis(&analysis);
    }
}
