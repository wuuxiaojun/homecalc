//! tool.rs
//! Financial instruments
use serde::{Deserialize, Serialize};

/// Cash
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cash {
    pub amount: f64,
    pub rate: f64, // annual cash yield (e.g. 3.9%)
}

/// Mortgage
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Mortgage {
    pub amount: f64,
    pub rate: f64, // annual mortgage interest rate (e.g. 6.0%)
    pub term: u32, // term in years
}

/// Line of Credit (LOC)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Loc {
    pub amount: f64,
    pub rate: f64, // annual loc interest rate (e.g. 5.5%)
}

/// Enum type for financial tools
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Tool {
    Mortgage(Mortgage),
    Loc(Loc),
    Cash(Cash),
}
