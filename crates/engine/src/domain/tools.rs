use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cash {
    pub amount: f64,
    pub rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mortgage {
    pub amount: f64,
    pub rate: f64,
    pub term: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loc {
    pub amount: f64,
    pub rate: f64,
    pub date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tool {
    Mortgage(Mortgage),
    Loc(Loc),
    Cash(Cash),
}
