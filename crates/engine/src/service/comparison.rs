use serde::{Deserialize, Serialize};
use crate::domain::scenario::Scenario;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScenarioComparison {
    // 1. Timeline
    pub baseline_payoff_month: u32,
    pub alternative_payoff_month: u32,
    pub months_saved: i32,

    // 2. Outflows
    pub baseline_monthly_payment: f64,
    pub alternative_monthly_payment: f64,
    pub delta_monthly_payment: f64,
    pub baseline_extra_payment: f64,
    pub alternative_extra_payment: f64,
    pub delta_extra_payment: f64,
    pub baseline_interest_paid: f64,
    pub alternative_interest_paid: f64,
    pub delta_interest_paid: f64,

    // 3. Inflows
    pub baseline_cash_interest: f64,
    pub alternative_cash_interest: f64,
    pub delta_cash_interest: f64,
    pub baseline_tax_savings: f64,
    pub alternative_tax_savings: f64,
    pub delta_tax_savings: f64,

    // 4. Aggregate
    pub baseline_gross_paid: f64,
    pub alternative_gross_paid: f64,
    pub delta_gross_paid: f64,

    // 5. Internal Rate of Return

    IRR,
    PV,

}
