//! wasm_parity_test.rs
//! Mathematical and structural parity tests between native engine and engine-wasm bindings.

use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::scenario::Scenario;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use engine::service::analysis::{ScenarioAnalysis, analyze_scenario};
use engine::service::comparison::{ScenarioComparison, compare_scenarios};
use engine::service::simulation::create_scenario;
use engine_wasm::{
    wasm_analyze_scenario_from_json, wasm_calculate_scenario_pv_from_json,
    wasm_compare_scenarios_from_json, wasm_create_scenario_from_json, wasm_solve_irr,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[test]
fn test_all_20_scenarios_wasm_parity() {
    let scenarios_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("scenarios");

    assert!(
        scenarios_dir.exists(),
        "scenarios directory must exist at {:?}",
        scenarios_dir
    );

    let entries = fs::read_dir(&scenarios_dir).expect("Read scenarios dir");
    let mut scenario_count = 0;

    for entry in entries {
        let entry = entry.expect("DirEntry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            scenario_count += 1;
            let json_content = fs::read_to_string(&path).expect("Read scenario file");
            let purchase: Purchase =
                serde_json::from_str(&json_content).expect("Parse Purchase from file");

            // 1. Native Engine Execution
            let native_scenario = create_scenario(purchase.clone());
            let native_analysis = analyze_scenario(&native_scenario);

            // 2. WASM Engine Execution (via JSON bridge)
            let wasm_scenario_json =
                wasm_create_scenario_from_json(&json_content).expect("WASM create_scenario");
            let wasm_scenario: Scenario =
                serde_json::from_str(&wasm_scenario_json).expect("Parse WASM scenario JSON");

            let wasm_analysis_json =
                wasm_analyze_scenario_from_json(&wasm_scenario_json).expect("WASM analyze");
            let wasm_analysis: ScenarioAnalysis =
                serde_json::from_str(&wasm_analysis_json).expect("Parse WASM analysis JSON");

            // 3. Verify Exact Scenario Parity
            assert_eq!(
                native_scenario.monthly_statement.len(),
                wasm_scenario.monthly_statement.len(),
                "Monthly statement length mismatch for {:?}",
                path.file_name()
            );

            assert_eq!(
                native_scenario.yearly_statement.len(),
                wasm_scenario.yearly_statement.len(),
                "Yearly statement length mismatch for {:?}",
                path.file_name()
            );

            assert!(
                (native_scenario.total_statement.total_paid
                    - wasm_scenario.total_statement.total_paid)
                    .abs()
                    < 1e-6,
                "Total paid mismatch for {:?}",
                path.file_name()
            );

            assert!(
                (native_scenario.total_statement.total_interest_paid
                    - wasm_scenario.total_statement.total_interest_paid)
                    .abs()
                    < 1e-6,
                "Total interest mismatch for {:?}",
                path.file_name()
            );

            // 4. Verify Analysis Parity
            assert_eq!(
                native_analysis.payoff_month,
                wasm_analysis.payoff_month,
                "Payoff month mismatch for {:?}",
                path.file_name()
            );
            assert!(
                (native_analysis.effective_monthly_cost - wasm_analysis.effective_monthly_cost)
                    .abs()
                    < 1e-6,
                "Effective monthly cost mismatch for {:?}",
                path.file_name()
            );
            assert!(
                (native_analysis.waste_ratio - wasm_analysis.waste_ratio).abs() < 1e-6,
                "Waste ratio mismatch for {:?}",
                path.file_name()
            );
            assert!(
                (native_analysis.tax_savings_ratio - wasm_analysis.tax_savings_ratio).abs() < 1e-6,
                "Tax savings ratio mismatch for {:?}",
                path.file_name()
            );
        }
    }

    assert_eq!(scenario_count, 20, "Must test all 20 scenario files");
}

#[test]
fn test_pairwise_comparison_wasm_parity() {
    let purchase_a = Purchase {
        name: "Standard 30yr".to_string(),
        house: House {
            purchase_price: 1_200_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 2_400.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 240_000.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 960_000.0,
                rate: 6.5,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let mut purchase_b = purchase_a.clone();
    purchase_b.name = "Accelerated 15yr".to_string();
    purchase_b.tools = vec![
        Tool::Cash(Cash {
            amount: 240_000.0,
            rate: 4.0,
        }),
        Tool::Mortgage(Mortgage {
            amount: 960_000.0,
            rate: 5.75,
            term: 15,
        }),
    ];

    let native_a = create_scenario(purchase_a.clone());
    let native_b = create_scenario(purchase_b.clone());
    let native_comp = compare_scenarios(&native_a, &native_b);

    let json_a = serde_json::to_string(&purchase_a).unwrap();
    let json_b = serde_json::to_string(&purchase_b).unwrap();
    let wasm_scen_a = wasm_create_scenario_from_json(&json_a).unwrap();
    let wasm_scen_b = wasm_create_scenario_from_json(&json_b).unwrap();

    let wasm_comp_json = wasm_compare_scenarios_from_json(&wasm_scen_a, &wasm_scen_b).unwrap();
    let wasm_comp: ScenarioComparison = serde_json::from_str(&wasm_comp_json).unwrap();

    assert_eq!(native_comp.months_saved, wasm_comp.months_saved);
    assert!((native_comp.delta_interest_paid - wasm_comp.delta_interest_paid).abs() < 1e-6);
    assert!((native_comp.delta_gross_paid - wasm_comp.delta_gross_paid).abs() < 1e-6);
    assert!((native_comp.delta_pv - wasm_comp.delta_pv).abs() < 1e-6);
    assert_eq!(native_comp.irr.is_some(), wasm_comp.irr.is_some());
}

#[test]
fn test_edge_case_empty_tools_and_zero_values() {
    let empty_purchase = Purchase {
        name: "Empty".to_string(),
        house: House {
            purchase_price: 0.0,
            annual_property_tax_rate: 0.0,
            annual_insurance: 0.0,
            monthly_hoa: 0.0,
        },
        tools: vec![],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let json_str = serde_json::to_string(&empty_purchase).unwrap();
    let scen_json = wasm_create_scenario_from_json(&json_str).unwrap();
    let scenario: Scenario = serde_json::from_str(&scen_json).unwrap();
    assert!(scenario.monthly_statement.is_empty());

    let analysis_json = wasm_analyze_scenario_from_json(&scen_json).unwrap();
    let analysis: ScenarioAnalysis = serde_json::from_str(&analysis_json).unwrap();
    assert_eq!(analysis.payoff_month, 0);
    assert_eq!(analysis.effective_monthly_cost, 0.0);
}

#[test]
fn test_irr_solver_wasm_boundary() {
    let bad_json = "not json";
    assert!(wasm_solve_irr(bad_json).is_err());

    let empty_flows = serde_json::to_string(&Vec::<f64>::new()).unwrap();
    let result = wasm_solve_irr(&empty_flows).unwrap();
    assert_eq!(result, None);
}
