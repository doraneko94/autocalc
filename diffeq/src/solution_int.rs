use number_calc::error::Result;
use number_calc::primary::FactMap;
use number_calc::sqrt::Sqrt;

pub enum D {
    Positive(String, String),
    Zero(String),
    Negative(String, String),
}

fn _discriminant(a: isize, b: isize, c: isize) -> Result<D> {
    let (a, b, c) = if a < 0 { (-a, -b, -c) } else { (a, b, c) };
    let d = b*b - 4 * a * c;
    if d == 0 {
        let mut a_f = FactMap::new(2 * a as usize)?;
        let mut b_f = FactMap::new(b.abs() as usize)?;
        b_f.reduct(&mut a_f);
        let pf = if b > 0 { "-" } else { "" };
        if a_f.is_one() { return Ok(D::Zero(format!("{}{}", pf, b_f.to_int()))); }
        else { return Ok(D::Zero(format!("{}\\frac{{{}}}{{{}}}", pf, b_f.to_int(), a_f.to_int()))); }
    } else if d > 0 {
        let mut den_f = FactMap::new(2 * a as usize)?;
        let (mut int_f, sqrt_f) = Sqrt::from_sqrt(d as usize)?.split();

        let (s1, s2) = if b == 0 {
            int_f.reduct(&mut den_f);
            let s = Sqrt::from_facts(int_f, sqrt_f).to_mathjax();
            if den_f.is_one() {
                (format!("{}", &s), format!("-{}", &s))
            } else {
                let den_s = den_f.to_int().to_string();
                (format!("\\frac{{{}}}{{{}}}", &s, &den_s), format!("-\\frac{{{}}}{{{}}}", &s, &den_s))
            }
        } else if sqrt_f.is_one() {
            let (mut den_fp, mut den_fm) = (den_f.clone(), den_f.clone());
            let (bp, bm) = (-b + int_f.to_int() as isize, -b - int_f.to_int() as isize);
            let sp = {
                let mut b_f = FactMap::new(bp.abs() as usize)?;
                b_f.reduct(&mut den_fp);
                let s = b_f.to_int().to_string();
                let pf = if bp < 0 { "-" } else { "" };
                if den_fp.is_one() {
                    format!("{}{}", pf, &s)
                } else {
                    format!("{}\\frac{{{}}}{{{}}}", pf, &s, den_fp.to_int().to_string())
                }
            };
            let sm = {
                let mut b_f = FactMap::new(bm.abs() as usize)?;
                b_f.reduct(&mut den_fm);
                let s = b_f.to_int().to_string();
                let pf = if bm < 0 { "-" } else { "" };
                if den_fm.is_one() {
                    format!("{}{}", pf, &s)
                } else {
                    format!("{}\\frac{{{}}}{{{}}}", pf, &s, den_fm.to_int().to_string())
                }
            };
            (sp, sm)
        } else {
            let mut b_f = FactMap::new(b.abs() as usize)?;
            let mut com_f = b_f.extract_common(&mut int_f);
            com_f.reduct(&mut den_f);
            b_f *= com_f.clone();
            int_f *= com_f;
            let b_s = b_f.to_int().to_string();
            let sqrt_s = Sqrt::from_facts(int_f, sqrt_f).to_mathjax();
            let pf = if b > 0 { "-" } else { "" };
            if den_f.is_one() {
                (format!("({}{}+{})", pf, &b_s, &sqrt_s), format!("({}{}-{})", pf, &b_s, &sqrt_s))
            } else {
                let den_s = den_f.to_int().to_string();
                (
                    format!("{}\\frac{{{}+{}}}{{{}}}", pf, &b_s, &sqrt_s, &den_s),
                    format!("{}\\frac{{{}-{}}}{{{}}}", pf, &b_s, &sqrt_s, &den_s)
                )
            }
        };
        Ok(D::Positive(s1, s2))
    } else {
        let p = if b == 0 { "0".to_string() }
        else {
            let mut b_f = FactMap::new(b.abs() as usize)?;
            let mut den_f = FactMap::new(2 * a as usize)?;
            b_f.reduct(&mut den_f);
            let pf = if b > 0 { "-".to_string() } else { "".to_string() };
            let b_s = b_f.to_int().to_string();
            if den_f.is_one() { pf + &b_s }
            else {
                let den_s = den_f.to_int().to_string();
                pf + "\\frac{" + &b_s + "}{" + &den_s + "}"
            }
        };
        let (mut int_f, sqrt_f) = Sqrt::from_sqrt(-d as usize)?.split();
        let mut den_f = FactMap::new(2 * a as usize)?;
        int_f.reduct(&mut den_f);
        let sqrt_s = Sqrt::from_facts(int_f, sqrt_f).to_mathjax();
        let q = if den_f.is_one() { sqrt_s }
        else { "\\frac{".to_string() + &sqrt_s + "}{" + &den_f.to_int().to_string() + "}" };
        Ok(D::Negative(p, q))
    }
}

pub fn solve(a: isize, b: isize, c: isize) -> Result<String> {
    if a == 0 {
        if b == 0 {
            if c == 0 { return Ok("\\mathrm{None}".to_string()); }
            else { return Ok("y(x)=0".to_string()); }
        } else {
            if c == 0 { return Ok("y(x)=C".to_string()); }
            else {
                let pf = if b*c > 0 { "-" } else { "" };
                let mut b_f = FactMap::new(b.abs() as usize)?;
                let mut c_f = FactMap::new(c.abs() as usize)?;
                c_f.reduct(&mut b_f);
                if b_f.is_one() {
                    if c_f.is_one() { return Ok(format!("y(x)=C\\exp({}x)", pf)); }
                    else { return Ok(format!("y(x)=C\\exp({}{}x)", pf, c_f.to_int())); }
                } else {
                    return Ok(format!("y(x)=C\\exp\\left({}\\frac{{{}}}{{{}}}x\\right)", pf, c_f.to_int(), b_f.to_int()));
                }
            }
        }
    } else {
        if c == 0 {
            if b == 0 { return Ok("y(x)=(C_1+C_2x)".to_string()); }
            else {
                let pf = if a*b > 0 { "-" } else { "" };
                let mut a_f = FactMap::new(a.abs() as usize)?;
                let mut b_f = FactMap::new(b.abs() as usize)?;
                b_f.reduct(&mut a_f);
                if a_f.is_one() {
                    if b_f.is_one() { return Ok(format!("y(x)=C_1\\exp({}x)+C_2", pf)); }
                    else { return Ok(format!("y(x)=C_1\\exp({}{}x)+C_2", pf, b_f.to_int())); }
                } else {
                    return Ok(format!("y(x)=C_1\\exp\\left({}\\frac{{{}}}{{{}}}x\\right)+C_2", pf, b_f.to_int(), a_f.to_int()));
                }
            }
        }
    }
    match _discriminant(a, b, c)? {
        D::Positive(lamb1, lamb2) => {
            let lamb1 = if &lamb1 == "1" { "" } else if &lamb1 == "-1" { "-" } else { &lamb1 };
            let lamb2 = if &lamb2 == "1" { "" } else if &lamb2 == "-1" { "-" } else { &lamb2 };
            match lamb1.chars().nth(0) {
                Some('(') => Ok(format!("y(x)=C_1\\exp\\left\\{{{}x\\right\\}}+C_2\\exp\\left\\{{{}x\\right\\}}", lamb1, lamb2)),
                _ => Ok(format!("y(x)=C_1\\exp\\left({}x\\right)+C_2\\exp\\left({}x\\right)", lamb1, lamb2))
            }
        }
        D::Zero(lamb) => {
            let lamb = if &lamb == "1" { "" } else if &lamb == "-1" { "-" } else { &lamb };
            Ok(format!("y(x)=(C_1+C_2x)\\exp\\left({}x\\right)", lamb))
        }
        D::Negative(p, q) => {
            let p = if &p == "1" { "" } else if &p == "-1" { "-" } else { &p };
            let q = if &q == "1" { "" } else if &q == "-1" { "-" } else { &q };
            if p == "0" {
                Ok(format!("y(x)=C_1\\cos\\left({0}x\\right)+C_2\\sin\\left({0}x\\right)", q))
            } else {
                Ok(format!("y(x)=\\exp\\left({0}x\\right)\\left\\{{C_1\\cos\\left({1}x\\right)+C_2\\sin\\left({1}x\\right)\\right\\}}", p, q))
            }
            
        }
    }
}