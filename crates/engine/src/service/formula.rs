// formula.rs
// Common Formulas

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

    let factor = (1.0 + monthly_rate).powi(total_payments);

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

/// Newton-Raphson solver for finding the monthly root and converting to annualized IRR.
pub fn solve_irr_newton_raphson(cash_flows: &[f64]) -> Option<f64> {
    let mut rate: f64 = 0.005; // Initial guess: 0.5% monthly (~6.0% annual)
    let max_iterations = 100;
    let tolerance = 1e-7;

    for _ in 0..max_iterations {
        let mut npv = 0.0;
        let mut derivative = 0.0;

        for (idx, &flow) in cash_flows.iter().enumerate() {
            let m = (idx + 1) as f64;
            let factor = (1.0 + rate).powf(m);

            npv += flow / factor;
            derivative -= m * flow / (factor * (1.0 + rate));
        }

        if npv.abs() < tolerance {
            // Convert monthly compounding rate to effective annualized IRR
            let annual_irr = (1.0 + rate).powi(12) - 1.0;
            return Some(annual_irr);
        }

        if derivative.abs() < 1e-10 {
            return None; // Avoid division by zero
        }

        rate -= npv / derivative;
    }

    None // Failed to converge
}
