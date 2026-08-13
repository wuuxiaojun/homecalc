//! main.rs
//! Main function for CLI

mod state;
mod storage;

#[allow(dead_code)]
use state::AppState;

fn main() {
    let _state = AppState::new();
}
