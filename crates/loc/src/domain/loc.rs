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
    pub interest_billed: f64,
    pub tax_and_insurance: f64,
    pub monthly_property_tax: f64,
    pub monthly_insurance: f64,
    pub extra_principal_paid: f64,
    pub total_outflow: f64,
    pub end_balance: f64,
}

/// Annual summary rollup aggregating monthly statements for a calendar year.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocAnnualSummary {
    pub year_label: String,
    pub start_balance: f64,
    pub total_interest_paid: f64,
    pub total_tax_and_insurance_paid: f64,
    pub total_extra_principal_paid: f64,
    pub total_outflow: f64,
    pub end_balance: f64,
}

/// LOC Housing Engine simulating Day 0 draw, monthly simple interest, tax + insurance, and principal payments.
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
    /// Creates a new `LocEngine` instance and calculates the initial statement schedule.
    pub fn new(
        name: impl Into<String>,
        start_date: NaiveDate,
        initial_draw: f64,
        annual_rate: f64,
        property_tax_rate: f64,
        annual_insurance: f64,
    ) -> Result<Self, String> {
        if initial_draw <= 0.0 {
            return Err("Initial draw must be positive".to_string());
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

    /// Returns the monthly property tax charge based on initial draw and property tax rate.
    pub fn monthly_property_tax(&self) -> f64 {
        (self.initial_draw * (self.property_tax_rate / 100.0)) / 12.0
    }

    /// Returns the monthly insurance cost based on annual insurance.
    pub fn monthly_insurance(&self) -> f64 {
        self.annual_insurance / 12.0
    }

    /// Returns the combined monthly property tax and insurance cost.
    pub fn monthly_tax_and_insurance(&self) -> f64 {
        self.monthly_property_tax() + self.monthly_insurance()
    }

    /// Returns the combined annual property tax and insurance cost.
    pub fn annual_tax_and_insurance(&self) -> f64 {
        (self.initial_draw * (self.property_tax_rate / 100.0)) + self.annual_insurance
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
            self.start_date.year() as i64 * 12 + (self.start_date.month() - 1) as i64;

        while balance > 0.0 && month_index <= 360 {
            // 1. Determine extra principal payment for current month
            let one_off = self.extra_payments.get(&month_index).copied().unwrap_or(0.0);
            let requested_extra = (one_off + self.recurring_extra_principal).max(0.0);
            let extra_principal_paid = requested_extra.min(balance);

            // 2. Apply extra principal payment on Day 1
            let start_balance = balance;
            let balance_after_extra = (balance - extra_principal_paid).max(0.0);

            // 3. Determine current date label and calendar days in month
            let total_months = start_total_months + (month_index - 1) as i64;
            let current_year = total_months.div_euclid(12) as i32;
            let current_month = (total_months.rem_euclid(12) + 1) as u32;

            let current_date = NaiveDate::from_ymd_opt(current_year, current_month, 1)
                .unwrap_or(self.start_date);
            let date_label = current_date.format("%b %Y").to_string();

            let days_in_month = formula::days_in_month(current_year, current_month);

            // 4. Calculate monthly simple interest billed
            let daily_rate = formula::daily_interest_rate(self.annual_rate);
            let interest_billed = balance_after_extra * daily_rate * (days_in_month as f64);

            // 5. Calculate monthly property tax & insurance
            let monthly_property_tax = self.monthly_property_tax();
            let monthly_insurance = self.monthly_insurance();
            let tax_and_insurance = self.monthly_tax_and_insurance();

            // 6. Record total outflow
            let total_outflow =
                interest_billed + tax_and_insurance + extra_principal_paid;

            // 7. Push statement
            self.schedule.push(LocMonthlyStatement {
                month_index,
                date_label,
                start_balance,
                interest_billed,
                tax_and_insurance,
                monthly_property_tax,
                monthly_insurance,
                extra_principal_paid,
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

    /// Aggregates monthly statements into annual summaries grouped by calendar year.
    pub fn annual_summaries(&self) -> Vec<LocAnnualSummary> {
        let mut summaries = Vec::new();
        if self.schedule.is_empty() {
            return summaries;
        }

        let mut current_year: Option<String> = None;
        let mut current_statements: Vec<&LocMonthlyStatement> = Vec::new();

        for stmt in &self.schedule {
            let year = stmt
                .date_label
                .split_whitespace()
                .last()
                .unwrap_or("Unknown")
                .to_string();

            if let Some(ref cy) = current_year {
                if cy != &year {
                    summaries.push(Self::build_annual_summary(cy, &current_statements));
                    current_year = Some(year);
                    current_statements = vec![stmt];
                } else {
                    current_statements.push(stmt);
                }
            } else {
                current_year = Some(year);
                current_statements.push(stmt);
            }
        }

        if let Some(cy) = current_year.filter(|_| !current_statements.is_empty()) {
            summaries.push(Self::build_annual_summary(&cy, &current_statements));
        }

        summaries
    }

    fn build_annual_summary(year_label: &str, stmts: &[&LocMonthlyStatement]) -> LocAnnualSummary {
        let start_balance = stmts.first().map(|s| s.start_balance).unwrap_or(0.0);
        let end_balance = stmts.last().map(|s| s.end_balance).unwrap_or(0.0);
        let total_interest_paid = stmts.iter().map(|s| s.interest_billed).sum();
        let total_tax_and_insurance_paid = stmts.iter().map(|s| s.tax_and_insurance).sum();
        let total_extra_principal_paid = stmts.iter().map(|s| s.extra_principal_paid).sum();
        let total_outflow = stmts.iter().map(|s| s.total_outflow).sum();

        LocAnnualSummary {
            year_label: year_label.to_string(),
            start_balance,
            total_interest_paid,
            total_tax_and_insurance_paid,
            total_extra_principal_paid,
            total_outflow,
            end_balance,
        }
    }

    /// Saves the current `LocEngine` scenario configuration to a JSON file on disk.
    pub fn save_to_json(&self, dir_path: &str, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(dir_path)?;

        let clean_filename = if filename.to_lowercase().ends_with(".json") {
            filename.to_string()
        } else {
            format!("{}.json", filename)
        };

        let file_path = std::path::Path::new(dir_path).join(clean_filename);
        let json_data = serde_json::to_string_pretty(self)?;
        std::fs::write(&file_path, json_data)?;

        Ok(file_path.to_string_lossy().to_string())
    }

    /// Loads a `LocEngine` scenario configuration from a JSON file on disk.
    pub fn load_from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(filepath)?;
        let mut engine: LocEngine = serde_json::from_str(&content)?;
        engine.recalculate();
        Ok(engine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_month_full_payoff() {
        let start_date = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let mut engine =
            LocEngine::new("Full Payoff Test", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();

        engine.add_extra_payment(1, 1_500_000.0);

        assert_eq!(engine.schedule.len(), 1);
        let stmt1 = &engine.schedule[0];
        assert_eq!(stmt1.month_index, 1);
        assert_eq!(stmt1.start_balance, 1_500_000.0);
        assert_eq!(stmt1.extra_principal_paid, 1_500_000.0);
        assert_eq!(stmt1.end_balance, 0.0);
    }

    #[test]
    fn test_overpaying_principal_caps_and_no_negative_balance() {
        let start_date = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let mut engine =
            LocEngine::new("Overpay Test", start_date, 1_500_000.0, 6.0, 1.2, 3600.0).unwrap();

        // Extra payment of $2.0M on a $1.5M balance
        engine.add_extra_payment(1, 2_000_000.0);

        assert_eq!(engine.schedule.len(), 1);
        let stmt1 = &engine.schedule[0];
        assert_eq!(stmt1.month_index, 1);
        assert_eq!(stmt1.start_balance, 1_500_000.0);
        assert_eq!(stmt1.extra_principal_paid, 1_500_000.0);
        assert_eq!(stmt1.end_balance, 0.0);
    }

    #[test]
    fn test_daily_interest_calculation_across_differing_month_lengths() {
        // Test leap year Feb 2028 (29 days), Mar 2028 (31 days), Apr 2028 (30 days)
        let start_date = NaiveDate::from_ymd_opt(2028, 2, 1).unwrap();
        let engine =
            LocEngine::new("Leap Intersect Test", start_date, 1_000_000.0, 7.3, 1.0, 1200.0).unwrap();

        assert!(engine.schedule.len() >= 3);

        // Month 1: Feb 2028 (29 days)
        let stmt_feb = &engine.schedule[0];
        assert_eq!(stmt_feb.date_label, "Feb 2028");
        let expected_feb_interest = 1_000_000.0 * (0.073 / 365.0) * 29.0;
        assert!(
            (stmt_feb.interest_billed - expected_feb_interest).abs() < 1e-6,
            "Feb 2028 (29 days) interest failed: got {}, expected {}",
            stmt_feb.interest_billed,
            expected_feb_interest
        );

        // Month 2: Mar 2028 (31 days)
        let stmt_mar = &engine.schedule[1];
        assert_eq!(stmt_mar.date_label, "Mar 2028");
        let expected_mar_interest = 1_000_000.0 * (0.073 / 365.0) * 31.0;
        assert!(
            (stmt_mar.interest_billed - expected_mar_interest).abs() < 1e-6,
            "Mar 2028 (31 days) interest failed: got {}, expected {}",
            stmt_mar.interest_billed,
            expected_mar_interest
        );

        // Month 3: Apr 2028 (30 days)
        let stmt_apr = &engine.schedule[2];
        assert_eq!(stmt_apr.date_label, "Apr 2028");
        let expected_apr_interest = 1_000_000.0 * (0.073 / 365.0) * 30.0;
        assert!(
            (stmt_apr.interest_billed - expected_apr_interest).abs() < 1e-6,
            "Apr 2028 (30 days) interest failed: got {}, expected {}",
            stmt_apr.interest_billed,
            expected_apr_interest
        );

        // Test non-leap year Feb 2027 (28 days)
        let start_date_2027 = NaiveDate::from_ymd_opt(2027, 2, 1).unwrap();
        let engine_2027 =
            LocEngine::new("Non-Leap Test", start_date_2027, 1_000_000.0, 7.3, 1.0, 1200.0).unwrap();
        let stmt_feb_2027 = &engine_2027.schedule[0];
        assert_eq!(stmt_feb_2027.date_label, "Feb 2027");
        let expected_feb_2027_interest = 1_000_000.0 * (0.073 / 365.0) * 28.0;
        assert!(
            (stmt_feb_2027.interest_billed - expected_feb_2027_interest).abs() < 1e-6,
            "Feb 2027 (28 days) interest failed: got {}, expected {}",
            stmt_feb_2027.interest_billed,
            expected_feb_2027_interest
        );
    }

    #[test]
    fn test_zero_extra_payment_runs_to_360_months() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let engine =
            LocEngine::new("Zero Extra Test", start_date, 1_500_000.0, 12.5, 1.2, 3600.0).unwrap();

        assert_eq!(engine.schedule.len(), 360);
        assert_eq!(engine.schedule.first().unwrap().month_index, 1);
        assert_eq!(engine.schedule.last().unwrap().month_index, 360);
        assert_eq!(engine.schedule.last().unwrap().end_balance, 1_500_000.0);
    }

    #[test]
    fn test_json_roundtrip_bit_exact_schedule() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 27).unwrap();
        let mut engine = LocEngine::new("JSON Bit-Exact Scenario", start_date, 1_500_000.0, 6.5, 1.2, 3600.0).unwrap();
        engine.add_extra_payment(3, 50_000.0);
        engine.add_extra_payment(12, 100_000.0);
        engine.add_extra_payment(24, 250_000.0);
        engine.set_recurring_extra_payment(5_000.0);

        let temp_dir = std::env::temp_dir().join("loc_test_bit_exact");
        let path_str = engine.save_to_json(&temp_dir.to_string_lossy(), "test_loc_bitexact").unwrap();

        let loaded = LocEngine::load_from_json(&path_str).unwrap();
        assert_eq!(loaded.name, engine.name);
        assert_eq!(loaded.initial_draw, engine.initial_draw);
        assert_eq!(loaded.extra_payments, engine.extra_payments);
        assert_eq!(loaded.recurring_extra_principal, engine.recurring_extra_principal);
        assert_eq!(loaded.schedule, engine.schedule);

        let _ = std::fs::remove_file(path_str);
        let _ = std::fs::remove_dir(temp_dir);
    }

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

        // Tax & Insurance checks
        let expected_tax = (1_500_000.0 * 0.012) / 12.0; // 1500.0
        let expected_insurance = 3600.0 / 12.0; // 300.0
        assert_eq!(stmt1.tax_and_insurance, expected_tax + expected_insurance);
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

    #[test]
    fn test_annual_summaries_grouping() {
        let start_date = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();
        let mut engine = LocEngine::new(
            "Annual Summary Test",
            start_date,
            1_500_000.0,
            6.0,
            1.2,
            3600.0,
        )
        .unwrap();
        engine.set_recurring_extra_payment(100_000.0);

        let annual = engine.annual_summaries();
        assert_eq!(annual.len(), 2);
        assert_eq!(annual[0].year_label, "2026");
        assert_eq!(annual[0].start_balance, 1_500_000.0);
        assert_eq!(annual[0].end_balance, 1_000_000.0);

        assert_eq!(annual[1].year_label, "2027");
        assert_eq!(annual[1].start_balance, 1_000_000.0);
        assert_eq!(annual[1].end_balance, 0.0);
    }

    #[test]
    fn test_domain_helper_methods() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let engine = LocEngine::new("Helper Test", start_date, 1_200_000.0, 5.0, 1.2, 3600.0).unwrap();

        // 1,200,000 * 0.012 / 12 = 1200.0
        assert_eq!(engine.monthly_property_tax(), 1200.0);
        // 3600.0 / 12 = 300.0
        assert_eq!(engine.monthly_insurance(), 300.0);
        // 1200 + 300 = 1500.0
        assert_eq!(engine.monthly_tax_and_insurance(), 1500.0);
        // 1500 * 12 = 18000.0
        assert_eq!(engine.annual_tax_and_insurance(), 18000.0);
    }

    #[test]
    fn test_initial_draw_guard_check() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        assert!(LocEngine::new("Zero Draw", start_date, 0.0, 5.0, 1.2, 3600.0).is_err());
        assert!(LocEngine::new("Negative Draw", start_date, -100.0, 5.0, 1.2, 3600.0).is_err());
        let res = LocEngine::new("Zero Draw Msg", start_date, 0.0, 5.0, 1.2, 3600.0);
        assert_eq!(res.unwrap_err(), "Initial draw must be positive");
    }

    #[test]
    fn test_recalculate_handles_zero_balances_and_boundary_months() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let mut engine = LocEngine::new("Boundary Test", start_date, 100_000.0, 5.0, 1.2, 3600.0).unwrap();

        // Add extra payment for month > 360
        engine.add_extra_payment(400, 50_000.0);
        // Schedule should still cap at max 360 months or payoff
        assert!(engine.schedule.len() <= 360);

        // Clear schedule and test empty schedule handling
        engine.schedule.clear();
        assert!(engine.annual_summaries().is_empty());
    }
}
