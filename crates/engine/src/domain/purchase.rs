//! purchase.rs
//! Purchase specification

use super::house::House;
use super::tool::{Cash, Loc, Mortgage, Tool};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Purchase information of real estate property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_extract_tools_and_principal() {
        let house = House {
            purchase_price: 1_500_000.0,
            annual_property_tax_rate: 1.25,
            annual_insurance: 3_000.0,
            monthly_hoa: 150.0,
        };

        let purchase = Purchase {
            name: "Test Purchase".to_string(),
            house,
            tools: vec![
                Tool::Cash(Cash {
                    amount: 300_000.0,
                    rate: 4.0,
                }),
                Tool::Mortgage(Mortgage {
                    amount: 1_000_000.0,
                    rate: 6.5,
                    term: 30,
                }),
                Tool::Loc(Loc {
                    amount: 200_000.0,
                    rate: 7.0,
                }),
            ],
            mortgage_repay: BTreeMap::from([(12, 50_000.0)]),
            loc_repay: BTreeMap::from([(6, 20_000.0)]),
        };

        assert!(purchase.cash().is_some());
        assert_eq!(purchase.cash().unwrap().amount, 300_000.0);

        assert!(purchase.mortgage().is_some());
        assert_eq!(purchase.mortgage().unwrap().amount, 1_000_000.0);

        assert!(purchase.loc().is_some());
        assert_eq!(purchase.loc().unwrap().amount, 200_000.0);

        assert_eq!(purchase.total_principal(), 1_200_000.0);
    }

    #[test]
    fn test_purchase_no_borrowed_tools() {
        let house = House {
            purchase_price: 800_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 1_200.0,
            monthly_hoa: 0.0,
        };

        let purchase = Purchase {
            name: "Cash Only".to_string(),
            house,
            tools: vec![Tool::Cash(Cash {
                amount: 800_000.0,
                rate: 4.0,
            })],
            mortgage_repay: BTreeMap::new(),
            loc_repay: BTreeMap::new(),
        };

        assert_eq!(purchase.total_principal(), 0.0);
        assert!(purchase.mortgage().is_none());
        assert!(purchase.loc().is_none());
    }
}
