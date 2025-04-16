use crate::det::det;

pub fn inv(a00: f64, a01: f64, a10: f64, a11: f64) -> Option<(f64, f64, f64, f64)> {
    let d = det(a00, a01, a10, a11);
    if d == 0.0 {
        None
    } else {
        Some((a11/d, -a01/d, -a10/d, a00/d))
    }
}