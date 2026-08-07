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
    pub property_tax_paid: f64,
    pub insurance_paid: f64,
    pub hoa_paid: f64,
    pub total_holding_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStatementRow {
    pub month: u32,
    pub cash: Option<CashStatement>,
    pub mortgage: Option<MortgageStatement>,
    pub loc: Option<LocStatement>,
    pub house: HouseStatement,
    pub total_interest_earned: f64,
    pub total_debt_paid: f64,
    pub total_extra_payment: f64,
    pub total_paid: f64, // debt + extra + housing - interest
    pub total_remaining_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyStatementRow {
    pub year: u32,
    pub total_interest_earned: f64, // after tax
    pub total_debt_paid: f64,
    pub total_tax_savings: f64,
    pub total_extra_payment: f64,
    pub total_paid: f64, // debt + extra + housing - interest - tax
    pub total_remaining_balance: f64,
}
