//! scenario.rs
//! Scenario
use super::purchase::Purchase;
use super::statement::{MonthlyStatementRow, TotalStatement, YearlyStatementRow};
use serde::{Deserialize, Serialize};

/// All-in-one purchase scenario
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub purchase: Purchase,
    pub monthly_statement: Vec<MonthlyStatementRow>,
    pub yearly_statement: Vec<YearlyStatementRow>,
    pub total_statement: TotalStatement,
}
