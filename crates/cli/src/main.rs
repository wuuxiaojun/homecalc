//! main.rs
//! Main entry point for Homecalc CLI application.

#![allow(dead_code)]
#![allow(unused_imports)]

mod render;
mod session;
mod storage;
mod wizard;

use anyhow::Result;
use session::state::AppState;

fn main() -> Result<()> {
    let mut state = AppState::new();
    wizard::menu::run_main_menu(&mut state)?;
    Ok(())
}
