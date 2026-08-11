// analysis.rs
// Single Purchase Analysis

use crate::domain::scenario::Scenario;
use crate::domain::tool::Tool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScenarioAnalysis {
    // 1. Efficiency & Friction Ratios
    pub waste_ratio: f64,
    pub tax_savings_ratio: f64,

    // 2. Speed & Capital Acceleration
    pub payoff_month: u32,
    pub effective_monthly_cost: f64,
}

pub fn compute_scenario_analysis(scenario: &Scenario) -> ScenarioAnalysis {
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
