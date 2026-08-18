//! house.rs
//! House information

use serde::{Deserialize, Serialize};

/// Real estate property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_holding_costs() {
        let house = House {
            purchase_price: 1_200_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 2_400.0,
            monthly_hoa: 200.0,
        };

        // Monthly tax = 1.2M * 0.012 / 12 = 1200.0
        assert_eq!(house.monthly_property_tax(), 1200.0);
        // Monthly insurance = 2400 / 12 = 200.0
        assert_eq!(house.monthly_insurance(), 200.0);
        // Initial holding cost = 1200 + 200 + 200 = 1600.0
        assert_eq!(house.initial_monthly_holding_cost(), 1600.0);
    }

    #[test]
    fn test_house_zero_values() {
        let house = House {
            purchase_price: 0.0,
            annual_property_tax_rate: 0.0,
            annual_insurance: 0.0,
            monthly_hoa: 0.0,
        };

        assert_eq!(house.monthly_property_tax(), 0.0);
        assert_eq!(house.monthly_insurance(), 0.0);
        assert_eq!(house.initial_monthly_holding_cost(), 0.0);
    }
}
