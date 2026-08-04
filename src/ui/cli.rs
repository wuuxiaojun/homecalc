use std::fs;
use std::io::{self, Write};
use std::path::Path;

use chrono::{Local, NaiveDate};
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use inquire::{CustomType, Text};

use crate::domain::loc::LocEngine;
use crate::ui::display::{
    print_annual_summary_table, print_banner, print_loc_summary, print_monthly_statement_table,
};
use crate::ui::terminal::{clear_screen, BOX_BORDER};

/// Renders a interactive menu with keyboard (Up/Down arrow, 1-9 digits, Enter) navigation.
pub fn select_menu<F>(render_header: F, options: &[&str]) -> Result<usize, io::Error>
where
    F: Fn(),
{
    let mut selected_index = 0;

    loop {
        clear_screen();
        render_header();
        println!();

        for (idx, opt) in options.iter().enumerate() {
            if idx == selected_index {
                println!("  > \x1b[36m[{}] {}\x1b[0m", idx + 1, opt);
            } else {
                println!("    [{}] {}", idx + 1, opt);
            }
        }
        println!(
            "\n  (Use Up/Down arrows or press number keys 1-{}, Enter to select)\n",
            options.len()
        );
        io::stdout().flush()?;

        enable_raw_mode()?;
        let event = read();
        disable_raw_mode()?;

        match event {
            Ok(Event::Key(key_event)) => match key_event.code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    } else {
                        selected_index = options.len() - 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index + 1 < options.len() {
                        selected_index += 1;
                    } else {
                        selected_index = 0;
                    }
                }
                KeyCode::Enter => {
                    return Ok(selected_index);
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if let Some(digit) = c.to_digit(10) {
                        let num = digit as usize;
                        if num >= 1 && num <= options.len() {
                            return Ok(num - 1);
                        }
                    }
                }
                _ => {}
            },
            Err(e) => return Err(e),
            _ => {}
        }
    }
}

/// Prompts the user interactively for SBLOC parameters using `inquire`.
fn prompt_sbloc_params() -> Option<(String, f64, f64, f64, f64, NaiveDate)> {
    clear_screen();
    print_banner("📝", "NEW SBLOC SCENARIO PARAMETERS");
    println!();

    let name = match Text::new("Scenario Name:")
        .with_default("Primary SBLOC Home Loan")
        .prompt()
    {
        Ok(val) => val,
        Err(_) => return None,
    };

    let initial_draw = match CustomType::<f64>::new("Initial Draw Amount ($):")
        .with_default(1_500_000.0)
        .prompt()
    {
        Ok(val) => val,
        Err(_) => return None,
    };

    let annual_rate = match CustomType::<f64>::new("Annual Interest Rate (%):")
        .with_default(6.0)
        .prompt()
    {
        Ok(val) => val,
        Err(_) => return None,
    };

    let property_tax_rate = match CustomType::<f64>::new("Property Tax Rate (%):")
        .with_default(1.2)
        .prompt()
    {
        Ok(val) => val,
        Err(_) => return None,
    };

    let annual_insurance = match CustomType::<f64>::new("Annual Homeowners Insurance ($):")
        .with_default(3600.0)
        .prompt()
    {
        Ok(val) => val,
        Err(_) => return None,
    };

    let default_date = Local::now().format("%Y-%m-%d").to_string();
    let start_date_str = match Text::new("Start Date (YYYY-MM-DD):")
        .with_default(&default_date)
        .prompt()
    {
        Ok(val) => val,
        Err(_) => return None,
    };

    let start_date = match NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("\x1b[31mInvalid date format. Defaulting to today's date.\x1b[0m");
            Local::now().date_naive()
        }
    };

    Some((
        name,
        initial_draw,
        annual_rate,
        property_tax_rate,
        annual_insurance,
        start_date,
    ))
}

/// Handles creating a new SBLOC scenario.
fn handle_new_sbloc(main_status: &mut Option<String>) {
    if let Some((name, draw, rate, tax, ins, start_date)) = prompt_sbloc_params() {
        match LocEngine::new(name, start_date, draw, rate, tax, ins) {
            Ok(engine) => {
                active_sbloc_menu(engine);
            }
            Err(e) => {
                *main_status = Some(format!("Error creating SBLOC engine: {}", e));
            }
        }
    }
}

/// Handles loading a saved SBLOC scenario from JSON files in `./products`.
fn handle_load_sbloc(main_status: &mut Option<String>) {
    let products_dir = Path::new("./products");
    if !products_dir.exists() {
        *main_status = Some("No ./products directory found. Save a scenario first.".to_string());
        return;
    }

    let entries = match fs::read_dir(products_dir) {
        Ok(read_dir) => read_dir,
        Err(e) => {
            *main_status = Some(format!("Failed to read ./products: {}", e));
            return;
        }
    };

    let mut json_files = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            json_files.push(path);
        }
    }

    json_files.sort();

    if json_files.is_empty() {
        *main_status = Some("No .json scenario files found in ./products".to_string());
        return;
    }

    let options: Vec<String> = json_files
        .iter()
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
        .collect();
    let option_strs: Vec<&str> = options.iter().map(|s| s.as_str()).collect();

    let render_header = || {
        print_banner("📂", "LOAD SAVED SBLOC SCENARIO");
    };

    match select_menu(render_header, &option_strs) {
        Ok(idx) => {
            let selected_file = &json_files[idx];
            match LocEngine::load_from_json(&selected_file.to_string_lossy()) {
                Ok(engine) => {
                    active_sbloc_menu(engine);
                }
                Err(e) => {
                    *main_status = Some(format!("Failed to load JSON scenario: {}", e));
                }
            }
        }
        Err(e) => {
            *main_status = Some(format!("Menu selection error: {}", e));
        }
    }
}

/// Active interactive menu loop for managing a specific `LocEngine` scenario.
fn active_sbloc_menu(mut engine: LocEngine) {
    let mut status_msg: Option<String> = None;

    loop {
        let engine_ref = &engine;
        let status_ref = &status_msg;

        let render_header = move || {
            print_loc_summary(engine_ref);
            println!();
            print_monthly_statement_table(engine_ref);
            println!();
            print_annual_summary_table(engine_ref);

            if let Some(msg) = status_ref {
                println!("\n  \x1b[33mℹ {}\x1b[0m", msg);
            }
        };

        let options = [
            "Add / Update Extra Principal Payment",
            "Set Recurring Monthly Principal Payoff",
            "Save Scenario to JSON",
            "Back to Main Menu",
        ];

        match select_menu(render_header, &options) {
            Ok(0) => {
                // Add / Update Extra Principal Payment
                clear_screen();
                println!("{}", BOX_BORDER);
                println!("|  ➕ ADD / UPDATE EXTRA PRINCIPAL PAYMENT                                                                   |");
                println!("{}", BOX_BORDER);
                println!();

                let month = match CustomType::<u32>::new("Month Index (1, 2, 3...):")
                    .with_default(1)
                    .prompt()
                {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                let amount = match CustomType::<f64>::new("Extra Principal Amount ($):")
                    .with_default(100_000.0)
                    .prompt()
                {
                    Ok(a) => a,
                    Err(_) => continue,
                };

                engine.add_extra_payment(month, amount);
                status_msg = Some(format!(
                    "Added extra payment of ${:.2} to Month {}",
                    amount, month
                ));
            }
            Ok(1) => {
                // Set Recurring Monthly Principal Payoff
                clear_screen();
                println!("{}", BOX_BORDER);
                println!("|  🔄 SET RECURRING MONTHLY EXTRA PRINCIPAL                                                                  |");
                println!("{}", BOX_BORDER);
                println!();

                let amount = match CustomType::<f64>::new("Recurring Extra Principal per Month ($):")
                    .with_default(10_000.0)
                    .prompt()
                {
                    Ok(a) => a,
                    Err(_) => continue,
                };

                engine.set_recurring_extra_payment(amount);
                status_msg = Some(format!(
                    "Set recurring monthly extra principal to ${:.2}/month",
                    amount
                ));
            }
            Ok(2) => {
                // Save Scenario to JSON
                clear_screen();
                println!("{}", BOX_BORDER);
                println!("|  💾 SAVE SCENARIO TO JSON                                                                                 |");
                println!("{}", BOX_BORDER);
                println!();

                let default_filename = engine.name.to_lowercase().replace(' ', "_");
                let filename = match Text::new("Filename (e.g. my_sbloc):")
                    .with_default(&default_filename)
                    .prompt()
                {
                    Ok(f) => f,
                    Err(_) => continue,
                };

                match engine.save_to_json("./products", &filename) {
                    Ok(path) => {
                        status_msg = Some(format!("Successfully saved scenario to {}", path));
                    }
                    Err(e) => {
                        status_msg = Some(format!("Failed to save scenario: {}", e));
                    }
                }
            }
            Ok(3) | Err(_) => {
                break;
            }
            _ => {}
        }
    }
}

/// Main entry point for running the interactive CLI event loop.
pub fn run_cli() {
    let mut main_status: Option<String> = None;

    loop {
        let status_ref = &main_status;
        let render_header = move || {
            print_banner("🏛️", "SBLOC HOUSING CALCULATOR - MAIN MENU");
            if let Some(msg) = status_ref {
                println!("\n  \x1b[31m⚠ {}\x1b[0m", msg);
            }
        };

        let options = [
            "Start New SBLOC Scenario",
            "Load Saved SBLOC Scenario from ./products",
            "Exit",
        ];

        match select_menu(render_header, &options) {
            Ok(0) => {
                main_status = None;
                handle_new_sbloc(&mut main_status);
            }
            Ok(1) => {
                main_status = None;
                handle_load_sbloc(&mut main_status);
            }
            Ok(2) | Err(_) => {
                clear_screen();
                println!("Thank you for using SBLOC Housing Calculator. Goodbye!");
                break;
            }
            _ => {}
        }
    }
}
