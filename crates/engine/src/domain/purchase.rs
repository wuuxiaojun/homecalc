//! purchase.rs
//! Purchase specification

use super::house::House;
use super::tool::Tool;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Purchase information of real estate property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub name: String,
    pub house: House,
    pub tools: Vec<Tool>,
    pub mortgage_repay: BTreeMap<u32, f64>, // repayment schedule for mortgage principal
    pub loc_repay: BTreeMap<u32, f64>,      // repayment schedule for loc principal
}
