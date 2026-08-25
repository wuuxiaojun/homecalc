//! wasm_api_test.rs
//! Boundary, error handling, and API integration tests for `engine-wasm`.

use engine::config::constant::DEFAULT_STARTING_CASH;
use engine_wasm::{
    wasm_analyze_scenario_from_json, wasm_calculate_scenario_pv_from_json,
    wasm_compare_scenarios_from_json, wasm_create_scenario_from_json, wasm_default_starting_cash,
    wasm_engine_version, wasm_solve_irr,
};

#[test]
fn test_wasm_metadata_exports() {
    let version = wasm_engine_version();
    assert_eq!(version, env!("CARGO_PKG_VERSION"));

    let starting_cash = wasm_default_starting_cash();
    assert_eq!(starting_cash, DEFAULT_STARTING_CASH);
}

#[test]
fn test_wasm_malformed_json_error_handling() {
    let malformed = "not valid json { [";

    // 1. Create scenario error handling
    let res_create = wasm_create_scenario_from_json(malformed);
    assert!(res_create.is_err());
    assert!(res_create.unwrap_err().contains("Failed to parse Purchase JSON"));

    // 2. Analyze scenario error handling
    let res_analyze = wasm_analyze_scenario_from_json(malformed);
    assert!(res_analyze.is_err());
    assert!(res_analyze.unwrap_err().contains("Failed to parse Scenario JSON"));

    // 3. Compare scenarios error handling (first argument bad)
    let res_comp_a = wasm_compare_scenarios_from_json(malformed, "{}");
    assert!(res_comp_a.is_err());
    assert!(res_comp_a.unwrap_err().contains("Failed to parse baseline Scenario JSON"));

    // 4. Calculate PV error handling
    let res_pv = wasm_calculate_scenario_pv_from_json(malformed);
    assert!(res_pv.is_err());
    assert!(res_pv.unwrap_err().contains("Failed to parse Scenario JSON"));

    // 5. Solve IRR error handling
    let res_irr = wasm_solve_irr(malformed);
    assert!(res_irr.is_err());
    assert!(res_irr.unwrap_err().contains("Failed to parse cash flows JSON"));
}

#[test]
fn test_irr_numerical_solver_scenarios() {
    // 1-month stream: -100 initial outflow, +110 in month 1 -> 10% monthly -> (1.10)^12 - 1 annualized
    let flows_json = serde_json::to_string(&vec![-100.0, 110.0]).unwrap();
    let irr = wasm_solve_irr(&flows_json).unwrap();
    assert!(irr.is_some());
    let rate = irr.unwrap();
    let expected_annual = (1.10f64).powi(12) - 1.0;
    assert!((rate - expected_annual).abs() < 1e-4);

    // Flat cash flows with zero return: -100, +100 -> 0.0%
    let flat_json = serde_json::to_string(&vec![-100.0, 100.0]).unwrap();
    let flat_irr = wasm_solve_irr(&flat_json).unwrap();
    assert!(flat_irr.is_some());
    assert!(flat_irr.unwrap().abs() < 1e-4);

    // Empty flows -> None
    let empty_json = serde_json::to_string(&Vec::<f64>::new()).unwrap();
    let empty_irr = wasm_solve_irr(&empty_json).unwrap();
    assert_eq!(empty_irr, None);

    // All positive flows (no outflow / investment) -> None
    let pos_json = serde_json::to_string(&vec![100.0, 100.0, 100.0]).unwrap();
    let pos_irr = wasm_solve_irr(&pos_json).unwrap();
    assert_eq!(pos_irr, None);
}

