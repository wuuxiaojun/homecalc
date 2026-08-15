//! serialize.rs
//! Auxiliary storage functions

use anyhow::{Context, Result};
use engine::domain::purchase::Purchase;
use std::fs;
use std::path::{Path, PathBuf};

use std::sync::OnceLock;

pub const DEFAULT_SCENARIOS_DIR: &str = "scenarios";
static SCENARIOS_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Finds the workspace root directory containing `Cargo.toml` with `[workspace]`.
pub fn get_scenarios_dir_path() -> PathBuf {
    SCENARIOS_DIR
        .get_or_init(|| {
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

            PathBuf::from(DEFAULT_SCENARIOS_DIR)
        })
        .clone()
}

/// Serializes and writes a `Purchase` struct as pretty-printed JSON to a file path.
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

/// Reads and deserializes a JSON file into a `Purchase` struct.
pub fn load_purchase_from_path(path: &Path) -> Result<Purchase> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read scenario file at {:?}", path))?;

    let purchase: Purchase = serde_json::from_str(&content).with_context(|| {
        format!(
            "Failed to parse valid JSON Purchase from file at {:?}",
            path
        )
    })?;

    Ok(purchase)
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::domain::house::House;
    use engine::domain::tool::{Cash, Mortgage, Tool};
    use std::collections::BTreeMap;

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
    fn test_save_and_load_purchase_to_path() {
        let purchase = sample_purchase();
        let temp_dir = std::env::temp_dir().join("homecalc_test_serialize");
        let test_path = temp_dir.join("test_scenario.json");

        // Save
        save_purchase_to_path(&purchase, &test_path).unwrap();
        assert!(test_path.exists());

        // Verify JSON content is formatted pretty
        let raw_json = fs::read_to_string(&test_path).unwrap();
        assert!(raw_json.contains("\"name\": \"Test Purchase\""));

        // Load
        let loaded = load_purchase_from_path(&test_path).unwrap();
        assert_eq!(loaded.name, "Test Purchase");
        assert_eq!(loaded.house.purchase_price, 1_000_000.0);

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }
}
