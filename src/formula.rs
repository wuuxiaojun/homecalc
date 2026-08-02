// Calculate monthly payment based on loan amount, rate (as decimal), term (in years)
pub fn monthly_payment(loan: f64, rate: f64, term: u32) -> f64 {
    let n = (term * 12) as f64;
    if n == 0.0 {
        return 0.0;
    }
    if rate <= 0.0 {
        return loan / n;
    }

    let r = rate / 12.0;
    let factor = (1.0 + r).powf(n);

    if factor == 1.0 {
        return loan / n;
    }

    loan * (r * factor) / (factor - 1.0)
}

