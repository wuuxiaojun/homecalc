//! main.rs
//! Main function for CLI

mod session;
mod storage;

use session::state::AppState;

fn main() {
    let _state = AppState::new();
}
