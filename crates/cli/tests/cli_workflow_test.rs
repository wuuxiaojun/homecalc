//! cli_workflow_test.rs
//! Comprehensive black-box integration tests for the `cli` crate.

use cli::render::analysis::render_analysis;
use cli::render::comparison::render_comparison;
use cli::render::statement::render_statement;
use cli::render::summary::render_summary;
use cli::session::state::AppState;
use cli::storage::io::{load_scenario, sanitize_filename};
use cli::storage::serialize::{load_purchase_from_path, save_purchase_to_path};
use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use engine::service::analysis::analyze_scenario;
use engine::service::comparison::compare_scenarios;
use engine::service::simulation::create_scenario;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn sample_purchases() -> Vec<Purchase> {
    vec![
        // 1. Standard 30yr mortgage
        Purchase {
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
        },
        // 2. 15yr accelerated mortgage
        Purchase {
            name: "15Y Accelerated Mortgage".to_string(),
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
        },
        // 3. Pure LOC
        Purchase {
            name: "Pure LOC Strategy".to_string(),
            house: House {
                purchase_price: 600_000.0,
                annual_property_tax_rate: 1.2,
                annual_insurance: 1_500.0,
                monthly_hoa: 0.0,
            },
            tools: vec![Tool::Loc(Loc {
                amount: 600_000.0,
                rate: 6.75,
            })],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        },
        // 4. All Cash
        Purchase {
            name: "All Cash".to_string(),
            house: House {
                purchase_price: 500_000.0,
                annual_property_tax_rate: 1.0,
                annual_insurance: 1_200.0,
                monthly_hoa: 50.0,
            },
            tools: vec![Tool::Cash(Cash {
                amount: 500_000.0,
                rate: 4.5,
            })],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        },
    ]
}

#[test]
fn test_app_state_slot_lifecycle_and_isolation() {
    let mut state = AppState::new();
    assert!(state.get_slot_1().is_none());
    assert!(state.get_slot_2().is_none());

    let purchases = sample_purchases();
    let s1 = create_scenario(purchases[0].clone());
    let s2 = create_scenario(purchases[1].clone());

    state.set_slot_1(s1);
    state.set_slot_2(s2);

    assert_eq!(state.get_slot_1().unwrap().purchase.name, "Standard 30Y Mortgage");
    assert_eq!(state.get_slot_2().unwrap().purchase.name, "15Y Accelerated Mortgage");

    // Overwrite slot 1 and verify slot 2 remains unaffected
    let s3 = create_scenario(purchases[2].clone());
    state.set_slot_1(s3);
    assert_eq!(state.get_slot_1().unwrap().purchase.name, "Pure LOC Strategy");
    assert_eq!(state.get_slot_2().unwrap().purchase.name, "15Y Accelerated Mortgage");
}

#[test]
fn test_storage_serialization_and_slot_loading() {
    let temp_dir = std::env::temp_dir().join(format!("homecalc_cli_test_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).expect("Create temp dir");

    let purchases = sample_purchases();
    let purchase = &purchases[0];
    let file_path = temp_dir.join("test_save_load.json");

    // 1. Save and verify file existence and content
    save_purchase_to_path(purchase, &file_path).expect("Save purchase to path");
    assert!(file_path.exists());

    // 2. Load from path
    let loaded = load_purchase_from_path(&file_path).expect("Load purchase from path");
    assert_eq!(loaded.name, purchase.name);
    assert_eq!(loaded.house.purchase_price, purchase.house.purchase_price);

    // 3. Load into AppState slots via load_scenario
    let mut state = AppState::new();
    load_scenario(&file_path, 1, &mut state).expect("Load scenario into slot 1");
    load_scenario(&file_path, 2, &mut state).expect("Load scenario into slot 2");

    assert!(state.get_slot_1().is_some());
    assert!(state.get_slot_2().is_some());
    assert_eq!(state.get_slot_1().unwrap().purchase.name, purchase.name);
    assert_eq!(state.get_slot_2().unwrap().purchase.name, purchase.name);

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_storage_error_boundaries() {
    let temp_dir = std::env::temp_dir().join(format!("homecalc_cli_errors_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).expect("Create temp dir");

    let invalid_json_path = temp_dir.join("corrupted.json");
    fs::write(&invalid_json_path, "{ broken_json: [ }").expect("Write corrupted file");

    let mut state = AppState::new();

    // Corrupted file loading
    let res = load_scenario(&invalid_json_path, 1, &mut state);
    assert!(res.is_err());

    // Non-existent file loading
    let missing_path = PathBuf::from("/non/existent/homecalc_scenario_xyz.json");
    let res_missing = load_scenario(&missing_path, 1, &mut state);
    assert!(res_missing.is_err());

    // Invalid slot id (must be 1 or 2)
    let valid_path = temp_dir.join("valid.json");
    save_purchase_to_path(&sample_purchases()[0], &valid_path).expect("Save valid");
    let res_slot0 = load_scenario(&valid_path, 0, &mut state);
    assert!(res_slot0.is_err());
    let res_slot3 = load_scenario(&valid_path, 3, &mut state);
    assert!(res_slot3.is_err());

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_render_pipelines_execute_without_panic() {
    let purchases = sample_purchases();

    for purchase in &purchases {
        let scenario = create_scenario(purchase.clone());

        // 1. Summary render
        render_summary(&scenario);

        // 2. Statements render (monthly and yearly)
        render_statement(&scenario);

        // 3. Analysis metrics render
        let analysis = analyze_scenario(&scenario);
        render_analysis(&analysis);
    }
}

#[test]
fn test_pairwise_comparison_workflow() {
    let purchases = sample_purchases();
    let scen_a = create_scenario(purchases[0].clone());
    let scen_b = create_scenario(purchases[1].clone());

    let comparison = compare_scenarios(&scen_a, &scen_b);

    // 15yr mortgage vs 30yr mortgage saves 180 months (15 years)
    assert_eq!(comparison.months_saved, 180);
    assert!(comparison.delta_interest_paid < 0.0); // 15yr pays significantly less interest
    assert!(comparison.baseline_gross_paid > 0.0);
    assert!(comparison.alternative_gross_paid > 0.0);

    // Ensure comparison render executes cleanly
    render_comparison(&comparison);
}

#[test]
fn test_sanitize_filename_utility() {
    assert_eq!(sanitize_filename("My 30-Year Dream Home!"), "my_30-year_dream_home!");
    assert_eq!(sanitize_filename("Special/Path\\Test:1*?"), "special_path_test_1");
    assert_eq!(sanitize_filename("   "), "scenario");
}

