pub enum D {
    Positive(f64),
    Zero,
    Negative(f64),
}

pub fn discriminant(a: f64, b: f64, c: f64) -> D {
    let d = b*b - 4.0 * a * c;
    if d > 0.0 { D::Positive(d) }
    else if d == 0.0 { D::Zero }
    else { D::Negative(-d) }
}

pub fn solve(a: f64, b: f64, c: f64) -> String {
    if a == 0.0 {
        if b == 0.0 {
            if c == 0.0 { return "\\mathrm{None}".to_string(); }
            else { return "y(x)=0".to_string(); }
        } else {
            if c == 0.0 { return "y(x)=C".to_string(); }
            else {
                let tmp = -c/b;
                if tmp == 1.0 { return "y(x)=C\\exp(x)".to_string(); }
                else if tmp == -1.0 { return "y(x)=C\\exp(-x)".to_string(); }
                else { return format!("y(x)=C\\exp({}x)", tmp); }
            }
        }
    } else {
        if c == 0.0 {
            if b == 0.0 { return "y(x)=(C_1+C_2x)".to_string(); }
            else {
                let tmp = -b/a;
                if tmp == 1.0 { return "y(x)=C_1\\exp(x)+C_2".to_string(); }
                else if tmp == -1.0 { return "y(x)=C_1\\exp(-x)+C_2".to_string(); }
                else { return format!("y(x)=C_1\\exp({}x)+C_2", tmp); }
            }
        }
    }
    let d = discriminant(a, b, c);
    match d {
        D::Positive(value) => {
            let re = -b / (2.0*a);
            let sq = value.sqrt() / (2.0*a);
            let (lamb_1, lamb_2) = (re - sq, re + sq);
            let s1 = if lamb_1 == 1.0 { "".to_string() } else if lamb_1 == -1.0 { "-".to_string() } else { format!("{:.3}", lamb_1) };
            let s2 = if lamb_2 == 1.0 { "".to_string() } else if lamb_2 == -1.0 { "-".to_string() } else { format!("{:.3}", lamb_2) };
            format!("y(x)=C_1\\exp({}x)+C_2\\exp({}x)", s1, s2)
        }
        D::Zero => {
            let tmp = -b / (2.0*a);
            let s1 = if tmp == 1.0 { "".to_string() } else if tmp == -1.0 { "-".to_string() } else { format!("{:.3}", tmp) };
            format!("y(x)=(C_1+C_2x)\\exp({}x)", s1)
        }
        D::Negative(value) => {
            let re = -b / (2.0*a);
            let sq = value.sqrt() / (2.0*a);
            let p = if re == 1.0 { "".to_string() } else if re == -1.0 { "-".to_string() } else { format!("{:.3}", re) };
            let q = if sq == 1.0 { "".to_string() } else if sq == -1.0 { "-".to_string() } else { format!("{:.3}", sq) };
            if re == 0.0 {
                format!("y(x)=C_1\\cos({0}x)+C_2\\sin({0}x)", q)
            } else {
                format!("y(x)=\\exp({0}x)\\{{C_1\\cos({1}x)+C_2\\sin({1}x)\\}}", p, q)
            }
        }
    }
}