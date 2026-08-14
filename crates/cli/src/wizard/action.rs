//! action.rs
//! Action handlers bridging CLI menus to AppState, storage, and rendering.

use anyhow::Result;
use engine::service::analysis::analyze_scenario;
use engine::service::comparison::compare_scenarios;
use engine::service::simulation::create_scenario;
use inquire::{Select, Text};
use std::fs;

use super::input::prompt_create_purchase;
use crate::render::analysis::render_analysis;
use crate::render::comparison::render_comparison;
use crate::render::statement::render_statement;
use crate::render::summary::render_summary;
use crate::session::state::AppState;
use crate::storage::io::{get_scenarios_path, load_scenario, save_purchase};

/// Action handler for loading a scenario JSON file into an AppState slot.
pub fn handle_load_scenario(state: &mut AppState) -> Result<()> {
    let slot_choice =
        Select::new("Select target AppState slot:", vec!["Slot 1", "Slot 2"]).prompt()?;
    let slot = if slot_choice == "Slot 1" { 1 } else { 2 };

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
        let manual_path = Text::new("Enter scenario file path or filename manually:").prompt()?;
        load_scenario(std::path::Path::new(&manual_path), slot, state)?;
    } else {
        json_files.sort();
        let selected_file = Select::new("Select scenario file to load:", json_files).prompt()?;
        let file_path = dir.join(selected_file);
        load_scenario(&file_path, slot, state)?;
    }

    println!("\n[✓] Scenario successfully loaded into Slot {}!", slot);
    Ok(())
}

/// Action handler for saving an active scenario from AppState to disk.
pub fn handle_save_scenario(state: &AppState) -> Result<()> {
    let slot1_exists = state.get_slot_1().is_some();
    let slot2_exists = state.get_slot_2().is_some();

    if !slot1_exists && !slot2_exists {
        println!("\n[!] No active scenarios loaded in AppState to save.");
        return Ok(());
    }

    let mut options = Vec::new();
    if let Some(s1) = state.get_slot_1() {
        options.push(format!("Slot 1: {}", s1.purchase.name));
    }
    if let Some(s2) = state.get_slot_2() {
        options.push(format!("Slot 2: {}", s2.purchase.name));
    }

    let choice = Select::new("Select scenario slot to save:", options).prompt()?;
    let slot_num = if choice.starts_with("Slot 1") { 1 } else { 2 };
    let scenario = if slot_num == 1 {
        state.get_slot_1().unwrap()
    } else {
        state.get_slot_2().unwrap()
    };

    let default_filename = format!(
        "{}.json",
        scenario.purchase.name.to_lowercase().replace(' ', "_")
    );
    let filename = Text::new("Filename to save (e.g. scenario.json):")
        .with_default(&default_filename)
        .prompt()?;

    let saved_path = save_purchase(&scenario.purchase, &filename)?;
    println!("\n[✓] Scenario successfully saved to {:?}", saved_path);

    Ok(())
}

/// Action handler for viewing active scenarios in AppState.
pub fn handle_view_scenario(state: &AppState) -> Result<()> {
    let slot1_exists = state.get_slot_1().is_some();
    let slot2_exists = state.get_slot_2().is_some();

    if !slot1_exists && !slot2_exists {
        println!("\n[!] No active scenarios loaded in AppState.");
        return Ok(());
    }

    let mut options = Vec::new();
    if let Some(s1) = state.get_slot_1() {
        options.push(format!("Slot 1: {}", s1.purchase.name));
    }
    if let Some(s2) = state.get_slot_2() {
        options.push(format!("Slot 2: {}", s2.purchase.name));
    }

    let choice = Select::new("Select scenario slot to view:", options).prompt()?;
    let scenario = if choice.starts_with("Slot 1") {
        state.get_slot_1().unwrap()
    } else {
        state.get_slot_2().unwrap()
    };

    let view_choice = Select::new(
        "Select display section:",
        vec![
            "1. Purchase Summary & Parameters",
            "2. Monthly Statement & Yearly Totals",
            "3. Single Scenario Analysis Metrics",
            "4. View Complete Report (All Sections)",
        ],
    )
    .prompt()?;

    match view_choice {
        c if c.starts_with("1") => render_summary(scenario),
        c if c.starts_with("2") => render_statement(scenario),
        c if c.starts_with("3") => {
            let analysis = analyze_scenario(scenario);
            render_analysis(&analysis);
        }
        _ => {
            render_summary(scenario);
            render_statement(scenario);
            let analysis = analyze_scenario(scenario);
            render_analysis(&analysis);
        }
    }

    Ok(())
}

/// Action handler for comparing scenarios between Slot 1 and Slot 2.
pub fn handle_compare_scenarios(state: &AppState) -> Result<()> {
    let slot1 = state.get_slot_1();
    let slot2 = state.get_slot_2();

    match (slot1, slot2) {
        (Some(s1), Some(s2)) => {
            let comparison = compare_scenarios(s1, s2);
            render_comparison(&comparison);
        }
        _ => {
            println!(
                "\n[!] Error: Both Slot 1 and Slot 2 must contain active scenarios to run comparison."
            );
            println!(
                "Current status: Slot 1 = {}, Slot 2 = {}",
                if slot1.is_some() { "Loaded" } else { "Empty" },
                if slot2.is_some() { "Loaded" } else { "Empty" }
            );
        }
    }

    Ok(())
}

/// Action handler for interactively creating a new scenario and storing it in AppState.
pub fn handle_create_scenario(state: &mut AppState) -> Result<()> {
    let purchase = prompt_create_purchase()?;
    let scenario = create_scenario(purchase);

    let slot_choice = Select::new(
        "Select AppState slot to store new scenario:",
        vec!["Slot 1", "Slot 2"],
    )
    .prompt()?;

    let slot = if slot_choice == "Slot 1" { 1 } else { 2 };
    match slot {
        1 => state.set_slot_1(scenario),
        2 => state.set_slot_2(scenario),
        _ => unreachable!(),
    }

    println!(
        "\n[✓] New Purchase Scenario successfully evaluated and stored in Slot {}!",
        slot
    );
    Ok(())
}

/// Action handler for clearing active scenario slots in AppState.
pub fn handle_clear_state(state: &mut AppState) -> Result<()> {
    let options = vec![
        "1. Clear Slot 1",
        "2. Clear Slot 2",
        "3. Clear Both Slots",
        "4. Cancel",
    ];

    let choice = Select::new("Select state clear option:", options).prompt()?;

    match choice {
        c if c.starts_with("1") => {
            state.clear_slot_1();
            println!("\n[✓] Cleared Slot 1.");
        }
        c if c.starts_with("2") => {
            state.clear_slot_2();
            println!("\n[✓] Cleared Slot 2.");
        }
        c if c.starts_with("3") => {
            state.clear_all();
            println!("\n[✓] Cleared both Slot 1 and Slot 2.");
        }
        _ => {}
    }

    Ok(())
}
