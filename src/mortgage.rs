// src/mortgage.rs

use crate::formula::monthly_payment;
use std::collections::HashMap;

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
#[derive(Debug, Clone, PartialEq)]
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

// Mortgage struct
pub struct Mortgage {
    pub price: f64,
    pub down: f64,
    pub loan: f64,                         // Total loan amount
    pub rate: f64,                         // Rate in percentage (e.g., 5.9)
    pub term: u32,                         // Term in years
    pub tax_rate: f64,                     // Annual property tax rate in % (e.g., 1.2)
    pub annual_insurance: f64,             // Annual homeowners insurance ($)
    pub base_payment: f64,                 // Base P&I payment each month
    pub extra_payments: HashMap<u32, f64>, // Extra payment schedule
    pub schedule: HashMap<u32, Payment>,   // Complete schedule
}

impl Mortgage {
    /// Construct new mortgage with Tax, Insurance, and Safeguard Validation
    pub fn new(
        price: f64,
        down: f64,
        rate: f64,
        term: u32,
        tax_rate: f64,
        annual_insurance: f64,
    ) -> Result<Self, InputError> {
        // Check Home Price
        if price <= 0.0 || price.is_nan() {
            return Err(InputError::InvalidPrice(
                "Price must be greater than 0.".to_string(),
            ));
        }
        // Check Down Payment
        if down < 0.0 || down.is_nan() {
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
        if rate < 0.0 || rate > 100.0 || rate.is_nan() {
            return Err(InputError::InvalidRate(
                "Interest rate must be between 0% and 100%.".to_string(),
            ));
        }
        // Check Loan Term
        if term < 1 || term > 30 {
            return Err(InputError::InvalidTerm(
                "Loan term must be between 1 and 30 years.".to_string(),
            ));
        }
        // Check Tax Rate
        if tax_rate < 0.0 || tax_rate > 20.0 || tax_rate.is_nan() {
            return Err(InputError::InvalidTaxRate(
                "Property tax rate must be between 0% and 20%.".to_string(),
            ));
        }
        // Check Annual Insurance
        if annual_insurance < 0.0 || annual_insurance.is_nan() {
            return Err(InputError::InvalidInsurance(
                "Annual insurance cannot be negative.".to_string(),
            ));
        }

        let mut mortgage = Mortgage {
            price,
            down,
            loan: price - down,
            rate,
            term,
            tax_rate,
            annual_insurance,
            base_payment: monthly_payment(price - down, rate / 100.0, term),
            extra_payments: HashMap::new(),
            schedule: HashMap::new(),
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
                println!(
                    "Month {} is past the payoff date. Loan is already fully paid off! No extra payment added.",
                    month
                );
                return Ok(0.0);
            }
        };

        let max_extra = current.balance + current.extra;
        if max_extra <= 0.0 {
            println!(
                "Loan balance at Month {} is already $0.00. No extra payment needed.",
                month
            );
            return Ok(0.0);
        }

        let actual_extra = if extra > max_extra {
            println!(
                "Requested extra payment (${:.2}) exceeds maximum remaining balance (${:.2}) at Month {}",
                extra, max_extra, month
            );
            println!(" Capping extra payment to ${:.2}.", max_extra);
            max_extra
        } else {
            extra
        };

        self.extra_payments.insert(month, actual_extra);
        self.recalculate();
        Ok(actual_extra)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase3_escrow_calculations() {
        let mortgage = Mortgage::new(1_500_000.0, 300_000.0, 5.9, 15, 1.2, 3_600.0).unwrap();

        // $1.5M * 1.2% / 12 = $1,500/mo tax
        assert_eq!(mortgage.monthly_tax(), 1_500.0);
        // $3,600 / 12 = $300/mo insurance
        assert_eq!(mortgage.monthly_insurance(), 300.0);
        // Escrow = $1,800/mo
        assert_eq!(mortgage.monthly_escrow(), 1_800.0);
    }
}
