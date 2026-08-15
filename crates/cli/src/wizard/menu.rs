//! menu.rs
//! Multi-tier interactive menu flow for Homecalc CLI.

use anyhow::Result;
use engine::service::analysis::analyze_scenario;
use engine::service::comparison::compare_scenarios;
use engine::service::simulation::create_scenario;
use inquire::{InquireError, Select, Text};
use std::fs;
use std::io::{Write, stdout};
use std::path::PathBuf;

use crate::render::analysis::render_analysis;
use crate::render::comparison::render_comparison;
use crate::render::statement::render_statement;
use crate::render::summary::render_summary;
use crate::session::state::AppState;
use crate::storage::io::{get_scenarios_path, load_scenario, save_purchase};
use crate::wizard::input::prompt_create_purchase;

/// Clears the terminal screen, scrollback buffer, and resets cursor position to top-left (1,1).
pub fn clear_screen() {
    print!("\x1B[3J\x1B[2J\x1B[1;1H");
    let _ = stdout().flush();
}

/// Helper function to browse and select a `.json` scenario file from `get_scenarios_path()`,
/// or allow entering a file path manually.
pub fn prompt_select_scenario_file() -> Result<Option<PathBuf>> {
    let dir = get_scenarios_path()?;
    let mut json_files = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Some(name) = path.file_name() {
                    json_files.push(name.to_string_lossy().to_string());
                }
            }
        }
    }

    if json_files.is_empty() {
        println!(
            "\n[!] No `.json` scenario files found in directory {:?}",
            dir
        );
        let manual_res = Text::new("Enter scenario file path or filename manually:").prompt();
        let manual = match manual_res {
            Ok(val) => val,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                return Ok(None);
            }
            Err(e) => return Err(e.into()),
        };

        let path = PathBuf::from(&manual);
        if path.is_absolute() || path.exists() {
            Ok(Some(path))
        } else {
            let filename = if manual.ends_with(".json") {
                manual
            } else {
                format!("{}.json", manual)
            };
            Ok(Some(dir.join(filename)))
        }
    } else {
        json_files.sort();
        let choice_res = Select::new("Select scenario file to load:", json_files).prompt();
        let choice = match choice_res {
            Ok(val) => val,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                return Ok(None);
            }
            Err(e) => return Err(e.into()),
        };
        let path = dir.join(choice);
        Ok(Some(path))
    }
}

/// 1. Main Menu (`run_main_menu(state: &mut AppState)`):
/// Options:
/// - 1. Create Scenario
/// - 2. Load Scenario
/// - 3. Compare Scenarios
/// - 4. Exit
pub fn run_main_menu(state: &mut AppState) -> Result<()> {
    loop {
        clear_screen();
        println!(
            "================================================================================"
        );
        println!(" WELCOME TO HOMECALC SCENARIO ENGINE");
        println!(
            "================================================================================"
        );

        let choices = vec![
            "1. Create Scenario",
            "2. Load Scenario",
            "3. Compare Scenarios",
            "4. Exit",
        ];

        let selection_res = Select::new("Main Menu Choice:", choices).prompt();
        let selection = match selection_res {
            Ok(val) => val,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                println!("\nExiting Homecalc. Goodbye!\n");
                break;
            }
            Err(e) => return Err(e.into()),
        };

        clear_screen();

        match selection {
            c if c.starts_with("1") => {
                let purchase_res = prompt_create_purchase();
                let purchase = match purchase_res {
                    Ok(p) => p,
                    Err(e) => {
                        if let Some(inq_err) = e.downcast_ref::<InquireError>() {
                            if matches!(
                                inq_err,
                                InquireError::OperationCanceled
                                    | InquireError::OperationInterrupted
                            ) {
                                continue;
                            }
                        }
                        return Err(e);
                    }
                };
                let scenario = create_scenario(purchase);
                state.set_slot_1(scenario);
                run_scenario_menu(state)?;
            }
            c if c.starts_with("2") => {
                if let Some(file_path) = prompt_select_scenario_file()? {
                    load_scenario(&file_path, 1, state)?;
                    run_scenario_menu(state)?;
                }
            }
            c if c.starts_with("3") => {
                println!("\n Select the file for baseline scenario: ");
                let file1_opt = prompt_select_scenario_file()?;
                let Some(file1) = file1_opt else { continue };
                load_scenario(&file1, 1, state)?;

                println!("\n Select the file for alternative scenario: ");
                let file2_opt = prompt_select_scenario_file()?;
                let Some(file2) = file2_opt else { continue };
                load_scenario(&file2, 2, state)?;

                clear_screen();
                if let (Some(s1), Some(s2)) = (state.get_slot_1(), state.get_slot_2()) {
                    let comparison = compare_scenarios(s1, s2);
                    render_comparison(&comparison);
                    run_comparison_menu()?;
                } else {
                    println!(
                        "\n[!] Error: Unable to perform comparison because one or both slots are empty."
                    );
                }
            }
            c if c.starts_with("4") => {
                clear_screen();
                println!("\nExiting Homecalc. Goodbye!\n");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/// 2. Scenario Menu (`run_scenario_menu(state: &mut AppState)`):
/// Context of active slot 1:
/// - 1. Save Scenario
/// - 2. View Statement
/// - 3. View Analysis
/// - 4. Compare Scenario
/// - 5. Back
pub fn run_scenario_menu(state: &mut AppState) -> Result<()> {
    loop {
        clear_screen();
        let Some(s) = state.get_slot_1() else {
            println!("\n[!] No active scenario loaded in Slot 1.");
            break;
        };

        render_summary(s);

        let choices = vec![
            "1. Save Scenario",
            "2. View Statement",
            "3. View Analysis",
            "4. Compare Scenario",
            "5. Back",
        ];

        let selection_res = Select::new("Scenario Menu Choice:", choices).prompt();
        let selection = match selection_res {
            Ok(val) => val,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                break;
            }
            Err(e) => return Err(e.into()),
        };

        match selection {
            c if c.starts_with("1") => {
                let default_filename =
                    format!("{}.json", s.purchase.name.to_lowercase().replace(' ', "_"));
                let filename_res = Text::new("Enter filename to save (e.g. scenario.json):")
                    .with_default(&default_filename)
                    .prompt();

                let filename = match filename_res {
                    Ok(val) => val,
                    Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                };

                let saved_path = save_purchase(&s.purchase, &filename)?;
                println!("\n[✓] Scenario successfully saved to {:?}", saved_path);
            }
            c if c.starts_with("2") => {
                clear_screen();
                render_statement(s);
                run_scenario_sub_menu()?;
            }
            c if c.starts_with("3") => {
                clear_screen();
                let analysis = analyze_scenario(s);
                render_analysis(&analysis);
                run_scenario_sub_menu()?;
            }
            c if c.starts_with("4") => {
                println!("\n[+] Select second scenario file to load into Slot 2 for comparison:");
                if let Some(file2) = prompt_select_scenario_file()? {
                    load_scenario(&file2, 2, state)?;

                    clear_screen();
                    if let (Some(s1), Some(s2)) = (state.get_slot_1(), state.get_slot_2()) {
                        let comparison = compare_scenarios(s1, s2);
                        render_comparison(&comparison);
                        run_comparison_menu()?;
                    }
                }
            }
            c if c.starts_with("5") => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/// 3. Scenario Sub Menu (`run_scenario_sub_menu()`):
/// Options:
/// - 1. Back
pub fn run_scenario_sub_menu() -> Result<()> {
    loop {
        let choices = vec!["1. Back"];
        let selection_res = Select::new("Sub Menu Choice:", choices).prompt();
        let selection = match selection_res {
            Ok(val) => val,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => break,
            Err(e) => return Err(e.into()),
        };
        if selection.starts_with("1") {
            break;
        }
    }
    Ok(())
}

/// 4. Comparison Sub-Menu (`run_comparison_menu()`):
/// Options:
/// - 1. Back
pub fn run_comparison_menu() -> Result<()> {
    loop {
        let choices = vec!["1. Back"];
        let selection_res = Select::new("Comparison Sub-Menu Choice:", choices).prompt();
        let selection = match selection_res {
            Ok(val) => val,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => break,
            Err(e) => return Err(e.into()),
        };
        if selection.starts_with("1") {
            break;
        }
    }
    Ok(())
}
