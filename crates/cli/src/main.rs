//! main.rs
//! Main entry point for Homecalc CLI application.

mod render;
mod session;
mod storage;
mod wizard;

use anyhow::Result;
use session::state::AppState;

fn main() -> Result<()> {
    let mut state = AppState::new();
    wizard::run_main_menu(&mut state)?;
    Ok(())
}
