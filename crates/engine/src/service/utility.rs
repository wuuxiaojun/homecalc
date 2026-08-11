use crate::config::constant::EPSILON;

#[inline]
pub fn clamp_zero(val: f64) -> f64 {
    if val.abs() < EPSILON { 0.0 } else { val }
}
