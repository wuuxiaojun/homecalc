// src/domain/formula.rs

/// Calculate monthly payment based on loan amount, rate (as decimal), term (in years)
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

/// Calculates annual and monthly tax savings from mortgage interest deduction.
/// Assumes Married Filing Jointly (MFJ) in California with 24% Fed marginal rate and 9.3% CA state marginal rate.
pub fn calculate_annual_tax_savings(initial_loan_amount: f64, annual_interest_paid: f64) -> (f64, f64) {
    if initial_loan_amount <= 0.0 || annual_interest_paid <= 0.0 {
        return (0.0, 0.0);
    }

    const FED_RATE: f64 = 0.24;
    const FED_LOAN_CAP: f64 = 750000.0;
    const FED_STD_DEDUCTION: f64 = 30000.0;
    const SALT_CAP: f64 = 10000.0;
    const CA_RATE: f64 = 0.093;
    const CA_LOAN_CAP: f64 = 1000000.0;
    const CA_STD_DEDUCTION: f64 = 11412.0;

    let fed_eligible_interest = annual_interest_paid * (FED_LOAN_CAP / initial_loan_amount).min(1.0);
    let ca_eligible_interest = annual_interest_paid * (CA_LOAN_CAP / initial_loan_amount).min(1.0);

    let fed_deduction = ((fed_eligible_interest + SALT_CAP) - FED_STD_DEDUCTION).max(0.0);
    let ca_deduction = (ca_eligible_interest - CA_STD_DEDUCTION).max(0.0);

    let annual_tax_savings = (fed_deduction * FED_RATE) + (ca_deduction * CA_RATE);
    let monthly_tax_savings = annual_tax_savings / 12.0;

    (annual_tax_savings, monthly_tax_savings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_annual_tax_savings_small_loan() {
        let (annual, monthly) = calculate_annual_tax_savings(500_000.0, 30_000.0);
        // fed_deduction = (30000 + 10000) - 30000 = 10000 -> 10000 * 0.24 = 2400
        // ca_deduction = 30000 - 11412 = 18588 -> 18588 * 0.093 = 1728.684
        // annual = 4128.684
        assert!((annual - 4128.684).abs() < 1e-3);
        assert!((monthly - 344.057).abs() < 1e-3);
    }

    #[test]
    fn test_calculate_annual_tax_savings_jumbo_loan() {
        let (annual, monthly) = calculate_annual_tax_savings(1_500_000.0, 90_000.0);
        // fed_eligible = 90000 * (750k/1.5M) = 45000
        // ca_eligible = 90000 * (1M/1.5M) = 60000
        // fed_deduction = (45000 + 10000) - 30000 = 25000 -> 25000 * 0.24 = 6000
        // ca_deduction = 60000 - 11412 = 48588 -> 48588 * 0.093 = 4518.684
        // annual = 10518.684
        assert!((annual - 10518.684).abs() < 1e-3);
        assert!((monthly - 876.557).abs() < 1e-3);
    }
}
