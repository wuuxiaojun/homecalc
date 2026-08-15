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

impl House {
    pub fn monthly_property_tax(&self) -> f64 {
        self.purchase_price * self.annual_property_tax_rate * 0.01 / 12.0
    }

    pub fn monthly_insurance(&self) -> f64 {
        self.annual_insurance / 12.0
    }

    pub fn initial_monthly_holding_cost(&self) -> f64 {
        self.monthly_property_tax() + self.monthly_insurance() + self.monthly_hoa
    }
}
