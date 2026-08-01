use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    InvalidPrice(String),
    InvalidDown(String),
    InvalidRate(String),
    InvalidTerm(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Payment {
    pub month: u32,
    pub principal: f64,
    pub extra: f64,
    pub interest: f64,
    pub total: f64,
    pub balance: f64,
}

pub struct Mortgage {
    pub price: f64,
    pub down: f64,
    pub loan: f64, // total loan amount
    pub rate: f64, // rate in decimal, e.g 0.059
    pub term: u32, // term in years
    pub extra_payments: HashMap<u32, f64>,
    pub schedule: HashMap<u32, Payment>,
}

// Implementation
impl Mortgage {
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
            extra_payments: HashMap::new(),
            schedule: HashMap::new(),
        };

        mortgage.recalculate();
        Ok(mortgage)
    }
}
