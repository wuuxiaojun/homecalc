//! purchase.rs
//! Purchase specification

use super::house::House;
use super::tool::{Cash, Loc, Mortgage, Tool};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Purchase information of real estate property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub name: String,
    pub house: House,
    pub tools: Vec<Tool>,
    pub mortgage_repay: BTreeMap<u32, f64>, // repayment schedule for mortgage principal
    pub loc_repay: BTreeMap<u32, f64>,      // repayment schedule for loc principal
}

impl Purchase {
    pub fn mortgage(&self) -> Option<&Mortgage> {
        self.tools.iter().find_map(|t| match t {
            Tool::Mortgage(m) => Some(m),
            _ => None,
        })
    }

    pub fn loc(&self) -> Option<&Loc> {
        self.tools.iter().find_map(|t| match t {
            Tool::Loc(l) => Some(l),
            _ => None,
        })
    }

    pub fn cash(&self) -> Option<&Cash> {
        self.tools.iter().find_map(|t| match t {
            Tool::Cash(c) => Some(c),
            _ => None,
        })
    }

    pub fn total_principal(&self) -> f64 {
        let mut total = 0.0;
        if let Some(m) = self.mortgage() {
            total += m.amount;
        }
        if let Some(l) = self.loc() {
            total += l.amount;
        }
        total
    }
}
