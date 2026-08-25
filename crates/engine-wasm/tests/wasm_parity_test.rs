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

fn sample_scenarios() -> Vec<Purchase> {
    let mut scenarios = Vec::new();

    // 1. Standard 30yr conventional mortgage
    scenarios.push(Purchase {
        name: "Standard 30Y Mortgage".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.25,
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
                rate: 6.5,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 2. Standard 15yr fixed mortgage
    scenarios.push(Purchase {
        name: "15Y Fixed Mortgage".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.25,
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
                rate: 5.75,
                term: 15,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 3. All Cash Starter
    scenarios.push(Purchase {
        name: "All Cash Starter".to_string(),
        house: House {
            purchase_price: 400_000.0,
            annual_property_tax_rate: 1.1,
            annual_insurance: 1_200.0,
            monthly_hoa: 50.0,
        },
        tools: vec![Tool::Cash(Cash {
            amount: 400_000.0,
            rate: 4.5,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 4. Pure LOC Aggressive Payoff
    let mut loc_4_repay = BTreeMap::new();
    for m in 1..=36 {
        loc_4_repay.insert(m, 15_000.0);
    }
    scenarios.push(Purchase {
        name: "LOC Aggressive 3Y".to_string(),
        house: House {
            purchase_price: 500_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 1_800.0,
            monthly_hoa: 0.0,
        },
        tools: vec![Tool::Loc(Loc {
            amount: 500_000.0,
            rate: 7.0,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: loc_4_repay,
    });

    // 5. Pure LOC Long
    scenarios.push(Purchase {
        name: "LOC Long".to_string(),
        house: House {
            purchase_price: 800_000.0,
            annual_property_tax_rate: 1.3,
            annual_insurance: 2_000.0,
            monthly_hoa: 75.0,
        },
        tools: vec![Tool::Loc(Loc {
            amount: 800_000.0,
            rate: 6.5,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 6. Pure Mortgage Long
    scenarios.push(Purchase {
        name: "Pure Mortgage Long".to_string(),
        house: House {
            purchase_price: 800_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 1_800.0,
            monthly_hoa: 0.0,
        },
        tools: vec![Tool::Mortgage(Mortgage {
            amount: 800_000.0,
            rate: 6.0,
            term: 30,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 7. Hybrid Mortgage + LOC Split
    scenarios.push(Purchase {
        name: "Hybrid Split".to_string(),
        house: House {
            purchase_price: 1_200_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 2_400.0,
            monthly_hoa: 150.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 200_000.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 800_000.0,
                rate: 6.5,
                term: 30,
            }),
            Tool::Loc(Loc {
                amount: 200_000.0,
                rate: 7.25,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 8. High Inflation Holding Cost
    scenarios.push(Purchase {
        name: "High Holding Cost".to_string(),
        house: House {
            purchase_price: 900_000.0,
            annual_property_tax_rate: 2.5,
            annual_insurance: 5_000.0,
            monthly_hoa: 500.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 180_000.0,
                rate: 3.5,
            }),
            Tool::Mortgage(Mortgage {
                amount: 720_000.0,
                rate: 7.0,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 9. Low Down FHA Style
    scenarios.push(Purchase {
        name: "Low Down Payment".to_string(),
        house: House {
            purchase_price: 500_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 1_500.0,
            monthly_hoa: 50.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 17_500.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 482_500.0,
                rate: 6.25,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 10. Lump Sum Early Payoff
    let mut mort_10_repay = BTreeMap::new();
    mort_10_repay.insert(36, 100_000.0);
    scenarios.push(Purchase {
        name: "Lump Sum Month 36".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.25,
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
                rate: 6.5,
                term: 30,
            }),
        ],
        mortgage_repay: mort_10_repay,
        loc_repay: BTreeMap::new(),
    });

    // 11. Intermittent Seasonal Extra Pay
    let mut mort_11_repay = BTreeMap::new();
    mort_11_repay.insert(12, 10_000.0);
    mort_11_repay.insert(24, 10_000.0);
    mort_11_repay.insert(36, 10_000.0);
    mort_11_repay.insert(48, 10_000.0);
    mort_11_repay.insert(60, 10_000.0);
    scenarios.push(Purchase {
        name: "Annual Bonus Prepayments".to_string(),
        house: House {
            purchase_price: 750_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 1_800.0,
            monthly_hoa: 0.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 150_000.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 600_000.0,
                rate: 6.5,
                term: 30,
            }),
        ],
        mortgage_repay: mort_11_repay,
        loc_repay: BTreeMap::new(),
    });

    // 12. Balloon Payoff
    let mut mort_12_repay = BTreeMap::new();
    mort_12_repay.insert(60, 700_000.0);
    scenarios.push(Purchase {
        name: "Balloon Payoff 5Y".to_string(),
        house: House {
            purchase_price: 800_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 2_000.0,
            monthly_hoa: 100.0,
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
        mortgage_repay: mort_12_repay,
        loc_repay: BTreeMap::new(),
    });

    // 13. Luxury Estate Cash LOC
    scenarios.push(Purchase {
        name: "Luxury Estate".to_string(),
        house: House {
            purchase_price: 3_000_000.0,
            annual_property_tax_rate: 1.3,
            annual_insurance: 6_000.0,
            monthly_hoa: 800.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 1_000_000.0,
                rate: 4.5,
            }),
            Tool::Loc(Loc {
                amount: 2_000_000.0,
                rate: 7.5,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 14. Micro Condo High HOA
    scenarios.push(Purchase {
        name: "Micro Condo".to_string(),
        house: House {
            purchase_price: 250_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 800.0,
            monthly_hoa: 600.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 50_000.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 200_000.0,
                rate: 6.75,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 15. Jumbo Mortgage High Tax
    scenarios.push(Purchase {
        name: "Jumbo High Tax".to_string(),
        house: House {
            purchase_price: 2_000_000.0,
            annual_property_tax_rate: 2.0,
            annual_insurance: 4_000.0,
            monthly_hoa: 200.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 400_000.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 1_600_000.0,
                rate: 6.875,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 16. Dual Loan Staggered Payoff
    let mut loc_16_repay = BTreeMap::new();
    for m in 1..=24 {
        loc_16_repay.insert(m, 5_000.0);
    }
    scenarios.push(Purchase {
        name: "Dual Loan Staggered".to_string(),
        house: House {
            purchase_price: 1_100_000.0,
            annual_property_tax_rate: 1.25,
            annual_insurance: 2_200.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 200_000.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 800_000.0,
                rate: 6.5,
                term: 30,
            }),
            Tool::Loc(Loc {
                amount: 100_000.0,
                rate: 7.5,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: loc_16_repay,
    });

    // 17. Rapid 1yr Payoff
    let mut loc_17_repay = BTreeMap::new();
    for m in 1..=12 {
        loc_17_repay.insert(m, 50_000.0);
    }
    scenarios.push(Purchase {
        name: "Rapid 1Y Payoff".to_string(),
        house: House {
            purchase_price: 600_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 1_500.0,
            monthly_hoa: 0.0,
        },
        tools: vec![Tool::Loc(Loc {
            amount: 600_000.0,
            rate: 6.0,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: loc_17_repay,
    });

    // 18. Zero Interest Promotional
    scenarios.push(Purchase {
        name: "Zero Rate Promotional".to_string(),
        house: House {
            purchase_price: 500_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 1_200.0,
            monthly_hoa: 0.0,
        },
        tools: vec![Tool::Loc(Loc {
            amount: 500_000.0,
            rate: 0.0,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 19. Conservative Baseline (50% down, 15yr)
    scenarios.push(Purchase {
        name: "Conservative 50 Down".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.25,
            annual_insurance: 2_400.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 500_000.0,
                rate: 4.25,
            }),
            Tool::Mortgage(Mortgage {
                amount: 500_000.0,
                rate: 5.5,
                term: 15,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    });

    // 20. Complex Irregular Prepayments
    let mut mort_20_repay = BTreeMap::new();
    mort_20_repay.insert(6, 5_000.0);
    mort_20_repay.insert(12, 10_000.0);
    mort_20_repay.insert(18, 15_000.0);
    mort_20_repay.insert(24, 20_000.0);
    mort_20_repay.insert(30, 25_000.0);
    scenarios.push(Purchase {
        name: "Complex Irregular Prepayments".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.25,
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
                rate: 6.5,
                term: 30,
            }),
        ],
        mortgage_repay: mort_20_repay,
        loc_repay: BTreeMap::new(),
    });

    scenarios
}

#[test]
fn test_all_20_scenarios_wasm_parity() {
    let scenarios = sample_scenarios();
    assert_eq!(scenarios.len(), 20, "Must test all 20 scenario types");

    for purchase in scenarios {
        let json_content = serde_json::to_string(&purchase).expect("Serialize purchase");

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
            "Monthly statement length mismatch for {}",
            purchase.name
        );

        assert_eq!(
            native_scenario.yearly_statement.len(),
            wasm_scenario.yearly_statement.len(),
            "Yearly statement length mismatch for {}",
            purchase.name
        );

        assert!(
            (native_scenario.total_statement.total_paid
                - wasm_scenario.total_statement.total_paid)
                .abs()
                < 1e-6,
            "Total paid mismatch for {}",
            purchase.name
        );

        assert!(
            (native_scenario.total_statement.total_interest_paid
                - wasm_scenario.total_statement.total_interest_paid)
                .abs()
                < 1e-6,
            "Total interest mismatch for {}",
            purchase.name
        );

        // 4. Verify Analysis Parity
        assert_eq!(
            native_analysis.payoff_month, wasm_analysis.payoff_month,
            "Payoff month mismatch for {}",
            purchase.name
        );
        assert!(
            (native_analysis.effective_monthly_cost - wasm_analysis.effective_monthly_cost)
                .abs()
                < 1e-6,
            "Effective monthly cost mismatch for {}",
            purchase.name
        );
        assert!(
            (native_analysis.waste_ratio - wasm_analysis.waste_ratio).abs() < 1e-6,
            "Waste ratio mismatch for {}",
            purchase.name
        );
        assert!(
            (native_analysis.tax_savings_ratio - wasm_analysis.tax_savings_ratio).abs() < 1e-6,
            "Tax savings ratio mismatch for {}",
            purchase.name
        );
    }
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

    let pv_a_wasm = wasm_calculate_scenario_pv_from_json(&wasm_scen_a).unwrap();
    let pv_b_wasm = wasm_calculate_scenario_pv_from_json(&wasm_scen_b).unwrap();
    assert!((native_comp.baseline_pv - pv_a_wasm).abs() < 1e-6);
    assert!((native_comp.alternative_pv - pv_b_wasm).abs() < 1e-6);

    assert_eq!(native_comp.months_saved, wasm_comp.months_saved);
    assert!((native_comp.delta_interest_paid - wasm_comp.delta_interest_paid).abs() < 1e-6);
    assert!((native_comp.delta_gross_paid - wasm_comp.delta_gross_paid).abs() < 1e-6);
    assert!((native_comp.delta_pv - wasm_comp.delta_pv).abs() < 1e-6);
    assert_eq!(native_comp.irr.is_some(), wasm_comp.irr.is_some());
}

#[test]
fn test_loc_tool_parity() {
    let purchase = Purchase {
        name: "LOC Parity".to_string(),
        house: House {
            purchase_price: 500_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 1_200.0,
            monthly_hoa: 0.0,
        },
        tools: vec![Tool::Loc(Loc {
            amount: 500_000.0,
            rate: 6.0,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let native = create_scenario(purchase.clone());
    let json_str = serde_json::to_string(&purchase).unwrap();
    let wasm_json = wasm_create_scenario_from_json(&json_str).unwrap();
    let wasm: Scenario = serde_json::from_str(&wasm_json).unwrap();

    assert_eq!(native.monthly_statement.len(), wasm.monthly_statement.len());
    assert!((native.total_statement.total_paid - wasm.total_statement.total_paid).abs() < 1e-6);
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
