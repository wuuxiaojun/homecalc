//! statement.rs
//! Monthly, yearly and total statement

// Cash-related
#[derive(Debug, Clone)]
pub struct CashStatement {
    pub cash_now: f64,      // unused cash current amount
    pub cash_interest: f64, // current - initial
}

// Mortgage-related
#[derive(Debug, Clone)]
pub struct MortgageStatement {
    pub monthly_payment: f64, // mortgage pmt
    pub principal_paid: f64,  // principal amount in pmt
    pub interest_paid: f64,   // interest amount in pmt
    pub extra_payment: f64,
    pub remaining_balance: f64,
}

// Loc-related
#[derive(Debug, Clone)]
pub struct LocStatement {
    pub monthly_payment: f64,
    pub extra_payment: f64,
    pub remaining_balance: f64,
}

// House-related
#[derive(Debug, Clone)]
pub struct HouseStatement {
    pub monthly_property_tax: f64,
    pub monthly_insurance: f64,
    pub monthly_hoa: f64,
}

// Monthly statement
#[derive(Debug, Clone)]
pub struct MonthlyStatementRow {
    pub month: u32,
    pub cash: Option<CashStatement>,
    pub mortgage: Option<MortgageStatement>,
    pub loc: Option<LocStatement>,
    pub house: HouseStatement,
    pub total_debt_paid: f64, // monthly mortgage + loc required payment
    pub total_extra_payment: f64,
    pub total_holding_cost: f64,
    pub total_paid: f64, // debt + extra + holding - cash interest
    pub total_remaining_balance: f64,
}

// Yearly statement
#[derive(Debug, Clone)]
pub struct YearlyStatementRow {
    pub year: u32,
    pub annual_cash_interest: f64,
    pub annual_interest_paid: f64, // mortgage + loc interest
    pub annual_debt_paid: f64,     // annual required payment
    pub annual_tax_savings: f64,   // tax savings for mortgage interest
    pub annual_extra_payment: f64,
    pub annual_holding_cost: f64,
    pub annual_paid: f64, // debt + extra + holding - cash - tax savings
    pub ending_remaining_balance: f64,
}

// Aggregate for all
#[derive(Debug, Clone)]
pub struct TotalStatement {
    pub total_cash_interest: f64,
    pub total_holding_cost: f64,
    pub total_interest_paid: f64,
    pub total_tax_savings: f64,
    pub total_paid: f64,
}
