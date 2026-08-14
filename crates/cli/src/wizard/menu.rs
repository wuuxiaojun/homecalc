//! menu.rs
//! Main interactive CLI menu loop.

use anyhow::Result;
use inquire::Select;

use super::action::{
    handle_clear_state, handle_compare_scenarios, handle_create_scenario, handle_load_scenario,
    handle_save_scenario, handle_view_scenario,
};
use crate::session::state::AppState;

/// Main interactive menu loop for Homecalc CLI.
pub fn run_main_menu(state: &mut AppState) -> Result<()> {
    println!("\n================================================================================");
    println!(" WELCOME TO HOMECALC FINANCIAL AMORTIZATION & SCENARIO ENGINE");
    println!("================================================================================");

    loop {
        // Render Active State Header
        println!("\n--------------------------------------------------------------------------------");
        println!(
            " ACTIVE STATE: Slot 1 [{}] | Slot 2 [{}]",
            state
                .get_slot_1()
                .map_or("Empty".to_string(), |s| s.purchase.name.clone()),
            state
                .get_slot_2()
                .map_or("Empty".to_string(), |s| s.purchase.name.clone())
        );
        println!("--------------------------------------------------------------------------------");

        let choices = vec![
            "1. View Active Scenario (Summary / Statements / Analysis)",
            "2. Create New Purchase Scenario",
            "3. Compare Scenarios (Slot 1 vs Slot 2)",
            "4. Load Scenario File into Slot",
            "5. Save Active Scenario to JSON",
            "6. Clear Active State Slot(s)",
            "7. Exit",
        ];

        let selection = Select::new("Select Main Menu Option:", choices).prompt()?;

        match selection {
            c if c.starts_with("1") => {
                if let Err(e) = handle_view_scenario(state) {
                    println!("\n[!] Error viewing scenario: {:?}", e);
                }
            }
            c if c.starts_with("2") => {
                if let Err(e) = handle_create_scenario(state) {
                    println!("\n[!] Error creating scenario: {:?}", e);
                }
            }
            c if c.starts_with("3") => {
                if let Err(e) = handle_compare_scenarios(state) {
                    println!("\n[!] Error comparing scenarios: {:?}", e);
                }
            }
            c if c.starts_with("4") => {
                if let Err(e) = handle_load_scenario(state) {
                    println!("\n[!] Error loading scenario: {:?}", e);
                }
            }
            c if c.starts_with("5") => {
                if let Err(e) = handle_save_scenario(state) {
                    println!("\n[!] Error saving scenario: {:?}", e);
                }
            }
            c if c.starts_with("6") => {
                if let Err(e) = handle_clear_state(state) {
                    println!("\n[!] Error clearing state: {:?}", e);
                }
            }
            c if c.starts_with("7") => {
                println!("\nExiting Homecalc. Goodbye!\n");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
