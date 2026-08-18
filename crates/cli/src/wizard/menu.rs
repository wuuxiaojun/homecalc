//! menu.rs
//! Multi-tier interactive menu flow for Homecalc CLI.

use anyhow::Result;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use engine::service::analysis::analyze_scenario;
use engine::service::comparison::compare_scenarios;
use engine::service::simulation::create_scenario;
use inquire::{InquireError, Text};
use std::fs;
use std::io::{Write, stdout};
use std::path::PathBuf;

use crate::render::analysis::render_analysis;
use crate::render::comparison::render_comparison;
use crate::render::statement::render_statement;
use crate::render::summary::render_summary;
use crate::session::state::AppState;
use crate::storage::io::{get_scenarios_path, load_scenario, sanitize_filename, save_purchase};
use crate::wizard::input::prompt_create_purchase;

/// Clears the terminal screen, scrollback buffer, and resets cursor position to top-left (1,1).
pub fn clear_screen() {
    print!("\x1B[3J\x1B[2J\x1B[1;1H");
    let _ = stdout().flush();
}

/// Custom interactive menu selector with:
/// - Instant number key selection (typing '1' immediately selects option 1)
/// - Ignores invalid numbers (e.g. if 4 options, pressing '5' does nothing)
/// - Up / Down / 'k' / 'j' arrow navigation
/// - Enter / Space to confirm highlighted option
/// - Esc / 'q' / Ctrl+C to cancel / go back
/// - Zero search / filter prompt or text box
pub fn select_menu_option(choices: &[&str]) -> Result<Option<usize>> {
    if choices.is_empty() {
        return Ok(None);
    }

    let mut selected: usize = 0;
    let total = choices.len();

    enable_raw_mode()?;
    print!("{}", Hide);
    let _ = stdout().flush();

    let result = (|| -> Result<Option<usize>> {
        loop {
            // Render options
            for (i, choice) in choices.iter().enumerate() {
                if i == selected {
                    print!("\r\x1B[2K");
                    print!(
                        "{}> {}{}\r\n",
                        SetForegroundColor(Color::Cyan),
                        choice,
                        ResetColor
                    );
                } else {
                    print!("\r\x1B[2K");
                    print!("  {}\r\n", choice);
                }
            }
            let _ = stdout().flush();

            // Wait for key event
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    return Ok(None);
                }

                match code {
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        selected = if selected == 0 {
                            total - 1
                        } else {
                            selected - 1
                        };
                        print!("\x1B[{}A", total);
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        selected = if selected + 1 >= total {
                            0
                        } else {
                            selected + 1
                        };
                        print!("\x1B[{}A", total);
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        return Ok(Some(selected));
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(None);
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let digit = c.to_digit(10).unwrap_or(0) as usize;
                        if (1..=total).contains(&digit) {
                            return Ok(Some(digit - 1));
                        }
                        print!("\x1B[{}A", total);
                    }
                    _ => {
                        print!("\x1B[{}A", total);
                    }
                }
            }
        }
    })();

    print!("{}", Show);
    let _ = stdout().flush();
    let _ = disable_raw_mode();

    result
}

/// Helper function to browse and select a `.json` scenario file from `get_scenarios_path()`,
/// or allow entering a file path manually.
pub fn prompt_select_scenario_file() -> Result<Option<PathBuf>> {
    let dir = get_scenarios_path()?;
    let mut json_files = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path.extension().is_some_and(|ext| ext == "json")
                && let Some(name) = path.file_name()
            {
                json_files.push(name.to_string_lossy().to_string());
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
        let choice_items: Vec<String> = json_files
            .iter()
            .enumerate()
            .map(|(i, f)| format!("{}. 📄 {}", i + 1, f))
            .collect();
        let choice_strs: Vec<&str> = choice_items.iter().map(|s| s.as_str()).collect();

        println!("\n📂 Select scenario file to load:");
        let selected_idx = match select_menu_option(&choice_strs)? {
            Some(idx) => idx,
            None => return Ok(None),
        };
        let path = dir.join(&json_files[selected_idx]);
        Ok(Some(path))
    }
}

/// 1. Main Menu (`run_main_menu(state: &mut AppState)`):
///
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
        println!(" 🏠 WELCOME TO HOMECALC SCENARIO ENGINE");
        println!(
            "================================================================================"
        );

        let choices = [
            "1. 📝 Create Scenario",
            "2. 📂 Load Scenario",
            "3. 📊 Compare Scenarios",
            "4. 🚪 Exit",
        ];

        let selection = match select_menu_option(&choices)? {
            Some(idx) => idx + 1,
            None => {
                clear_screen();
                println!("\n👋 Exiting Homecalc. Goodbye!\n");
                break;
            }
        };

        match selection {
            1 => {
                clear_screen();
                let purchase_res = prompt_create_purchase();
                let purchase = match purchase_res {
                    Ok(p) => p,
                    Err(e) => {
                        if let Some(inq_err) = e.downcast_ref::<InquireError>()
                            && matches!(
                                inq_err,
                                InquireError::OperationCanceled
                                    | InquireError::OperationInterrupted
                            )
                        {
                            continue;
                        }
                        return Err(e);
                    }
                };
                let scenario = create_scenario(purchase);
                state.set_slot_1(scenario);
                run_scenario_menu(state)?;
            }
            2 => {
                clear_screen();
                if let Some(file_path) = prompt_select_scenario_file()? {
                    load_scenario(&file_path, 1, state)?;
                    run_scenario_menu(state)?;
                }
            }
            3 => {
                clear_screen();
                println!("\n📊 Select the file for baseline scenario: ");
                let file1_opt = prompt_select_scenario_file()?;
                let Some(file1) = file1_opt else { continue };
                load_scenario(&file1, 1, state)?;

                println!("\n📊 Select the file for alternative scenario: ");
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
                        "\n⚠️  Error: Unable to perform comparison because one or both slots are empty."
                    );
                }
            }
            4 => {
                clear_screen();
                println!("\n👋 Exiting Homecalc. Goodbye!\n");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/// 2. Scenario Menu (`run_scenario_menu(state: &mut AppState)`):
///
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
            println!("\n⚠️  No active scenario loaded in Slot 1.");
            break;
        };

        render_summary(s);

        let choices = [
            "1. 💾 Save Scenario",
            "2. 📅 View Statement",
            "3. 🔍 View Analysis",
            "4. 📊 Compare Scenario",
            "5. 🔙 Back",
        ];

        let selection = match select_menu_option(&choices)? {
            Some(idx) => idx + 1,
            None => break,
        };

        match selection {
            1 => {
                clear_screen();
                let default_filename = format!("{}.json", sanitize_filename(&s.purchase.name));
                let filename_res = Text::new("Enter filename to save:")
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
                println!("\n✅ Scenario successfully saved to {:?}", saved_path);
            }
            2 => {
                clear_screen();
                render_statement(s);
                run_scenario_sub_menu()?;
            }
            3 => {
                clear_screen();
                let analysis = analyze_scenario(s);
                render_analysis(&analysis);
                run_scenario_sub_menu()?;
            }
            4 => {
                clear_screen();
                println!("\n📂 Select second scenario file for comparison:");
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
            5 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/// 3. Scenario Sub Menu (`run_scenario_sub_menu()`):
///
/// Options:
/// - 1. Back
pub fn run_scenario_sub_menu() -> Result<()> {
    loop {
        let choices = ["1. 🔙 Back"];
        if let Some(idx) = select_menu_option(&choices)? {
            if idx == 0 {
                break;
            }
        } else {
            break;
        }
    }
    Ok(())
}

/// 4. Comparison Sub-Menu (`run_comparison_menu()`):
///
/// Options:
/// - 1. Back
pub fn run_comparison_menu() -> Result<()> {
    loop {
        let choices = ["1. 🔙 Back"];
        if let Some(idx) = select_menu_option(&choices)? {
            if idx == 0 {
                break;
            }
        } else {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_menu_option_empty() {
        let choices: Vec<&str> = vec![];
        let res = select_menu_option(&choices).unwrap();
        assert_eq!(res, None);
    }
}
