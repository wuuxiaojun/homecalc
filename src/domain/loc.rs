use std::collections::BTreeMap;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use super::formula;

/// Monthly statement capturing financial details for a single billing period.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocMonthlyStatement {
    pub month_index: u32,
    pub date_label: String,
    pub start_balance: f64,
    pub extra_principal_paid: f64,
    pub interest_billed: f64,
    pub monthly_property_tax: f64,
    pub monthly_insurance: f64,
    pub total_outflow: f64,
    pub end_balance: f64,
}

/// SBLOC Housing Engine simulating Day 0 draw, monthly simple interest, escrow, and principal payments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocEngine {
    pub name: String,
    pub start_date: NaiveDate,
    pub initial_draw: f64,
    pub annual_rate: f64,
    pub property_tax_rate: f64,
    pub annual_insurance: f64,
    pub extra_payments: BTreeMap<u32, f64>,
    pub recurring_extra_principal: f64,
    pub schedule: Vec<LocMonthlyStatement>,
}

impl LocEngine {
    /// Creates a new `LocEngine` instance and calculates the initial amortization/statement schedule.
    pub fn new(
        name: impl Into<String>,
        start_date: NaiveDate,
        initial_draw: f64,
        annual_rate: f64,
        property_tax_rate: f64,
        annual_insurance: f64,
    ) -> Result<Self, String> {
        if initial_draw < 0.0 {
            return Err("Initial draw must be non-negative".to_string());
        }
        if annual_rate < 0.0 {
            return Err("Annual rate must be non-negative".to_string());
        }
        if property_tax_rate < 0.0 {
            return Err("Property tax rate must be non-negative".to_string());
        }
        if annual_insurance < 0.0 {
            return Err("Annual insurance must be non-negative".to_string());
        }

        let mut engine = Self {
            name: name.into(),
            start_date,
            initial_draw,
            annual_rate,
            property_tax_rate,
            annual_insurance,
            extra_payments: BTreeMap::new(),
            recurring_extra_principal: 0.0,
            schedule: Vec::new(),
        };

        engine.recalculate();
        Ok(engine)
    }

    /// Adds or updates a one-off extra principal payment for a specific month index.
    pub fn add_extra_payment(&mut self, month_index: u32, amount: f64) {
        self.extra_payments.insert(month_index, amount);
        self.recalculate();
    }

    /// Sets recurring extra principal amount applied on the 1st of every month.
    pub fn set_recurring_extra_payment(&mut self, amount: f64) {
        self.recurring_extra_principal = amount;
        self.recalculate();
    }

    /// Recalculates the monthly statement schedule from month 1 until balance hits 0 or max 360 months.
    pub fn recalculate(&mut self) {
        self.schedule.clear();

        if self.initial_draw <= 0.0 {
            return;
        }

        let mut balance = self.initial_draw;
        let mut month_index: u32 = 1;

        let start_total_months =
            self.start_date.year() * 12 + (self.start_date.month() - 1) as i32;

        while balance > 0.0 && month_index <= 360 {
            // 1. Determine extra principal payment for current month
            let one_off = self.extra_payments.get(&month_index).copied().unwrap_or(0.0);
            let requested_extra = (one_off + self.recurring_extra_principal).max(0.0);
            let extra_principal_paid = requested_extra.min(balance);

            // 2. Apply extra principal payment on Day 1
            let start_balance = balance;
            let balance_after_extra = (balance - extra_principal_paid).max(0.0);

            // 3. Determine current date label and calendar days in month
            let total_months = start_total_months + (month_index - 1) as i32;
            let current_year = total_months / 12;
            let current_month = (total_months % 12 + 1) as u32;

            let current_date = NaiveDate::from_ymd_opt(current_year, current_month, 1)
                .unwrap_or(self.start_date);
            let date_label = current_date.format("%b %Y").to_string();

            let days_in_month = formula::days_in_month(current_year, current_month);

            // 4. Calculate monthly simple interest billed
            let daily_rate = formula::daily_interest_rate(self.annual_rate);
            let interest_billed = balance_after_extra * daily_rate * (days_in_month as f64);

            // 5. Calculate monthly escrow
            let monthly_property_tax = (self.initial_draw * (self.property_tax_rate / 100.0)) / 12.0;
            let monthly_insurance = self.annual_insurance / 12.0;

            // 6. Record total outflow
            let total_outflow =
                interest_billed + monthly_property_tax + monthly_insurance + extra_principal_paid;

            // 7. Push statement
            self.schedule.push(LocMonthlyStatement {
                month_index,
                date_label,
                start_balance,
                extra_principal_paid,
                interest_billed,
                monthly_property_tax,
                monthly_insurance,
                total_outflow,
                end_balance: balance_after_extra,
            });

            // 8. Advance balance and terminate if paid off
            balance = balance_after_extra;
            if balance == 0.0 {
                break;
            }

            month_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_1_august_interest() {
        let start_date = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let engine =
            LocEngine::new("August Test", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();

        assert!(!engine.schedule.is_empty());
        let stmt1 = &engine.schedule[0];

        assert_eq!(stmt1.month_index, 1);
        assert_eq!(stmt1.date_label, "Aug 2026");
        assert_eq!(stmt1.start_balance, 1_500_000.0);
        assert_eq!(stmt1.extra_principal_paid, 0.0);
        assert_eq!(stmt1.end_balance, 1_500_000.0);

        // Month 1 (August) has 31 days. Interest = 1,500,000 * (0.06 / 365) * 31
        let expected_interest = 1_500_000.0 * (0.06 / 365.0) * 31.0;
        assert!(
            (stmt1.interest_billed - expected_interest).abs() < 1e-6,
            "Expected {}, got {}",
            expected_interest,
            stmt1.interest_billed
        );

        // Escrow checks
        let expected_tax = (1_500_000.0 * 0.012) / 12.0; // 1500.0
        let expected_insurance = 3600.0 / 12.0; // 300.0
        assert_eq!(stmt1.monthly_property_tax, expected_tax);
        assert_eq!(stmt1.monthly_insurance, expected_insurance);
        assert_eq!(
            stmt1.total_outflow,
            stmt1.interest_billed + expected_tax + expected_insurance
        );
    }

    #[test]
    fn test_extra_payment_month_1_reduces_interest() {
        let start_date = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let mut engine =
            LocEngine::new("Extra Payment Test", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();

        engine.add_extra_payment(1, 500_000.0);

        let stmt1 = &engine.schedule[0];
        assert_eq!(stmt1.month_index, 1);
        assert_eq!(stmt1.start_balance, 1_500_000.0);
        assert_eq!(stmt1.extra_principal_paid, 500_000.0);
        assert_eq!(stmt1.end_balance, 1_000_000.0);

        // Interest should be accrued on the reduced balance ($1,000,000) for 31 days
        let expected_interest = 1_000_000.0 * (0.06 / 365.0) * 31.0;
        assert!(
            (stmt1.interest_billed - expected_interest).abs() < 1e-6,
            "Expected {}, got {}",
            expected_interest,
            stmt1.interest_billed
        );
    }

    #[test]
    fn test_recurring_extra_principal_zero_balance_termination() {
        let start_date = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let mut engine = LocEngine::new(
            "Recurring Extra Test",
            start_date,
            1_500_000.0,
            6.0,
            1.2,
            3600.0,
        )
        .unwrap();

        engine.set_recurring_extra_payment(100_000.0);

        // 1.5M / 100k per month = 15 months to pay off
        assert_eq!(engine.schedule.len(), 15);

        let last_stmt = engine.schedule.last().unwrap();
        assert_eq!(last_stmt.month_index, 15);
        assert_eq!(last_stmt.start_balance, 100_000.0);
        assert_eq!(last_stmt.extra_principal_paid, 100_000.0);
        assert_eq!(last_stmt.end_balance, 0.0);

        // Verify balance steadily decreased
        for (idx, stmt) in engine.schedule.iter().enumerate() {
            let expected_start = 1_500_000.0 - (idx as f64 * 100_000.0);
            let expected_end = expected_start - 100_000.0;
            assert_eq!(stmt.start_balance, expected_start);
            assert_eq!(stmt.end_balance, expected_end);
        }
    }
}
