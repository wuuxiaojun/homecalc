//! io.rs
//! High-level storage interface

use super::serialize::{get_scenarios_dir_path, load_purchase_from_path, save_purchase_to_path};
use crate::session::state::AppState;
use anyhow::{Context, Result, bail};
use engine::domain::purchase::Purchase;
use engine::service::simulation::create_scenario;
use std::fs;
use std::path::{Path, PathBuf};

/// Ensures the workspace scenarios directory exists and returns its `PathBuf`.
pub fn get_scenarios_path() -> Result<PathBuf> {
    let dir = get_scenarios_dir_path();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create scenarios directory at {:?}", dir))?;
    }
    Ok(dir)
}

/// Sanitizes a string for safe filesystem usage as a filename.
pub fn sanitize_filename(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            ' ' => '_',
            _ => c,
        })
        .collect();
    let trimmed = sanitized.trim_matches('_');
    if trimmed.is_empty() {
        "scenario".to_string()
    } else {
        trimmed.to_lowercase()
    }
}

/// Saves a `Purchase` struct as a formatted `.json` file into the workspace scenarios directory.
pub fn save_purchase(purchase: &Purchase, filename: &str) -> Result<PathBuf> {
    let dir = get_scenarios_path()?;
    let clean_name = sanitize_filename(filename.trim_end_matches(".json"));
    let file_name = format!("{clean_name}.json");
    let target_path = dir.join(file_name);
    save_purchase_to_path(purchase, &target_path)?;
    Ok(target_path)
}

/// Loads a `Purchase` JSON file, computes the `Scenario`, and stores it into `AppState` slot 1 or 2.
pub fn load_scenario(path_or_filename: &Path, slot: u8, state: &mut AppState) -> Result<()> {
    if slot != 1 && slot != 2 {
        bail!("Invalid slot number: {slot}. Slot must be 1 or 2.");
    }

    let resolved_path = if path_or_filename.exists() {
        path_or_filename.to_path_buf()
    } else {
        let dir = get_scenarios_path()?;
        let mut file_name = path_or_filename.to_string_lossy().to_string();
        if !file_name.ends_with(".json") {
            file_name.push_str(".json");
        }
        dir.join(file_name)
    };

    let purchase = load_purchase_from_path(&resolved_path)?;
    let scenario = create_scenario(purchase);

    match slot {
        1 => state.set_slot_1(scenario),
        2 => state.set_slot_2(scenario),
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::state::AppState;
    use engine::domain::house::House;
    use engine::domain::tool::{Cash, Mortgage, Tool};
    use std::collections::BTreeMap;
    use std::fs;

    fn sample_purchase() -> Purchase {
        Purchase {
            name: "Test Purchase IO".to_string(),
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
    fn test_save_purchase_default_dir() {
        let purchase = sample_purchase();
        let saved_default = save_purchase(&purchase, "unit_test_io_scenario.json").unwrap();
        assert!(saved_default.exists());
        let _ = fs::remove_file(&saved_default);
    }

    #[test]
    fn test_load_scenario_slots() {
        let purchase = sample_purchase();
        let temp_dir = std::env::temp_dir().join("homecalc_test_io_slots");
        let test_path = temp_dir.join("test_scenario.json");

        save_purchase_to_path(&purchase, &test_path).unwrap();

        // Load into AppState slot 1
        let mut state = AppState::new();
        load_scenario(&test_path, 1, &mut state).unwrap();
        let scenario1 = state.get_slot_1().unwrap();
        assert_eq!(scenario1.purchase.name, "Test Purchase IO");
        assert_eq!(scenario1.monthly_statement.len(), 361);

        // Load into AppState slot 2
        load_scenario(&test_path, 2, &mut state).unwrap();
        let scenario2 = state.get_slot_2().unwrap();
        assert_eq!(scenario2.purchase.name, "Test Purchase IO");

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_invalid_json_handling() {
        let temp_dir = std::env::temp_dir().join("homecalc_test_invalid_json_io");
        let test_path = temp_dir.join("invalid.json");
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(&test_path, "{ invalid json content }").unwrap();

        let mut state = AppState::new();
        let result = load_scenario(&test_path, 1, &mut state);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.err().unwrap());
        assert!(err_msg.contains("Failed to parse valid JSON Purchase"));

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_missing_file_handling() {
        let missing_path = Path::new("./non_existent_file_12345.json");
        let mut state = AppState::new();
        let result = load_scenario(missing_path, 1, &mut state);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_slot_number() {
        let temp_dir = std::env::temp_dir().join("homecalc_test_invalid_slot_io");
        let test_path = temp_dir.join("test.json");
        save_purchase_to_path(&sample_purchase(), &test_path).unwrap();

        let mut state = AppState::new();
        let result = load_scenario(&test_path, 3, &mut state);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.err().unwrap());
        assert!(err_msg.contains("Invalid slot number"));

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Dream House"), "dream_house");
        assert_eq!(sanitize_filename("House 3/2: Ocean & Lake*"), "house_3_2__ocean_&_lake");
        assert_eq!(sanitize_filename(""), "scenario");
        assert_eq!(sanitize_filename("___"), "scenario");
    }
}
