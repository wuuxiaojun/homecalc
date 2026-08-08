// Fixed Mortgage Monthly Payment
pub fn calculate_mortgage_pmt(principal: f64, rate: f64, year: u32) -> f64 {
    if principal <= 0.0 || year == 0 {
        return 0.0;
    }

    let total_payments = (year * 12) as i32;

    if rate <= 0.0 {
        return principal / total_payments as f64;
    }

    let monthly_rate = (rate * 0.01) / 12.0;

    let factor = (1.0 + monthly_rate) * powi(total_payments);

    principal * (monthly_rate * factor) / (factor - 1.0)
}

// Monthly Compound Function for Mortgage & Loc & Cash
pub struct Compound {
    pub interest: f64,
    pub total: f64,
}

pub fn calculate_monthly_compound(amount: f64, annual_rate: f64) -> Compound {
    if amount <= 0.0 || annual_rate <= 0.0 {
        return Compound {
            interest: 0.0,
            total: amount.max(0.0),
        };
    }

    let monthly_rate = (annual_rate * 0.01) / 12.0;
    let interest = amount * monthly_rate;

    Compound {
        interest,
        total: amount + interest,
    }
}
