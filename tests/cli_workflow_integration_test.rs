//! cli_workflow_integration_test.rs
//! End-to-end integration tests verifying AppState slot management,
//! serialization roundtripping, error boundaries, and full render pipeline formatting.

use cli::render::analysis::render_analysis;
use cli::render::comparison::render_comparison;
use cli::render::statement::render_statement;
use cli::render::summary::render_summary;
use cli::session::state::AppState;
use cli::storage::io::{get_scenarios_path, load_scenario, sanitize_filename, save_purchase};
use cli::storage::serialize::{load_purchase_from_path, save_purchase_to_path};
use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::tool::{Cash, Tool};
use engine::service::analysis::analyze_scenario;
use engine::service::comparison::compare_scenarios;
use engine::service::simulation::create_scenario;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[test]
fn test_app_state_slot_workflow_and_isolation() {
    let dir = get_scenarios_path().expect("Scenarios directory exists");
    let file1 = dir.join("standard_30yr_conventional.json");
    let file2 = dir.join("all_cash_starter.json");

    assert!(file1.exists(), "{:?} must exist", file1);
    assert!(file2.exists(), "{:?} must exist", file2);

    let mut state = AppState::new();
    assert!(state.get_slot_1().is_none());
    assert!(state.get_slot_2().is_none());

    // Load file1 into Slot 1
    load_scenario(&file1, 1, &mut state).expect("Load slot 1 succeeds");
    assert!(state.get_slot_1().is_some());
    assert!(state.get_slot_2().is_none());
    assert_eq!(
        state.get_slot_1().unwrap().purchase.name,
        "Standard 30yr Conventional"
    );

    // Load file2 into Slot 2
    load_scenario(&file2, 2, &mut state).expect("Load slot 2 succeeds");
    assert!(state.get_slot_1().is_some());
    assert!(state.get_slot_2().is_some());
    assert_eq!(
        state.get_slot_2().unwrap().purchase.name,
        "All Cash Starter"
    );

    // Verify Slot 1 remained untouched
    assert_eq!(
        state.get_slot_1().unwrap().purchase.name,
        "Standard 30yr Conventional"
    );

    // Overwrite Slot 1 with file2
    load_scenario(&file2, 1, &mut state).expect("Overwrite slot 1 succeeds");
    assert_eq!(
        state.get_slot_1().unwrap().purchase.name,
        "All Cash Starter"
    );
    assert_eq!(
        state.get_slot_2().unwrap().purchase.name,
        "All Cash Starter"
    );
}

#[test]
fn test_serialization_roundtrip_all_scenarios() {
    let dir = get_scenarios_path().expect("Scenarios directory exists");
    let temp_dir = std::env::temp_dir().join("homecalc_cli_roundtrip_test");
    fs::create_dir_all(&temp_dir).expect("Create temp dir");

    let mut count = 0;
    for entry in fs::read_dir(&dir).expect("Read scenarios dir") {
        let entry = entry.expect("Valid entry");
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            let original = load_purchase_from_path(&path).expect("Load purchase");
            let temp_target = temp_dir.join(path.file_name().unwrap());

            // Save to temp
            save_purchase_to_path(&original, &temp_target).expect("Save purchase to path");
            assert!(temp_target.exists());

            // Reload from temp
            let reloaded = load_purchase_from_path(&temp_target).expect("Reload purchase");

            // Deep equality check
            assert_eq!(
                original,
                reloaded,
                "Purchase struct mismatch after roundtrip serialization for {:?}",
                path.file_name()
            );

            // Verify simulation scenario results match exactly
            let s_orig = create_scenario(original);
            let s_reloaded = create_scenario(reloaded);
            assert_eq!(
                s_orig.monthly_statement.len(),
                s_reloaded.monthly_statement.len()
            );
            assert_eq!(
                s_orig.yearly_statement.len(),
                s_reloaded.yearly_statement.len()
            );
            assert_eq!(
                s_orig.total_statement.total_paid,
                s_reloaded.total_statement.total_paid
            );
            assert_eq!(
                s_orig.total_statement.total_interest_paid,
                s_reloaded.total_statement.total_interest_paid
            );

            count += 1;
        }
    }

    assert!(
        count >= 16,
        "Must roundtrip at least 16 scenarios, did {}",
        count
    );
    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_render_pipeline_no_panic_all_scenarios() {
    let dir = get_scenarios_path().expect("Scenarios directory exists");
    let mut scenarios = Vec::new();

    for entry in fs::read_dir(&dir).expect("Read scenarios dir") {
        let entry = entry.expect("Valid entry");
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            let purchase = load_purchase_from_path(&path).expect("Load purchase");
            let scenario = create_scenario(purchase);
            scenarios.push(scenario);
        }
    }

    // 1. Single scenario renders
    for s in &scenarios {
        render_summary(s);
        render_statement(s);
        let analysis = analyze_scenario(s);
        render_analysis(&analysis);
    }

    // 2. Pairwise comparison renders
    for (i, s1) in scenarios.iter().enumerate() {
        for (j, s2) in scenarios.iter().enumerate() {
            if i != j {
                let cmp = compare_scenarios(s1, s2);
                render_comparison(&cmp);
            }
        }
    }
}

#[test]
fn test_storage_and_slot_error_boundaries() {
    let mut state = AppState::new();

    // Invalid slot error
    let dummy_path = Path::new("dummy.json");
    let err_slot_0 = load_scenario(dummy_path, 0, &mut state);
    assert!(err_slot_0.is_err());
    assert!(format!("{:?}", err_slot_0.err().unwrap()).contains("Invalid slot number"));

    let err_slot_3 = load_scenario(dummy_path, 3, &mut state);
    assert!(err_slot_3.is_err());
    assert!(format!("{:?}", err_slot_3.err().unwrap()).contains("Invalid slot number"));

    // Missing file error
    let missing_path = Path::new("/non/existent/dir/missing_file.json");
    let err_missing = load_scenario(missing_path, 1, &mut state);
    assert!(err_missing.is_err());

    // Sanitize filename edge cases
    assert_eq!(sanitize_filename(""), "scenario");
    assert_eq!(sanitize_filename("///:::***???"), "scenario");
    assert_eq!(
        sanitize_filename("My Dream House #1!"),
        "my_dream_house_#1!"
    );
    assert_eq!(sanitize_filename("__test__name__"), "test__name");

    // Save purchase with special characters
    let sample = Purchase {
        name: "Special Name Test".to_string(),
        house: House {
            purchase_price: 500_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 1_000.0,
            monthly_hoa: 0.0,
        },
        tools: vec![Tool::Cash(Cash {
            amount: 500_000.0,
            rate: 4.0,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };
    let saved_path = save_purchase(&sample, "Sanitize: Special * Name?").expect("Save purchase");
    assert!(saved_path.exists());
    let _ = fs::remove_file(&saved_path);
}
