// src/mortgage.rs

use crate::formula::monthly_payment;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::BufReader;

// Enum to check parameter input error
#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    InvalidPrice(String),
    InvalidDown(String),
    InvalidRate(String),
    InvalidTerm(String),
    InvalidTaxRate(String),
    InvalidInsurance(String),
}

// Payment struct representing a single month's breakdown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    pub month: u32,
    pub total_p_i: f64,     // Principal + Interest payment
    pub principal: f64,     // Scheduled principal
    pub extra: f64,         // Extra principal paid
    pub interest: f64,      // Interest paid
    pub escrow: f64,        // Monthly Tax + Insurance contribution
    pub total_outflow: f64, // Total PITI (Principal, Interest, Tax, Insurance)
    pub balance: f64,       // Remaining loan balance
}

// AnnualSummary struct representing a year-by-year rollup
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnualSummary {
    pub year: u32,
    pub principal_paid: f64,
    pub extra_principal_paid: f64,
    pub interest_paid: f64,
    pub escrow_paid: f64,
    pub total_outflow: f64,
    pub year_end_balance: f64,
}

// Mortgage struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mortgage {
    pub name: String,
    pub price: f64,
    pub down: f64,
    pub loan: f64,                          // Total loan amount
    pub rate: f64,                          // Rate in percentage (e.g., 5.9)
    pub term: u32,                          // Term in years
    pub tax_rate: f64,                      // Annual property tax rate in % (e.g., 1.2)
    pub annual_insurance: f64,              // Annual homeowners insurance ($)
    pub base_payment: f64,                  // Base P&I payment each month
    pub extra_payments: BTreeMap<u32, f64>, // Extra payment schedule
    pub schedule: BTreeMap<u32, Payment>,    // Complete schedule
}

impl Mortgage {
    /// Construct new mortgage with Tax, Insurance, and Safeguard Validation
    pub fn new(
        name: String,
        price: f64,
        down: f64,
        rate: f64,
        term: u32,
        tax_rate: f64,
        annual_insurance: f64,
    ) -> Result<Self, InputError> {
        // Check Home Price
        if price <= 0.0 || price.is_nan() || price.is_infinite() {
            return Err(InputError::InvalidPrice(
                "Price must be greater than 0.".to_string(),
            ));
        }
        // Check Down Payment
        if down < 0.0 || down.is_nan() || down.is_infinite() {
            return Err(InputError::InvalidDown(
                "Down payment cannot be negative.".to_string(),
            ));
        }
        if down > price {
            return Err(InputError::InvalidDown(format!(
                "Down payment (${:.2}) cannot exceed home price (${:.2}).",
                down, price
            )));
        }
        // Check Interest Rate
        if !(0.0..=100.0).contains(&rate) {
            return Err(InputError::InvalidRate(
                "Interest rate must be between 0% and 100%.".to_string(),
            ));
        }
        // Check Loan Term
        if !(1..=30).contains(&term) {
            return Err(InputError::InvalidTerm(
                "Loan term must be between 1 and 30 years.".to_string(),
            ));
        }
        // Check Tax Rate
        if !(0.0..=20.0).contains(&tax_rate) {
            return Err(InputError::InvalidTaxRate(
                "Property tax rate must be between 0% and 20%.".to_string(),
            ));
        }
        // Check Annual Insurance
        if annual_insurance < 0.0 || annual_insurance.is_nan() || annual_insurance.is_infinite() {
            return Err(InputError::InvalidInsurance(
                "Annual insurance cannot be negative.".to_string(),
            ));
        }

        let mut mortgage = Mortgage {
            name,
            price,
            down,
            loan: price - down,
            rate,
            term,
            tax_rate,
            annual_insurance,
            base_payment: monthly_payment(price - down, rate / 100.0, term),
            extra_payments: BTreeMap::new(),
            schedule: BTreeMap::new(),
        };

        mortgage.recalculate();
        Ok(mortgage)
    }

    /// Monthly Property Tax ($)
    pub fn monthly_tax(&self) -> f64 {
        (self.price * (self.tax_rate / 100.0)) / 12.0
    }

    /// Monthly Homeowners Insurance ($)
    pub fn monthly_insurance(&self) -> f64 {
        self.annual_insurance / 12.0
    }

    /// Monthly Escrow Obligation (Tax + Insurance)
    pub fn monthly_escrow(&self) -> f64 {
        self.monthly_tax() + self.monthly_insurance()
    }

    /// Estimated Closing Day Prepaids:
    /// - 12 Months Upfront Insurance
    /// - 2 Months Tax Buffer + 2 Months Insurance Buffer Reserve
    pub fn closing_prepaids(&self) -> (f64, f64, f64) {
        let upfront_insurance = self.annual_insurance; // 1 full year upfront
        let tax_buffer = self.monthly_tax() * 2.0; // 2 months tax buffer
        let ins_buffer = self.monthly_insurance() * 2.0; // 2 months insurance buffer

        let total_prepaids = upfront_insurance + tax_buffer + ins_buffer;
        (upfront_insurance, tax_buffer + ins_buffer, total_prepaids)
    }

    /// Recalculate the amortization schedule internally
    pub fn recalculate(&mut self) {
        self.schedule.clear();
        let monthly_rate: f64 = self.rate / 100.0 / 12.0;
        let total_months: u32 = self.term * 12;
        let escrow_amount = self.monthly_escrow();
        let mut balance: f64 = self.loan;

        for month in 1..=total_months {
            if balance <= 1e-6 {
                break;
            }

            let interest_due: f64 = balance * monthly_rate;
            let mut principal_due: f64 = self.base_payment - interest_due;

            if balance <= principal_due {
                principal_due = balance;
                let total_pi = principal_due + interest_due;
                self.schedule.insert(
                    month,
                    Payment {
                        month,
                        total_p_i: total_pi,
                        principal: principal_due,
                        extra: 0.0,
                        interest: interest_due,
                        escrow: escrow_amount,
                        total_outflow: total_pi + escrow_amount,
                        balance: 0.0,
                    },
                );
                break;
            }

            let extra_input: f64 = self.extra_payments.get(&month).copied().unwrap_or(0.0);
            let max_extra: f64 = balance - principal_due;
            let extra: f64 = extra_input.clamp(0.0, max_extra);
            let total_principal: f64 = principal_due + extra;

            balance -= total_principal;
            if balance < 1e-6 {
                balance = 0.0;
            }

            let total_pi = total_principal + interest_due;
            self.schedule.insert(
                month,
                Payment {
                    month,
                    total_p_i: total_pi,
                    principal: principal_due,
                    extra,
                    interest: interest_due,
                    escrow: escrow_amount,
                    total_outflow: total_pi + escrow_amount,
                    balance,
                },
            );
        }
    }

    /// Add extra principal payment with validation & bounds checking
    pub fn add_extra_payment(&mut self, month: u32, extra: f64) -> Result<f64, String> {
        let max_month = self.term * 12;
        if month < 1 || month > max_month {
            return Err(format!(
                "Invalid month: {}. Month must be between 1 and {}.",
                month, max_month
            ));
        }

        if extra <= 0.0 || extra.is_nan() {
            return Err("Extra payment amount must be greater than $0.00.".to_string());
        }

        let current = match self.schedule.get(&month) {
            Some(payment) => payment,
            None => {
                return Ok(0.0);
            }
        };

        let max_extra = current.balance + current.extra;
        if max_extra <= 0.0 {
            return Ok(0.0);
        }

        let actual_extra = if extra > max_extra {
            max_extra
        } else {
            extra
        };

        self.extra_payments.insert(month, actual_extra);
        self.recalculate();
        Ok(actual_extra)
    }

    /// Calculate annual aggregation summaries by grouping monthly schedule entries into yearly buckets
    pub fn annual_summaries(&self) -> Vec<AnnualSummary> {
        let total_entries = self.schedule.len();
        if total_entries == 0 {
            return Vec::new();
        }

        let mut summaries = Vec::new();
        let mut current_year = 0;
        let mut principal_paid = 0.0;
        let mut extra_principal_paid = 0.0;
        let mut interest_paid = 0.0;
        let mut escrow_paid = 0.0;
        let mut total_outflow = 0.0;
        let mut year_end_balance = 0.0;

        for (idx, (&m, payment)) in self.schedule.iter().enumerate() {
            let year = ((m - 1) / 12) + 1;

            if current_year != year {
                if current_year != 0 {
                    summaries.push(AnnualSummary {
                        year: current_year,
                        principal_paid,
                        extra_principal_paid,
                        interest_paid,
                        escrow_paid,
                        total_outflow,
                        year_end_balance,
                    });
                }
                current_year = year;
                principal_paid = 0.0;
                extra_principal_paid = 0.0;
                interest_paid = 0.0;
                escrow_paid = 0.0;
                total_outflow = 0.0;
            }

            principal_paid += payment.principal;
            extra_principal_paid += payment.extra;
            interest_paid += payment.interest;
            escrow_paid += payment.escrow;
            total_outflow += payment.total_outflow;
            year_end_balance = payment.balance;

            if idx == total_entries - 1 {
                summaries.push(AnnualSummary {
                    year: current_year,
                    principal_paid,
                    extra_principal_paid,
                    interest_paid,
                    escrow_paid,
                    total_outflow,
                    year_end_balance,
                });
            }
        }

        summaries
    }

    /// Returns the first month where scheduled monthly principal is greater than or equal to monthly interest
    pub fn crossover_month(&self) -> Option<u32> {
        for (&m, payment) in &self.schedule {
            if payment.principal >= payment.interest {
                return Some(m);
            }
        }
        None
    }

    /// Returns the first month where remaining balance is less than or equal to 50% of the initial loan amount
    pub fn half_equity_month(&self) -> Option<u32> {
        let half_loan = self.loan / 2.0;

        for (&m, payment) in &self.schedule {
            if payment.balance <= half_loan {
                return Some(m);
            }
        }
        None
    }

    /// Serializes Mortgage struct to a JSON file inside dir_path directory
    pub fn save_to_json(&self, dir_path: &str, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
        fs::create_dir_all(dir_path)?;

        let filename_clean = if filename.ends_with(".json") {
            filename.to_string()
        } else {
            format!("{}.json", filename)
        };

        let filepath = format!("{}/{}", dir_path, filename_clean);
        let json_str = serde_json::to_string_pretty(self)?;
        fs::write(&filepath, json_str)?;

        Ok(filepath)
    }

    /// Deserializes a JSON file into a Mortgage struct and recalculates its schedule
    pub fn load_from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let mut mortgage: Mortgage = serde_json::from_reader(reader)?;
        mortgage.recalculate();
        Ok(mortgage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_phase3_escrow_calculations() {
        let mortgage = Mortgage::new("Test Loan".to_string(), 1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();

        // $1.5M * 1.2% / 12 = $1,500/mo tax
        assert_eq!(mortgage.monthly_tax(), 1_500.0);
        // $3,600 / 12 = $300/mo insurance
        assert_eq!(mortgage.monthly_insurance(), 300.0);
        // Escrow = $1,800/mo
        assert_eq!(mortgage.monthly_escrow(), 1_800.0);
    }

    #[test]
    fn test_annual_summaries_standard_loan() {
        let mortgage = Mortgage::new("Standard Loan".to_string(), 1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();
        let summaries = mortgage.annual_summaries();

        // 1. Verify 15-year loan generates 15 annual summaries
        assert_eq!(summaries.len(), 15);
        assert_eq!(summaries[0].year, 1);
        assert_eq!(summaries[14].year, 15);
        assert_eq!(summaries[14].year_end_balance, 0.0);

        // 2. Verify summing interest across all AnnualSummary structs matches total interest in schedule
        let annual_interest_sum: f64 = summaries.iter().map(|s| s.interest_paid).sum();
        let schedule_interest_sum: f64 = mortgage.schedule.values().map(|p| p.interest).sum();
        assert!((annual_interest_sum - schedule_interest_sum).abs() < 1e-6);
    }

    #[test]
    fn test_annual_summaries_early_payoff() {
        let mut mortgage = Mortgage::new("Accelerated Loan".to_string(), 1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();
        let _ = mortgage.add_extra_payment(1, 50_000.0);
        let _ = mortgage.add_extra_payment(12, 100_000.0);
        let _ = mortgage.add_extra_payment(24, 200_000.0);

        let summaries = mortgage.annual_summaries();

        // Paid off at month 115 -> 10 annual summaries (Years 1..=10)
        assert_eq!(summaries.len(), 10);
        assert_eq!(summaries.last().unwrap().year_end_balance, 0.0);

        // Interest sum match verification
        let annual_interest_sum: f64 = summaries.iter().map(|s| s.interest_paid).sum();
        let schedule_interest_sum: f64 = mortgage.schedule.values().map(|p| p.interest).sum();
        assert!((annual_interest_sum - schedule_interest_sum).abs() < 1e-6);
    }

    #[test]
    fn test_milestones_standard_and_accelerated() {
        let standard = Mortgage::new("Standard".to_string(), 1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();
        let crossover_std = standard.crossover_month();
        let half_eq_std = standard.half_equity_month();

        assert!(crossover_std.is_some());
        assert!(half_eq_std.is_some());

        let mut accelerated = Mortgage::new("Accelerated".to_string(), 1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();
        let _ = accelerated.add_extra_payment(1, 50_000.0);
        let _ = accelerated.add_extra_payment(12, 100_000.0);
        let _ = accelerated.add_extra_payment(24, 200_000.0);

        let crossover_acc = accelerated.crossover_month();
        let half_eq_acc = accelerated.half_equity_month();

        assert!(crossover_acc.is_some());
        assert!(half_eq_acc.is_some());

        // Extra principal payments move both milestones earlier
        assert!(crossover_acc.unwrap() <= crossover_std.unwrap());
        assert!(half_eq_acc.unwrap() < half_eq_std.unwrap());
    }

    #[test]
    fn test_json_persistence() {
        let temp_dir = std::env::temp_dir().join("mortgage_tests");
        let dir_str = temp_dir.to_str().unwrap();

        let mut original = Mortgage::new("Persist Test".to_string(), 500_000.0, 100_000.0, 6.0, 30, 1.0, 1_200.0).unwrap();
        let _ = original.add_extra_payment(6, 10_000.0);

        // Test JSON save & load
        let json_path = original.save_to_json(dir_str, "test_scenario").unwrap();
        assert!(Path::new(&json_path).exists());

        let loaded = Mortgage::load_from_json(&json_path).unwrap();
        assert_eq!(loaded.name, "Persist Test");
        assert_eq!(loaded.price, 500_000.0);
        assert_eq!(loaded.extra_payments.get(&6).copied(), Some(10_000.0));
        assert_eq!(loaded.schedule.len(), original.schedule.len());

        // Cleanup
        let _ = fs::remove_dir_all(temp_dir);
    }
}
