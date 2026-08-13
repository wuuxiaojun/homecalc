//! storage.rs
//! JSON persistence module for Homecalc CLI.

use anyhow::{bail, Context, Result};
use engine::domain::purchase::Purchase;
use engine::service::simulation::create_scenario;
use std::fs;
use std::path::{Path, PathBuf};

use crate::state::AppState;

/// Default relative directory name for storing scenario JSON files.
pub const DEFAULT_SCENARIOS_DIR: &str = "scenarios";

/// Finds the workspace root directory containing `Cargo.toml` with `[workspace]`
/// and returns the path to `<workspace_root>/scenarios`.
pub fn get_workspace_scenarios_dir() -> PathBuf {
    // 1. Check CARGO_MANIFEST_DIR (available during build/test)
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = PathBuf::from(manifest_dir);
        for ancestor in manifest_path.ancestors() {
            let cargo_toml = ancestor.join("Cargo.toml");
            if cargo_toml.exists() {
                if let Ok(content) = fs::read_to_string(&cargo_toml) {
                    if content.contains("[workspace]") {
                        return ancestor.join(DEFAULT_SCENARIOS_DIR);
                    }
                }
            }
        }
    }

    // 2. Walk up from current working directory
    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            let cargo_toml = ancestor.join("Cargo.toml");
            if cargo_toml.exists() {
                if let Ok(content) = fs::read_to_string(&cargo_toml) {
                    if content.contains("[workspace]") {
                        return ancestor.join(DEFAULT_SCENARIOS_DIR);
                    }
                }
            }
        }
    }

    // Fallback to relative ./scenarios
    PathBuf::from(DEFAULT_SCENARIOS_DIR)
}

/// Ensures that the workspace `scenarios/` directory exists (`homecalc/scenarios`).
pub fn ensure_scenarios_dir() -> Result<PathBuf> {
    let dir = get_workspace_scenarios_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create scenarios directory at {:?}", dir))?;
    }
    Ok(dir)
}


/// Saves a `Purchase` struct as a formatted `.json` file into workspace `./scenarios/`.
///
/// Automatically appends `.json` extension if not present.
pub fn save_purchase(purchase: &Purchase, filename: &str) -> Result<PathBuf> {
    let dir = ensure_scenarios_dir()?;
    let mut file_name = filename.to_string();
    if !file_name.ends_with(".json") {
        file_name.push_str(".json");
    }
    let target_path = dir.join(file_name);
    save_purchase_to_path(purchase, &target_path)?;
    Ok(target_path)
}

/// Saves a `Purchase` struct as a formatted `.json` file to a specific file path.
pub fn save_purchase_to_path(purchase: &Purchase, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create parent directory at {:?}", parent))?;
        }
    }

    let json_str = serde_json::to_string_pretty(purchase)
        .with_context(|| format!("Failed to serialize purchase {:?} to JSON", purchase.name))?;

    fs::write(path, json_str)
        .with_context(|| format!("Failed to write purchase JSON file to {:?}", path))?;

    Ok(())
}

/// Loads a `Purchase` JSON file from disk into a `Purchase` struct.
pub fn load_purchase_from_path(path: &Path) -> Result<Purchase> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read scenario file at {:?}", path))?;

    let purchase: Purchase = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse valid JSON Purchase from file at {:?}", path))?;

    Ok(purchase)
}

/// Loads a `Purchase` JSON file, evaluates it into a `Scenario`, and stores it into `AppState` slot 1 or 2.
///
/// `path_or_filename` can be a path or a filename relative to workspace `./scenarios/`.
pub fn load_scenario_into_slot(
    path_or_filename: &Path,
    slot: u8,
    state: &mut AppState,
) -> Result<()> {
    if slot != 1 && slot != 2 {
        bail!("Invalid slot number: {slot}. Slot must be 1 or 2.");
    }

    let resolved_path = if path_or_filename.exists() {
        path_or_filename.to_path_buf()
    } else {
        let dir = ensure_scenarios_dir()?;
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
    use engine::domain::house::House;
    use engine::domain::tool::{Cash, Mortgage, Tool};
    use std::collections::BTreeMap;
    use std::fs;

    fn sample_purchase() -> Purchase {
        Purchase {
            name: "Test Purchase".to_string(),
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
    fn test_ensure_scenarios_dir() {
        let dir = ensure_scenarios_dir().unwrap();
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    #[test]
    fn test_save_and_load_purchase() {
        let purchase = sample_purchase();
        let temp_dir = std::env::temp_dir().join("homecalc_test_storage");
        let test_path = temp_dir.join("test_scenario.json");

        // Save
        save_purchase_to_path(&purchase, &test_path).unwrap();
        assert!(test_path.exists());

        // Test save_purchase default directory helper
        let saved_default = save_purchase(&purchase, "unit_test_scenario.json").unwrap();
        assert!(saved_default.exists());
        let _ = fs::remove_file(&saved_default);

        // Verify JSON content is formatted pretty
        let raw_json = fs::read_to_string(&test_path).unwrap();
        assert!(raw_json.contains("\"name\": \"Test Purchase\""));

        // Load into AppState slot 1
        let mut state = AppState::new();
        load_scenario_into_slot(&test_path, 1, &mut state).unwrap();

        let scenario1 = state.get_slot_1().unwrap();
        assert_eq!(scenario1.purchase.name, "Test Purchase");
        assert_eq!(scenario1.monthly_statement.len(), 360);

        // Load into AppState slot 2
        load_scenario_into_slot(&test_path, 2, &mut state).unwrap();
        let scenario2 = state.get_slot_2().unwrap();
        assert_eq!(scenario2.purchase.name, "Test Purchase");

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_invalid_json_handling() {
        let temp_dir = std::env::temp_dir().join("homecalc_test_invalid_json");
        let test_path = temp_dir.join("invalid.json");
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(&test_path, "{ invalid json content }").unwrap();

        let mut state = AppState::new();
        let result = load_scenario_into_slot(&test_path, 1, &mut state);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.err().unwrap());
        assert!(err_msg.contains("Failed to parse valid JSON Purchase"));

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_missing_file_handling() {
        let missing_path = Path::new("./non_existent_file_12345.json");
        let mut state = AppState::new();
        let result = load_scenario_into_slot(missing_path, 1, &mut state);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_slot_number() {
        let temp_dir = std::env::temp_dir().join("homecalc_test_invalid_slot");
        let test_path = temp_dir.join("test.json");
        save_purchase_to_path(&sample_purchase(), &test_path).unwrap();

        let mut state = AppState::new();
        let result = load_scenario_into_slot(&test_path, 3, &mut state);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.err().unwrap());
        assert!(err_msg.contains("Invalid slot number"));

        let _ = fs::remove_dir_all(temp_dir);
    }
}
