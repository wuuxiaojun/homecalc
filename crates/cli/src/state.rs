//! state.rs
//! App State

use homecalc_engine::domain::scenario::Scenario;
// 2-slot RAM buffer for the CLI session
#[derive(Debug, Default)]
pub struct AppState {
    slot_1: Option<Scenario>,
    slot_2: Option<Scenario>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            slot_1: None,
            slot_2: None,
        }
    }

    pub fn set_slot_1(&mut self, scenario: Scenario) {
        self.slot_1 = Some(scenario);
    }

    pub fn set_slot_2(&mut self, scenario: Scenario) {
        self.slot_2 = Some(scenario);
    }

    pub fn get_slot_1(&self) -> Option<&Scenario> {
        self.slot_1.as_ref()
    }

    pub fn get_slot_2(&self) -> Option<&Scenario> {
        self.slot_2.as_ref()
    }
}
