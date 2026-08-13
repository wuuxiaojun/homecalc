//! analysis.rs
//! Single scenario analysis module

use crate::domain::scenario::Scenario;
use crate::domain::tool::Tool;

// Single scenario metrics
#[derive(Debug, Clone)]
pub struct ScenarioAnalysis {
    pub waste_ratio: f64,       // interest paid / principal borrowed
    pub tax_savings_ratio: f64, //tax savings / interest paid
    pub payoff_month: u32,
    pub effective_monthly_cost: f64, // total paid / payoff month
}

// Computes single-scenario analysis metrics
pub fn analyze_scenario(scenario: &Scenario) -> ScenarioAnalysis {
    let monthly = &scenario.monthly_statement;
    let total = &scenario.total_statement;
    let purchase = &scenario.purchase;
    let mut principal = 0.0;
    for tool in &purchase.tools {
        match tool {
            Tool::Mortgage(mortgage) => principal += mortgage.amount,
            Tool::Loc(loc) => principal += loc.amount,
            _ => {}
        }
    }

    let waste_ratio = if principal > 0.0 {
        total.total_interest_paid / principal
    } else {
        0.0
    };

    let tax_savings_ratio = if total.total_interest_paid > 0.0 {
        total.total_tax_savings / total.total_interest_paid
    } else {
        0.0
    };

    let payoff_month = monthly.last().map_or(0, |r| r.month);

    let effective_monthly_cost = if payoff_month > 0 {
        total.total_paid / payoff_month as f64
    } else {
        0.0
    };

    ScenarioAnalysis {
        waste_ratio,
        tax_savings_ratio,
        payoff_month,
        effective_monthly_cost,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::house::House;
    use crate::domain::purchase::Purchase;
    use crate::domain::statement::{HouseStatement, MonthlyStatementRow, TotalStatement};
    use crate::domain::tool::{Cash, Mortgage};
    use std::collections::BTreeMap;

    #[test]
    fn test_analyze_scenario_zero_principal() {
        let scenario = Scenario {
            purchase: Purchase {
                name: "Cash Only".to_string(),
                house: House {
                    purchase_price: 500_000.0,
                    annual_property_tax_rate: 1.2,
                    annual_insurance: 1_200.0,
                    monthly_hoa: 50.0,
                },
                tools: vec![Tool::Cash(Cash {
                    amount: 500_000.0,
                    rate: 4.0,
                })],
                mortgage_repay: BTreeMap::new(),
                loc_repay: BTreeMap::new(),
            },
            monthly_statement: vec![],
            yearly_statement: vec![],
            total_statement: TotalStatement {
                total_cash_interest: 20_000.0,
                total_holding_cost: 10_000.0,
                total_interest_paid: 0.0,
                total_tax_savings: 0.0,
                total_paid: 10_000.0,
            },
        };

        let analysis = analyze_scenario(&scenario);
        assert_eq!(analysis.waste_ratio, 0.0);
        assert_eq!(analysis.tax_savings_ratio, 0.0);
        assert_eq!(analysis.payoff_month, 0);
        assert_eq!(analysis.effective_monthly_cost, 0.0);
    }

    #[test]
    fn test_analyze_scenario_metrics() {
        let mock_monthly_row = MonthlyStatementRow {
            month: 120,
            cash: None,
            mortgage: None,
            loc: None,
            house: HouseStatement {
                monthly_property_tax: 500.0,
                monthly_insurance: 100.0,
                monthly_hoa: 50.0,
            },
            total_debt_paid: 4000.0,
            total_extra_payment: 0.0,
            total_holding_cost: 650.0,
            total_paid: 4650.0,
            total_remaining_balance: 0.0,
        };

        let scenario = Scenario {
            purchase: Purchase {
                name: "Mortgage Purchase".to_string(),
                house: House {
                    purchase_price: 600_000.0,
                    annual_property_tax_rate: 1.0,
                    annual_insurance: 1_200.0,
                    monthly_hoa: 50.0,
                },
                tools: vec![Tool::Mortgage(Mortgage {
                    amount: 500_000.0,
                    rate: 6.0,
                    term: 10,
                })],
                mortgage_repay: BTreeMap::new(),
                loc_repay: BTreeMap::new(),
            },
            monthly_statement: vec![mock_monthly_row],
            yearly_statement: vec![],
            total_statement: TotalStatement {
                total_cash_interest: 0.0,
                total_holding_cost: 78_000.0,
                total_interest_paid: 100_000.0,
                total_tax_savings: 20_000.0,
                total_paid: 600_000.0,
            },
        };

        let analysis = analyze_scenario(&scenario);
        assert_eq!(analysis.waste_ratio, 100_000.0 / 500_000.0); // 0.2
        assert_eq!(analysis.tax_savings_ratio, 20_000.0 / 100_000.0); // 0.2
        assert_eq!(analysis.payoff_month, 120);
        assert_eq!(analysis.effective_monthly_cost, 600_000.0 / 120.0); // 5000.0
    }
}
