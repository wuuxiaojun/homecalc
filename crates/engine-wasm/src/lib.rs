//! lib.rs
//! WebAssembly bindings for Homecalc core calculation engine.

use engine::domain::purchase::Purchase;
use engine::domain::scenario::Scenario;
use engine::service::analysis::{ScenarioAnalysis, analyze_scenario};
use engine::service::comparison::{
    ScenarioComparison, calculate_scenario_pv, compare_scenarios, solve_irr_newton_raphson,
};
use engine::service::simulation::create_scenario;
use wasm_bindgen::prelude::*;

/// Returns the engine version and build info.
#[wasm_bindgen]
pub fn wasm_engine_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Returns the default starting cash limit constant from engine configuration.
#[wasm_bindgen]
pub fn wasm_default_starting_cash() -> f64 {
    engine::config::constant::DEFAULT_STARTING_CASH
}

/// Simulates a purchase scenario from a JavaScript `Purchase` object and returns the full `Scenario`.
#[wasm_bindgen]
pub fn wasm_create_scenario(val: JsValue) -> Result<JsValue, JsValue> {
    let purchase: Purchase = serde_wasm_bindgen::from_value(val)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse Purchase object: {}", e)))?;
    let scenario = create_scenario(purchase);
    serde_wasm_bindgen::to_value(&scenario)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize Scenario object: {}", e)))
}

/// Computes single scenario analysis metrics from a JavaScript `Scenario` object.
#[wasm_bindgen]
pub fn wasm_analyze_scenario(val: JsValue) -> Result<JsValue, JsValue> {
    let scenario: Scenario = serde_wasm_bindgen::from_value(val)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse Scenario object: {}", e)))?;
    let analysis: ScenarioAnalysis = analyze_scenario(&scenario);
    serde_wasm_bindgen::to_value(&analysis).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to serialize ScenarioAnalysis object: {}",
            e
        ))
    })
}

/// Computes comparative metrics between baseline (A) and alternative (B) scenarios.
#[wasm_bindgen]
pub fn wasm_compare_scenarios(val_a: JsValue, val_b: JsValue) -> Result<JsValue, JsValue> {
    let scenario_a: Scenario = serde_wasm_bindgen::from_value(val_a)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse baseline Scenario: {}", e)))?;
    let scenario_b: Scenario = serde_wasm_bindgen::from_value(val_b)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse alternative Scenario: {}", e)))?;
    let comparison: ScenarioComparison = compare_scenarios(&scenario_a, &scenario_b);
    serde_wasm_bindgen::to_value(&comparison).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to serialize ScenarioComparison object: {}",
            e
        ))
    })
}

/// Simulates a purchase scenario from a JSON string representation of `Purchase` and returns the JSON string `Scenario`.
#[wasm_bindgen]
pub fn wasm_create_scenario_from_json(purchase_json: &str) -> Result<String, String> {
    let purchase: Purchase = serde_json::from_str(purchase_json)
        .map_err(|e| format!("Failed to parse Purchase JSON: {}", e))?;
    let scenario = create_scenario(purchase);
    serde_json::to_string(&scenario)
        .map_err(|e| format!("Failed to serialize Scenario to JSON: {}", e))
}

/// Computes single scenario analysis metrics from a JSON string `Scenario`.
#[wasm_bindgen]
pub fn wasm_analyze_scenario_from_json(scenario_json: &str) -> Result<String, String> {
    let scenario: Scenario = serde_json::from_str(scenario_json)
        .map_err(|e| format!("Failed to parse Scenario JSON: {}", e))?;
    let analysis: ScenarioAnalysis = analyze_scenario(&scenario);
    serde_json::to_string(&analysis)
        .map_err(|e| format!("Failed to serialize ScenarioAnalysis to JSON: {}", e))
}

/// Computes comparative metrics between baseline (A) and alternative (B) JSON string scenarios.
#[wasm_bindgen]
pub fn wasm_compare_scenarios_from_json(
    scenario_a_json: &str,
    scenario_b_json: &str,
) -> Result<String, String> {
    let scenario_a: Scenario = serde_json::from_str(scenario_a_json)
        .map_err(|e| format!("Failed to parse baseline Scenario JSON: {}", e))?;
    let scenario_b: Scenario = serde_json::from_str(scenario_b_json)
        .map_err(|e| format!("Failed to parse alternative Scenario JSON: {}", e))?;
    let comparison: ScenarioComparison = compare_scenarios(&scenario_a, &scenario_b);
    serde_json::to_string(&comparison)
        .map_err(|e| format!("Failed to serialize ScenarioComparison to JSON: {}", e))
}

/// Calculates the discounted Present Value (PV) of a scenario's cash outflows from a JSON string `Scenario`.
#[wasm_bindgen]
pub fn wasm_calculate_scenario_pv_from_json(scenario_json: &str) -> Result<f64, String> {
    let scenario: Scenario = serde_json::from_str(scenario_json)
        .map_err(|e| format!("Failed to parse Scenario JSON: {}", e))?;
    Ok(calculate_scenario_pv(&scenario))
}

/// Solves Internal Rate of Return (IRR) from a JSON array of monthly cash flows.
#[wasm_bindgen]
pub fn wasm_solve_irr(cash_flows_json: &str) -> Result<Option<f64>, String> {
    let cash_flows: Vec<f64> = serde_json::from_str(cash_flows_json)
        .map_err(|e| format!("Failed to parse cash flows JSON array: {}", e))?;
    Ok(solve_irr_newton_raphson(&cash_flows))
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::domain::house::House;
    use engine::domain::tool::{Cash, Mortgage, Tool};
    use std::collections::BTreeMap;

    fn sample_purchase() -> Purchase {
        Purchase {
            name: "WASM Test Purchase".to_string(),
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
        }
    }

    #[test]
    fn test_wasm_create_scenario_from_json() {
        let purchase = sample_purchase();
        let json_str = serde_json::to_string(&purchase).unwrap();
        let scenario_json = wasm_create_scenario_from_json(&json_str).unwrap();
        let scenario: Scenario = serde_json::from_str(&scenario_json).unwrap();

        assert_eq!(scenario.purchase.name, "WASM Test Purchase");
        assert_eq!(scenario.monthly_statement.len(), 361);
        assert_eq!(scenario.yearly_statement.len(), 30);
        assert!(scenario.total_statement.total_paid > 0.0);
    }

    #[test]
    fn test_wasm_analyze_scenario_from_json() {
        let purchase = sample_purchase();
        let json_str = serde_json::to_string(&purchase).unwrap();
        let scenario_json = wasm_create_scenario_from_json(&json_str).unwrap();
        let analysis_json = wasm_analyze_scenario_from_json(&scenario_json).unwrap();
        let analysis: ScenarioAnalysis = serde_json::from_str(&analysis_json).unwrap();

        assert_eq!(analysis.payoff_month, 360);
        assert!(analysis.effective_monthly_cost > 0.0);
        assert!(analysis.waste_ratio > 0.0);
    }

    #[test]
    fn test_wasm_compare_scenarios_from_json() {
        let purchase_a = sample_purchase();
        let mut purchase_b = sample_purchase();
        purchase_b.name = "Accelerated".to_string();
        purchase_b.mortgage_repay.insert(12, 100_000.0);

        let json_a = serde_json::to_string(&purchase_a).unwrap();
        let json_b = serde_json::to_string(&purchase_b).unwrap();

        let scen_a_json = wasm_create_scenario_from_json(&json_a).unwrap();
        let scen_b_json = wasm_create_scenario_from_json(&json_b).unwrap();

        let comparison_json = wasm_compare_scenarios_from_json(&scen_a_json, &scen_b_json).unwrap();
        let comparison: ScenarioComparison = serde_json::from_str(&comparison_json).unwrap();

        assert!(comparison.months_saved > 0);
        assert!(comparison.delta_interest_paid < 0.0);
    }

    #[test]
    fn test_wasm_solve_irr() {
        let flows = serde_json::to_string(&vec![-100.0, 110.0]).unwrap();
        let irr = wasm_solve_irr(&flows).unwrap();
        assert!(irr.is_some());
        let val = irr.unwrap();
        let expected = (1.10_f64).powi(12) - 1.0;
        assert!((val - expected).abs() < 1e-4);
    }

    #[test]
    fn test_wasm_default_starting_cash() {
        assert_eq!(wasm_default_starting_cash(), 1_000_000.00);
    }

    #[test]
    fn test_wasm_engine_version() {
        assert!(!wasm_engine_version().is_empty());
    }
}
