//! constant.rs
//! Financial assumptions and constants

pub const DEFAULT_TAX_GROWTH_RATE: f64 = 0.02;
pub const DEFAULT_INSURANCE_GROWTH_RATE: f64 = 0.05;
pub const DEFAULT_HOA_GROWTH_RATE: f64 = 0.04;
pub const DEFAULT_MARGINAL_TAX_RATE: f64 = 0.24;
pub const DEFAULT_STARTING_CASH: f64 = 1_000_000.00;
pub const DEFAULT_DISCOUNT_RATE: f64 = 0.065;
pub const IRS_MORTGAGE_LIMIT: f64 = 750_000.00;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_values() {
        let tax_rate = DEFAULT_TAX_GROWTH_RATE;
        let ins_rate = DEFAULT_INSURANCE_GROWTH_RATE;
        let hoa_rate = DEFAULT_HOA_GROWTH_RATE;
        let marginal_tax = DEFAULT_MARGINAL_TAX_RATE;
        let starting_cash = DEFAULT_STARTING_CASH;
        let discount_rate = DEFAULT_DISCOUNT_RATE;
        let limit = IRS_MORTGAGE_LIMIT;

        assert_eq!(tax_rate, 0.02);
        assert_eq!(ins_rate, 0.05);
        assert_eq!(hoa_rate, 0.04);
        assert_eq!(marginal_tax, 0.24);
        assert_eq!(starting_cash, 1_000_000.0);
        assert_eq!(discount_rate, 0.065);
        assert_eq!(limit, 750_000.0);
    }
}
