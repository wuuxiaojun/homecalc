//! utility.rs
//! Utility Functions

// Clamps floating-point values near zero to exact `0.0` if within tolerance (`1e-6`).
#[inline]
pub fn clamp_zero(val: f64) -> f64 {
    if val.abs() < 1e-6 { 0.0 } else { val }
}
