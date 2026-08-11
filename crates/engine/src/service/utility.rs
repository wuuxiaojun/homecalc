//! utility.rs
//! Utility Functions

// Clamps floating-point values near zero to exact `0.0` if within tolerance (`1e-6`).
#[inline]
pub fn clamp_zero(val: f64) -> f64 {
    if val.abs() < 1e-6 { 0.0 } else { val }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_zero() {
        assert_eq!(clamp_zero(1e-7), 0.0);
        assert_eq!(clamp_zero(-1e-7), 0.0);
        assert_eq!(clamp_zero(0.0), 0.0);
        assert_eq!(clamp_zero(10.5), 10.5);
        assert_eq!(clamp_zero(-10.5), -10.5);
    }
}
