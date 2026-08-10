use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashStatement {
    pub cash_now: f64,
    pub interest_earned: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageStatement {
    pub monthly_payment: f64,
    pub principal_paid: f64,
    pub interest_paid: f64,
    pub extra_payment: f64,
    pub remaining_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocStatement {
    pub monthly_payment: f64,
    pub extra_payment: f64,
    pub remaining_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseStatement {
    pub monthly_property_tax: f64,
    pub monthly_insurance: f64,
    pub monthly_hoa: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStatementRow {
    pub month: u32,
    pub cash: Option<CashStatement>,
    pub mortgage: Option<MortgageStatement>,
    pub loc: Option<LocStatement>,
    pub house: HouseStatement,
    pub total_debt_paid: f64,
    pub total_extra_payment: f64,
    pub total_holding_cost: f64,
    pub total_paid: f64, // debt + extra + holding - interest
    pub total_remaining_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyStatementRow {
    pub year: u32,
    pub annual_cash_interest: f64,
    pub annual_interest_paid: f64,
    pub annual_debt_paid: f64,
    pub annual_tax_savings: f64,
    pub annual_extra_payment: f64,
    pub annual_holding_cost: f64,
    pub annual_paid: f64, // debt + extra + holding - interest - tax
    pub ending_remaining_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub payoff_month: u32,
    pub total_cash_interest: f64,
    pub total_holding_cost: f64,
    pub total_interest_paid: f64,
    pub total_tax_savings: f64,
    pub total_paid: f64,
}
