use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    pub purchase_price: f64,
    pub annual_property_tax_rate: f64,
    pub annual_insurance: f64,
    pub monthly_hoa: f64,
}
