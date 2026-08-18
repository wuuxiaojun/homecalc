//! state.rs
//! App State

use engine::domain::scenario::Scenario;

/// 2-slot RAM buffer for the CLI session
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

#[cfg(test)]
mod tests {
    use super::*;
    use engine::domain::house::House;
    use engine::domain::purchase::Purchase;
    use engine::domain::tool::{Cash, Tool};
    use engine::service::simulation::create_scenario;
    use std::collections::BTreeMap;

    fn make_test_scenario(name: &str) -> Scenario {
        let purchase = Purchase {
            name: name.to_string(),
            house: House {
                purchase_price: 500_000.0,
                annual_property_tax_rate: 1.0,
                annual_insurance: 1_000.0,
                monthly_hoa: 0.0,
            },
            tools: vec![Tool::Cash(Cash {
                amount: 500_000.0,
                rate: 4.0,
            })],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };
        create_scenario(purchase)
    }

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        assert!(state.get_slot_1().is_none());
        assert!(state.get_slot_2().is_none());
    }

    #[test]
    fn test_app_state_set_and_get_slots() {
        let mut state = AppState::new();
        let s1 = make_test_scenario("Scenario 1");
        let s2 = make_test_scenario("Scenario 2");

        state.set_slot_1(s1);
        assert!(state.get_slot_1().is_some());
        assert_eq!(state.get_slot_1().unwrap().purchase.name, "Scenario 1");
        assert!(state.get_slot_2().is_none());

        state.set_slot_2(s2);
        assert!(state.get_slot_2().is_some());
        assert_eq!(state.get_slot_2().unwrap().purchase.name, "Scenario 2");
    }

    #[test]
    fn test_app_state_overwrite() {
        let mut state = AppState::new();
        let s1 = make_test_scenario("Initial");
        let s2 = make_test_scenario("Overwritten");

        state.set_slot_1(s1);
        assert_eq!(state.get_slot_1().unwrap().purchase.name, "Initial");

        state.set_slot_1(s2);
        assert_eq!(state.get_slot_1().unwrap().purchase.name, "Overwritten");
    }
}
