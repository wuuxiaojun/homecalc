use crate::formula::monthly_payment;
use std::collections::HashMap;

// Enum to check parameter input error
#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    InvalidPrice(String),
    InvalidDown(String),
    InvalidRate(String),
    InvalidTerm(String),
}

// Payment struct
#[derive(Debug, Clone, PartialEq)]
pub struct Payment {
    pub month: u32,
    pub total: f64,
    pub principal: f64,
    pub extra: f64,
    pub interest: f64,
    pub balance: f64,
}

// Mortgage struct
pub struct Mortgage {
    pub price: f64,
    pub down: f64,
    pub loan: f64,                         // total loan amount
    pub rate: f64,                         // rate in decimal, e.g 0.059
    pub term: u32,                         // term in years
    pub base_payment: f64,                 // base payment each month
    pub extra_payments: HashMap<u32, f64>, // extra payment schedule
    pub schedule: HashMap<u32, Payment>,   // complete schedule
}

// Implementation
impl Mortgage {
    // Construct new mortgage
    pub fn new(price: f64, down: f64, rate: f64, term: u32) -> Result<Self, InputError> {
        // Check Home Price
        if price <= 0.0 || price.is_nan() {
            return Err(InputError::InvalidPrice(
                "Price must be greater than 0.".to_string(),
            ));
        }

        // Check Down Payment
        if down <= 0.0 || down.is_nan() {
            return Err(InputError::InvalidDown(
                "Down payment must be greater than 0.".to_string(),
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

        let mut mortgage = Mortgage {
            price,
            down,
            loan: price - down,
            rate,
            term,
            base_payment: monthly_payment(loan, rate, term),
            extra_payments: HashMap::new(),
            schedule: HashMap::new(),
        };

        mortgage.recalculate();
        Ok(mortgage)
    }

    // Recalculate the amortization schedule internally
    pub fn recalculate(&mut self) {
        self.schedule.clear();
        let monthly_rate: f64 = self.rate / 12.0;
        let total_months: u32 = self.term * 12;
        let mut balance: f64 = self.loan;

        for month in 1..=total_months {
            if balance <= 1e-6 {
                break;
            }

            let interest_due: f64 = balance * monthly_rate;
            let mut principal_due: f64 = self.base_payment - interest_due;

            if balance <= principal_due {
                principal_due = balance;
                balance = 0.0;

                self.schedule.insert(
                    month,
                    Payment {
                        month,
                        total: principal_due + interest_due,
                        principal: principal_due,
                        extra_principal: 0.0,
                        interest: interest_due,
                        balance: 0.0,
                    },
                )
            }

            let extra_input: f64 = self.extra_payments.get(&month).copied().unwrap_or(0.0);
            let max_extra: f64 = balance - principal_due;
            let extra: f64 = extra_input.clamp(0, 0, max_extra);
            let total_principal: f64 = principal_due + extra;
            balance -= total_principal;

            if balance < 1e-6 {
                balance = 0.0;
            }

            self.schedule.insert(
                month,
                Payment {
                    month,
                    total: total_principal + interest_due,
                    principal: principal_due,
                    extra_principal: extra,
                    interest: interest_due,
                    balance,
                },
            )
        }
    }

    // Add extra payment
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

        let max_extra = current.balance;

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
