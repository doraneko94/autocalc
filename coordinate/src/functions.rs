use crate::error::Result;
use crate::point::{LatLon, XY};

fn _safe_divide(num: f64, den: f64) -> f64 {
    if den == 0.0 { num / (den + 1e-7) } else { num / den }
}

pub fn circle_center(p0: LatLon, p1: LatLon, p2: LatLon) -> Result<(LatLon, f64)> {
    /*
    let d01 = p0.direction_sphere(p1).0;
    let d12 = p1.direction_sphere(p2).0;
    let d20 = p2.direction_sphere(p0).0;
    let (r, s1, s2) = if d20 < d12 {
        if d01 < d12 { (p0, p1, p2) } else { (p2, p0, p1) }
    } else {
        if d01 < d20 { (p1, p2, p0) } else { (p2, p0, p1) }
    };
    */ let (r, s1, s2) = (p0, p1, p2);
    let xy1 = XY::from_latlon(s1, r)?;
    let xy2 = XY::from_latlon(s2, r)?;
    let (x1, y1) = (xy1.x(), xy1.y());
    let (x2, y2) = (xy2.x(), xy2.y());
    let b = (x1*(x2*x2 + y2*y2) - x2*(x1*x1 + y1*y1)) / (x2*y1 - x1*y2);
    let a = -(x1*x1 + y1*y1 + b*y1) / x1;
    let (x0, y0) = (-a/2.0, -b/2.0);
    let distance = ((a*a + b*b) / 4.0).sqrt();
    let center = XY::from_xy(x0, y0, r)?.source();

    Ok((center, distance))
}