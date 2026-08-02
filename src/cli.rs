// src/cli.rs

use crate::analysis::compare_mortgages;
use crate::display::{clear_screen, print_annual_summary_table, print_comparison_report, print_mortgage_summary};
use crate::mortgage::Mortgage;
use inquire::{Confirm, CustomType, Select, Text};
use std::fs;

pub fn run_cli() {
    let mut main_status: Option<String> = None;

    loop {
        clear_screen();
        println!("===============================================================================================================");
        println!(" 🏠 MORTGAGE ENGINE & HOUSING FINANCIAL CALCULATOR (CLI)");
        println!("===============================================================================================================");
        println!(" [1] Start New Mortgage Scenario");
        println!(" [2] Load Saved Mortgage Scenario");
        println!(" [3] Compare 2 Mortgage Scenarios");
        println!(" [4] Quit");
        println!("===============================================================================================================");

        if let Some(msg) = &main_status {
            println!(" {}\n", msg);
        }

        let input = match Text::new("Select option [1-4]:").with_default("1").prompt() {
            Ok(val) => val.trim().to_string(),
            Err(_) => break,
        };

        main_status = None;

        match input.as_str() {
            "1" => handle_new_mortgage(&mut main_status),
            "2" => handle_load_mortgage(&mut main_status),
            "3" => handle_compare_mortgages(&mut main_status),
            "4" => {
                println!("\nThank you for using Mortgage Engine. Goodbye!");
                break;
            }
            _ => {
                main_status = Some("❌ Invalid option. Please enter 1, 2, 3, or 4.".to_string());
            }
        }
    }
}

fn handle_new_mortgage(main_status: &mut Option<String>) {
    clear_screen();
    println!("--- Create New Mortgage Scenario ---");

    let name = match Text::new("Mortgage Scenario Name:").with_default("Primary Home").prompt() {
        Ok(val) => val,
        Err(_) => return,
    };

    let price = match CustomType::<f64>::new("Home Price ($):").with_default(1_500_000.0).prompt() {
        Ok(val) => val,
        Err(_) => return,
    };

    let down = match CustomType::<f64>::new("Down Payment ($):").with_default(300_000.0).prompt() {
        Ok(val) => val,
        Err(_) => return,
    };

    let rate = match CustomType::<f64>::new("Interest Rate (%):").with_default(5.9).prompt() {
        Ok(val) => val,
        Err(_) => return,
    };

    let term = match CustomType::<u32>::new("Loan Term (Years):").with_default(15).prompt() {
        Ok(val) => val,
        Err(_) => return,
    };

    let tax_rate = match CustomType::<f64>::new("Annual Property Tax Rate (%):").with_default(1.2).prompt() {
        Ok(val) => val,
        Err(_) => return,
    };

    let annual_insurance = match CustomType::<f64>::new("Annual Home Insurance ($):").with_default(3_600.0).prompt() {
        Ok(val) => val,
        Err(_) => return,
    };

    let confirm = Confirm::new("Create mortgage scenario with these parameters?")
        .with_default(true)
        .prompt();

    if let Ok(true) = confirm {
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
    clear_screen();
    println!("--- Load Saved Mortgage Scenario ---");

    let filepath = match select_json_file("Select a saved mortgage scenario file:") {
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
    clear_screen();
    println!("--- Compare 2 Mortgage Scenarios Side-by-Side ---");

    println!("\nSelecting Option A (First Scenario):");
    let path1 = match select_json_file("Select file for Option A:") {
        Some(path) => path,
        None => return,
    };

    println!("\nSelecting Option B (Second Scenario):");
    let path2 = match select_json_file("Select file for Option B:") {
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
    let _ = Text::new("Press Enter to return to Main Menu:").with_default("").prompt();
}

fn active_mortgage_menu(mut mortgage: Mortgage) {
    let mut status_message: Option<String> = None;

    loop {
        clear_screen();
        print_mortgage_summary(&mortgage);
        print_annual_summary_table(&mortgage);

        if let Some(msg) = &status_message {
            println!(" {}\n", msg);
        }

        println!("===============================================================================================================");
        println!(" ACTIVE SCENARIO ACTIONS:");
        println!(" [1] Add / Update Extra Principal Payment");
        println!(" [2] Save Mortgage Scenario to JSON");
        println!(" [3] Back to Main Menu");
        println!("===============================================================================================================");

        let input = match Text::new("Select action [1-3]:").with_default("1").prompt() {
            Ok(val) => val.trim().to_string(),
            Err(_) => break,
        };

        status_message = None;

        match input.as_str() {
            "1" => {
                let month = match CustomType::<u32>::new("Target Month for Extra Payment (1 to N):").prompt() {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                let amount = match CustomType::<f64>::new("Extra Principal Amount ($):").prompt() {
                    Ok(a) => a,
                    Err(_) => continue,
                };
                match mortgage.add_extra_payment(month, amount) {
                    Ok(added) => {
                        status_message = Some(format!("✅ Applied ${:.2} extra payment at Month {}.", added, month));
                    }
                    Err(e) => {
                        status_message = Some(format!("❌ Error adding extra payment: {}", e));
                    }
                }
            }
            "2" => {
                let default_filename = mortgage.name.to_lowercase().replace(' ', "_");
                let filename = match Text::new("Filename to save (under ./products):").with_default(&default_filename).prompt() {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                match mortgage.save_to_json("./products", &filename) {
                    Ok(path) => {
                        status_message = Some(format!("✅ Successfully saved mortgage scenario to {}", path));
                    }
                    Err(e) => {
                        status_message = Some(format!("❌ Failed to save scenario: {}", e));
                    }
                }
            }
            "3" => break,
            _ => {
                status_message = Some("❌ Invalid action. Please enter 1, 2, or 3.".to_string());
            }
        }
    }
}

fn select_json_file(prompt: &str) -> Option<String> {
    let mut files = Vec::new();
    let dir_path = "./products";

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    files.push(filename.to_string());
                }
            }
        }
    }
    files.sort();

    if files.is_empty() {
        let custom = Text::new("No files found in ./products. Enter custom file path (or blank to cancel):").prompt();
        match custom {
            Ok(path) if !path.trim().is_empty() => Some(path.trim().to_string()),
            _ => None,
        }
    } else {
        let custom_option = "Custom File Path...";
        let cancel_option = "Cancel";

        let mut choices = files.clone();
        choices.push(custom_option.to_string());
        choices.push(cancel_option.to_string());

        match Select::new(prompt, choices).prompt() {
            Ok(selected) => {
                if selected == custom_option {
                    match Text::new("Enter custom JSON file path:").prompt() {
                        Ok(p) if !p.trim().is_empty() => Some(p.trim().to_string()),
                        _ => None,
                    }
                } else if selected == cancel_option {
                    None
                } else {
                    Some(format!("{}/{}", dir_path, selected))
                }
            }
            Err(_) => None,
        }
    }
}
