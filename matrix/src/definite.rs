use crate::det::det;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Def {
    Pos = 2,
    PosSemi = 1,
    Non = 0,
    NegSemi = -1,
    Neg = -2,
}

pub fn is_def(a00: f64, a01: f64, a10: f64, a11: f64) -> Def {
    let d = det(a00, a01, a10, a11);
    if a00 >= 0.0 && a11 >= 0.0 {
        if a00 > 0.0 && a11 > 0.0 && d > 0.0 {
            Def::Pos
        } else if d >= 0.0 {
            Def::PosSemi
        } else {
            Def::Non
        }
    } else if a00 <= 0.0 && a11 <= 0.0 {
        if a00 < 0.0 && a11 < 0.0 && d < 0.0 {
            Def::Neg
        } else if d <= 0.0 {
            Def::NegSemi
        } else {
            Def::Non
        }
    } else {
        Def::Non
    }
}