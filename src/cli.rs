// src/cli.rs

use crate::analysis::compare_mortgages;
use crate::display::{clear_screen, print_annual_summary_table, print_banner, print_comparison_report, print_mortgage_summary};
use crate::mortgage::Mortgage;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use inquire::validator::Validation;
use inquire::{Confirm, CustomType, Text};
use std::fs;
use std::io::{self, Write};

/// Custom menu selector supporting Up/Down arrows + Enter AND instant numeric key (1-9) selection.
/// Accepts a rendering closure for the header so full screens are preserved on redraw.
pub fn select_menu<F>(render_header: F, options: &[&str]) -> Result<usize, io::Error>
where
    F: Fn(),
{
    let mut selected_idx: usize = 0;
    let total = options.len();

    if total == 0 {
        return Ok(0);
    }

    loop {
        clear_screen();
        render_header();

        println!();
        for (idx, opt) in options.iter().enumerate() {
            let num = idx + 1;
            if idx == selected_idx {
                println!("  \x1B[1;33m➔ [{}] {}\x1B[0m", num, opt);
            } else {
                println!("    \x1B[2m[{}] {}\x1B[0m", num, opt);
            }
        }
        println!();
        let _ = io::stdout().flush();

        enable_raw_mode()?;
        let event_res = event::read();
        let _ = disable_raw_mode();

        match event_res {
            Ok(Event::Key(KeyEvent { code, modifiers, .. })) => {
                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    return Ok(total.saturating_sub(1));
                }

                match code {
                    KeyCode::Up => {
                        selected_idx = (selected_idx + total - 1) % total;
                    }
                    KeyCode::Down => {
                        selected_idx = (selected_idx + 1) % total;
                    }
                    KeyCode::Enter => {
                        return Ok(selected_idx);
                    }
                    KeyCode::Char(ch) => {
                        if let Some(digit) = ch.to_digit(10) {
                            let d = digit as usize;
                            if d >= 1 && d <= total {
                                return Ok(d - 1);
                            }
                        }
                        if ch == 'q' || ch == 'Q' {
                            return Ok(total.saturating_sub(1));
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(total.saturating_sub(1));
                    }
                    _ => {}
                }
            }
            Err(e) => {
                return Err(e);
            }
            _ => {}
        }
    }
}

pub fn run_cli() {
    let mut main_status: Option<String> = None;

    loop {
        let options = vec![
            "Start New Mortgage Scenario",
            "Load Saved Mortgage Scenario",
            "Compare 2 Mortgage Scenarios",
            "Quit",
        ];

        let selection = match select_menu(
            || {
                print_banner("🏠", "MORTGAGE ENGINE & HOUSING FINANCIAL CALCULATOR (CLI)");
                if let Some(msg) = &main_status {
                    println!("\n {}", msg);
                }
            },
            &options,
        ) {
            Ok(idx) => idx,
            Err(_) => break,
        };

        main_status = None;

        match selection {
            0 => handle_new_mortgage(&mut main_status),
            1 => handle_load_mortgage(&mut main_status),
            2 => handle_compare_mortgages(&mut main_status),
            _ => {
                clear_screen();
                println!("\nThank you for using Mortgage Engine. Goodbye!");
                break;
            }
        }
    }
}

fn prompt_mortgage_params() -> Option<(String, f64, f64, f64, u32, f64, f64)> {
    clear_screen();
    print_banner("🏠", "CREATE NEW MORTGAGE SCENARIO");
    println!();

    let name = Text::new("Mortgage Scenario Name:")
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(Validation::Invalid("Scenario name cannot be empty.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let price = CustomType::<f64>::new("Home Price ($):")
        .with_validator(|val: &f64| {
            if *val <= 0.0 || val.is_nan() || val.is_infinite() {
                Ok(Validation::Invalid("Home price must be greater than 0.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let down = CustomType::<f64>::new("Down Payment ($):")
        .with_validator(move |val: &f64| {
            if *val < 0.0 || val.is_nan() || val.is_infinite() {
                Ok(Validation::Invalid("Down payment cannot be negative.".into()))
            } else if *val >= price {
                Ok(Validation::Invalid(format!("Down payment cannot equal or exceed home price (${:.2}).", price).into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let rate = CustomType::<f64>::new("Interest Rate (%):")
        .with_validator(|val: &f64| {
            if *val < 0.0 || *val > 100.0 || val.is_nan() || val.is_infinite() {
                Ok(Validation::Invalid("Interest rate must be between 0% and 100%.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let term = CustomType::<u32>::new("Loan Term (Years):")
        .with_validator(|val: &u32| {
            if *val < 1 || *val > 30 {
                Ok(Validation::Invalid("Loan term must be between 1 and 30 years.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let tax_rate = CustomType::<f64>::new("Annual Property Tax Rate (%):")
        .with_validator(|val: &f64| {
            if *val < 0.0 || *val > 20.0 || val.is_nan() || val.is_infinite() {
                Ok(Validation::Invalid("Property tax rate must be between 0% and 20%.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let annual_insurance = CustomType::<f64>::new("Annual Home Insurance ($):")
        .with_validator(|val: &f64| {
            if *val < 0.0 || val.is_nan() || val.is_infinite() {
                Ok(Validation::Invalid("Annual insurance cannot be negative.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let confirm = Confirm::new("Create mortgage scenario with these parameters?").prompt();
    if let Ok(true) = confirm {
        Some((name, price, down, rate, term, tax_rate, annual_insurance))
    } else {
        None
    }
}

fn handle_new_mortgage(main_status: &mut Option<String>) {
    if let Some((name, price, down, rate, term, tax_rate, annual_insurance)) = prompt_mortgage_params() {
        match Mortgage::new(name, price, down, rate, term, tax_rate, annual_insurance) {
            Ok(mortgage) => {
                active_mortgage_menu(mortgage);
            }
            Err(e) => {
                *main_status = Some(format!("❌ Error creating mortgage: {:?}", e));
            }
        }
    }
}

fn handle_load_mortgage(main_status: &mut Option<String>) {
    let filepath = match select_json_file("📂", "LOAD SAVED MORTGAGE SCENARIO") {
        Some(path) => path,
        None => return,
    };

    match Mortgage::load_from_json(&filepath) {
        Ok(mortgage) => {
            active_mortgage_menu(mortgage);
        }
        Err(e) => {
            *main_status = Some(format!("❌ Failed to load mortgage from {}: {}", filepath, e));
        }
    }
}

fn handle_compare_mortgages(main_status: &mut Option<String>) {
    let path1 = match select_json_file("⚖️", "COMPARE MORTGAGES - SELECT OPTION A") {
        Some(path) => path,
        None => return,
    };

    let path2 = match select_json_file("⚖️", "COMPARE MORTGAGES - SELECT OPTION B") {
        Some(path) => path,
        None => return,
    };

    let option_a = match Mortgage::load_from_json(&path1) {
        Ok(m) => m,
        Err(e) => {
            *main_status = Some(format!("❌ Failed to load Option A from {}: {}", path1, e));
            return;
        }
    };

    let option_b = match Mortgage::load_from_json(&path2) {
        Ok(m) => m,
        Err(e) => {
            *main_status = Some(format!("❌ Failed to load Option B from {}: {}", path2, e));
            return;
        }
    };

    let report = compare_mortgages(&option_a, &option_b);

    clear_screen();
    print_comparison_report(&report, &option_a.name, &option_b.name);
    let _ = Text::new("Press Enter to return to Main Menu:").prompt();
}

fn prompt_extra_payment_input(mortgage: &Mortgage) -> Option<(u32, f64)> {
    let max_month = mortgage.schedule.len() as u32;
    let month = CustomType::<u32>::new("Target Month for Extra Payment:")
        .with_validator(move |val: &u32| {
            if *val < 1 || *val > max_month {
                Ok(Validation::Invalid(format!("Target month must be between 1 and {}.", max_month).into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    let amount = CustomType::<f64>::new("Extra Principal Amount ($):")
        .with_validator(|val: &f64| {
            if *val <= 0.0 || val.is_nan() || val.is_infinite() {
                Ok(Validation::Invalid("Extra payment amount must be greater than 0.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()?;

    Some((month, amount))
}

fn prompt_save_filename_input() -> Option<String> {
    Text::new("Filename to save:")
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(Validation::Invalid("Filename cannot be empty.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .ok()
}

fn active_mortgage_menu(mut mortgage: Mortgage) {
    let mut status_message: Option<String> = None;

    loop {
        let options = vec![
            "Add / Update Extra Principal Payment",
            "Save Mortgage Scenario to JSON",
            "Back to Main Menu",
        ];

        let selection = match select_menu(
            || {
                print_mortgage_summary(&mortgage);
                print_annual_summary_table(&mortgage);
                if let Some(msg) = &status_message {
                    println!(" {}\n", msg);
                }
                print_banner("⚙️", "ACTIVE SCENARIO ACTIONS");
            },
            &options,
        ) {
            Ok(idx) => idx,
            Err(_) => break,
        };

        status_message = None;

        match selection {
            0 => {
                if let Some((month, amount)) = prompt_extra_payment_input(&mortgage) {
                    match mortgage.add_extra_payment(month, amount) {
                        Ok(added) => {
                            status_message = Some(format!("✅ Applied ${:.2} extra payment at Month {}.", added, month));
                        }
                        Err(e) => {
                            status_message = Some(format!("❌ Error adding extra payment: {}", e));
                        }
                    }
                }
            }
            1 => {
                if let Some(filename) = prompt_save_filename_input() {
                    match mortgage.save_to_json("./products", &filename) {
                        Ok(path) => {
                            status_message = Some(format!("✅ Successfully saved mortgage scenario to {}", path));
                        }
                        Err(e) => {
                            status_message = Some(format!("❌ Failed to save scenario: {}", e));
                        }
                    }
                }
            }
            _ => break,
        }
    }
}

fn get_json_files_in_dir(dir_path: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let is_json = path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json");
            if is_json {
                let filename = path.file_name().and_then(|s| s.to_str());
                if let Some(name) = filename {
                    files.push(name.to_string());
                }
            }
        }
    }
    files.sort();
    files
}

fn select_json_file(emoji: &str, title: &str) -> Option<String> {
    let dir_path = "./products";
    let files = get_json_files_in_dir(dir_path);

    if files.is_empty() {
        clear_screen();
        print_banner(emoji, title);
        let custom = Text::new("No saved files found in ./products. Enter custom file path:").prompt();
        match custom {
            Ok(path) if !path.trim().is_empty() => Some(path.trim().to_string()),
            _ => None,
        }
    } else {
        let mut options: Vec<String> = files.iter().map(|f| format!("Product: {}", f)).collect();
        options.push("Custom File Path...".to_string());
        options.push("Cancel".to_string());

        let str_options: Vec<&str> = options.iter().map(|s| s.as_str()).collect();

        match select_menu(
            || {
                print_banner(emoji, title);
            },
            &str_options,
        ) {
            Ok(idx) if idx < files.len() => Some(format!("{}/{}", dir_path, files[idx])),
            Ok(idx) if idx == files.len() => {
                clear_screen();
                print_banner(emoji, title);
                match Text::new("Enter custom JSON file path:").prompt() {
                    Ok(p) if !p.trim().is_empty() => Some(p.trim().to_string()),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
