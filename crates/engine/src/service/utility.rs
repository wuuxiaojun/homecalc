//! utility.rs
//! Utility Functions

/// Clamps floating-point values near zero to exact `0.0` if within tolerance (`1e-6`).
#[inline]
pub fn clamp_zero(val: f64) -> f64 {
    if val.abs() < 1e-6 { 0.0 } else { val }
}

/// Rounds a floating-point number to 2 decimal places (standard currency cents precision).
#[inline]
pub fn round_cents(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_zero() {
        assert_eq!(clamp_zero(1e-7), 0.0);
        assert_eq!(clamp_zero(-1e-7), 0.0);
        assert_eq!(clamp_zero(0.0), 0.0);
        assert_eq!(clamp_zero(-0.0), 0.0);
        assert_eq!(clamp_zero(10.5), 10.5);
        assert_eq!(clamp_zero(-10.5), -10.5);
        assert_eq!(clamp_zero(9.99e-7), 0.0);
        assert_eq!(clamp_zero(1.01e-6), 1.01e-6);
    }

    #[test]
    fn test_round_cents() {
        assert_eq!(round_cents(123.456), 123.46);
        assert_eq!(round_cents(123.454), 123.45);
        assert_eq!(round_cents(0.0), 0.0);
    }
}
