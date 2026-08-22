//! scenario_loading_and_math_test.rs
//! Integration test validating all scenario files in `scenarios/` directory,
//! simulation termination, accounting invariants, and single-scenario analysis metrics.

use cli::storage::serialize::{get_scenarios_dir_path, load_purchase_from_path};
use engine::config::constant::DEFAULT_MARGINAL_TAX_RATE;
use engine::service::analysis::analyze_scenario;
use engine::service::simulation::create_scenario;
use std::fs;

#[test]
fn test_load_all_scenarios_and_verify_mathematical_invariants() {
    let scenarios_dir = get_scenarios_dir_path();
    assert!(
        scenarios_dir.exists(),
        "Scenarios directory must exist at {:?}",
        scenarios_dir
    );

    let mut scenario_files = Vec::new();
    for entry in fs::read_dir(&scenarios_dir).expect("Failed to read scenarios directory") {
        let entry = entry.expect("Valid directory entry");
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            scenario_files.push(path);
        }
    }

    assert!(
        scenario_files.len() >= 16,
        "Expected at least 16 scenario JSON files, found {}",
        scenario_files.len()
    );

    for path in &scenario_files {
        let purchase = load_purchase_from_path(path)
            .unwrap_or_else(|e| panic!("Failed to load scenario at {:?}: {}", path, e));

        assert!(
            !purchase.name.is_empty(),
            "Scenario name must not be empty in {:?}",
            path
        );
        assert!(
            purchase.house.purchase_price > 0.0,
            "House price must be > 0 in {:?}",
            path
        );
        assert!(
            purchase.house.annual_property_tax_rate >= 0.0,
            "Property tax rate must be >= 0 in {:?}",
            path
        );
        assert!(
            purchase.house.annual_insurance >= 0.0,
            "Annual insurance must be >= 0 in {:?}",
            path
        );
        assert!(
            purchase.house.monthly_hoa >= 0.0,
            "Monthly HOA must be >= 0 in {:?}",
            path
        );

        let scenario = create_scenario(purchase.clone());
        let monthly = &scenario.monthly_statement;
        let yearly = &scenario.yearly_statement;
        let total = &scenario.total_statement;

        // 1. Monthly statement verification
        assert!(
            !monthly.is_empty(),
            "Monthly statement must not be empty for {:?}",
            purchase.name
        );
        assert!(
            monthly.len() <= 361,
            "Monthly statement length ({}) exceeds 361 for {:?}",
            monthly.len(),
            purchase.name
        );
        assert_eq!(
            monthly[0].month, 0,
            "Month 0 must be the first row for {:?}",
            purchase.name
        );

        for (i, row) in monthly.iter().enumerate() {
            assert_eq!(
                row.month as usize, i,
                "Month sequence must be continuous for {:?}",
                purchase.name
            );
            assert!(
                row.total_remaining_balance >= 0.0,
                "Remaining balance must be non-negative in month {} for {:?}",
                row.month,
                purchase.name
            );

            // Verify monthly net cash outlay equation: debt + extra + holding - cash_interest
            if row.month > 0 {
                let cash_int = row.cash.as_ref().map_or(0.0, |c| c.cash_interest);
                let expected_net_paid =
                    row.total_debt_paid + row.total_extra_payment + row.total_holding_cost
                        - cash_int;
                assert!(
                    (row.total_paid - expected_net_paid).abs() < 1e-4,
                    "Monthly total paid must match net cash outlay in month {} for {:?}",
                    row.month,
                    purchase.name
                );
            }
        }

        // Final month must clamp remaining balance to 0.0
        let last_monthly = monthly.last().unwrap();
        assert_eq!(
            last_monthly.total_remaining_balance, 0.0,
            "Final remaining balance must be 0.0 for {:?}",
            purchase.name
        );

        // 2. Yearly statement verification
        assert!(
            !yearly.is_empty(),
            "Yearly statement must not be empty for {:?}",
            purchase.name
        );
        for (y_idx, y_row) in yearly.iter().enumerate() {
            assert_eq!(
                y_row.year as usize,
                y_idx + 1,
                "Year index must be continuous for {:?}",
                purchase.name
            );
            assert!(
                y_row.ending_remaining_balance >= 0.0,
                "Ending remaining balance must be non-negative in year {} for {:?}",
                y_row.year,
                purchase.name
            );
        }

        let last_yearly = yearly.last().unwrap();
        assert_eq!(
            last_yearly.ending_remaining_balance, 0.0,
            "Final year ending balance must be 0.0 for {:?}",
            purchase.name
        );

        // Yearly sum vs Total Statement reconciliation
        let sum_cash_int: f64 = yearly.iter().map(|y| y.annual_cash_interest).sum();
        let sum_int_paid: f64 = yearly.iter().map(|y| y.annual_interest_paid).sum();
        let sum_tax_sav: f64 = yearly.iter().map(|y| y.annual_tax_savings).sum();
        let sum_holding: f64 = yearly.iter().map(|y| y.annual_holding_cost).sum();
        let sum_paid: f64 = yearly.iter().map(|y| y.annual_paid).sum();

        assert!(
            (sum_cash_int - total.total_cash_interest).abs() < 1e-4,
            "Cash interest sum reconciliation error for {:?}",
            purchase.name
        );
        assert!(
            (sum_int_paid - total.total_interest_paid).abs() < 1e-4,
            "Interest paid sum reconciliation error for {:?}",
            purchase.name
        );
        assert!(
            (sum_tax_sav - total.total_tax_savings).abs() < 1e-4,
            "Tax savings sum reconciliation error for {:?}",
            purchase.name
        );
        assert!(
            (sum_holding - total.total_holding_cost).abs() < 1e-4,
            "Holding cost sum reconciliation error for {:?}",
            purchase.name
        );
        assert!(
            (sum_paid - total.total_paid).abs() < 1e-4,
            "Total paid sum reconciliation error for {:?}",
            purchase.name
        );

        // 3. Tax savings bounds
        assert!(
            total.total_tax_savings >= 0.0,
            "Total tax savings must be non-negative for {:?}",
            purchase.name
        );
        assert!(
            total.total_tax_savings <= total.total_interest_paid * DEFAULT_MARGINAL_TAX_RATE + 1e-4,
            "Tax savings exceeds theoretical limit for {:?}",
            purchase.name
        );

        // 4. Single-scenario analysis metrics verification
        let analysis = analyze_scenario(&scenario);
        assert!(
            analysis.waste_ratio >= 0.0,
            "Waste ratio must be non-negative for {:?}",
            purchase.name
        );
        assert!(
            analysis.tax_savings_ratio >= 0.0
                && analysis.tax_savings_ratio <= DEFAULT_MARGINAL_TAX_RATE + 1e-4,
            "Tax savings ratio must be in [0, marginal_rate] for {:?}",
            purchase.name
        );
        assert_eq!(
            analysis.payoff_month, last_monthly.month,
            "Payoff month must match final month for {:?}",
            purchase.name
        );

        if analysis.payoff_month > 0 {
            let expected_monthly_cost = total.total_paid / analysis.payoff_month as f64;
            assert!(
                (analysis.effective_monthly_cost - expected_monthly_cost).abs() < 1e-4,
                "Effective monthly cost mismatch for {:?}",
                purchase.name
            );
        } else {
            assert_eq!(
                analysis.effective_monthly_cost, 0.0,
                "Effective monthly cost must be 0 for 0 payoff months in {:?}",
                purchase.name
            );
        }
    }
}
