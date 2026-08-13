//! main.rs
//! Main function for CLI

mod display;
mod session;
mod storage;

use session::state::AppState;

fn main() {
    let _state = AppState::new();
}
