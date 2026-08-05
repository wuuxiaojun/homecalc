// src/main.rs

pub mod analysis;
pub mod domain;
pub mod ui;

fn main() {
    ui::cli::run_cli();
}
