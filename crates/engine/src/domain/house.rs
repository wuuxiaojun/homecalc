//! house.rs
//! House information

use serde::{Deserialize, Serialize};

/// Real estate property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    pub purchase_price: f64,
    pub annual_property_tax_rate: f64, // in percentage, e.g. 1.2%
    pub annual_insurance: f64,
    pub monthly_hoa: f64,
}
