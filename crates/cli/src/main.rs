#![allow(dead_code)]

mod render;
mod session;
mod storage;

use session::state::AppState;

fn main() {
    let _state = AppState::new();
}
