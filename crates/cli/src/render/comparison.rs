//! comparison.rs
//! Formats and displays 4-column side-by-side comparison tables (`ScenarioComparison`).

use super::format::{format_currency, format_months, format_percent};
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use engine::service::comparison::ScenarioComparison;

/// Formats and displays scenario comparison metrics in a 4-column side-by-side table:
/// `[Metric Category / Field | Baseline (A) | Alternative (B) | Delta (B - A)]`.
pub fn render_comparison(comparison: &ScenarioComparison) {
    println!("\n================================================================================");
    println!(" ⚖️  SCENARIO COMPARISON");
    println!("================================================================================");

    let mut table = Table::new();
    table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Metric", "Baseline", "Alternative", "Delta"]);

    // 1. Timeline
    table.add_row(vec![
        "⏱️  Payoff Timeline",
        &format_months(comparison.baseline_payoff_month),
        &format_months(comparison.alternative_payoff_month),
        &format!("{} Months Saved", comparison.months_saved),
    ]);

    // 2. Outflows
    table.add_row(vec![
        "⚡ Extra Principal Paid",
        &format_currency(comparison.baseline_extra_payment),
        &format_currency(comparison.alternative_extra_payment),
        &format_currency(comparison.delta_extra_payment),
    ]);
    table.add_row(vec![
        "📉 Total Interest Paid",
        &format_currency(comparison.baseline_interest_paid),
        &format_currency(comparison.alternative_interest_paid),
        &format_currency(comparison.delta_interest_paid),
    ]);

    // 3. Inflows
    table.add_row(vec![
        "💵 Total Cash Yield Earned",
        &format_currency(comparison.baseline_cash_interest),
        &format_currency(comparison.alternative_cash_interest),
        &format_currency(comparison.delta_cash_interest),
    ]);
    table.add_row(vec![
        "🧾 Total Tax Savings",
        &format_currency(comparison.baseline_tax_savings),
        &format_currency(comparison.alternative_tax_savings),
        &format_currency(comparison.delta_tax_savings),
    ]);

    // 4. Aggregate
    table.add_row(vec![
        "💳 Gross Paid",
        &format_currency(comparison.baseline_gross_paid),
        &format_currency(comparison.alternative_gross_paid),
        &format_currency(comparison.delta_gross_paid),
    ]);

    // 5. Analytics
    table.add_row(vec![
        "📊 Present Value Outflow",
        &format_currency(comparison.baseline_pv),
        &format_currency(comparison.alternative_pv),
        &format_currency(comparison.delta_pv),
    ]);

    let irr_str = comparison
        .irr
        .map_or("N/A".to_string(), |r| format_percent(r * 100.0));

    table.add_row(vec!["📈 Strategy IRR", "N/A", "N/A", &irr_str]);

    println!("{table}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::domain::house::House;
    use engine::domain::purchase::Purchase;
    use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
    use engine::service::comparison::compare_scenarios;
    use engine::service::simulation::create_scenario;
    use std::collections::BTreeMap;

    #[test]
    fn test_render_comparison() {
        let purchase_a = Purchase {
            name: "Scenario A".to_string(),
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

        let mut purchase_b = purchase_a.clone();
        purchase_b.name = "Scenario B".to_string();
        purchase_b.mortgage_repay.insert(12, 100_000.0);

        let scenario_a = create_scenario(purchase_a);
        let scenario_b = create_scenario(purchase_b);

        let comparison = compare_scenarios(&scenario_a, &scenario_b);
        render_comparison(&comparison);
    }

    #[test]
    fn test_render_comparison_no_irr() {
        let purchase_a = Purchase {
            name: "Scenario A".to_string(),
            house: House {
                purchase_price: 500_000.0,
                annual_property_tax_rate: 1.0,
                annual_insurance: 1_200.0,
                monthly_hoa: 0.0,
            },
            tools: vec![Tool::Cash(Cash {
                amount: 500_000.0,
                rate: 4.0,
            })],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        let purchase_b = Purchase {
            name: "Scenario B".to_string(),
            house: House {
                purchase_price: 500_000.0,
                annual_property_tax_rate: 1.0,
                annual_insurance: 1_200.0,
                monthly_hoa: 0.0,
            },
            tools: vec![Tool::Loc(Loc {
                amount: 500_000.0,
                rate: 5.0,
            })],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        let scenario_a = create_scenario(purchase_a);
        let scenario_b = create_scenario(purchase_b);

        let comparison = compare_scenarios(&scenario_a, &scenario_b);
        render_comparison(&comparison);
    }
}
