use std::ops::{Mul, MulAssign, Div, DivAssign};

use crate::error::{Result, NumError};
use crate::primary::FactMap;

#[derive(PartialEq, Eq, Clone)]
pub struct Sqrt {
    int: FactMap,
    sqrt: FactMap,
}

impl Sqrt {
    pub fn new(int: usize, sqrt: usize) -> Result<Self> {
        if int == 0 || sqrt == 0 { return Ok(Self::zero()); }
        let int = FactMap::new(int)?;
        let sqrt = FactMap::new(sqrt)?;
        let mut ret = Self { int, sqrt };
        ret.organize();
        Ok(ret)
    }
    pub fn zero() -> Self { Self { int: FactMap::zero(), sqrt: FactMap::zero() } }
    pub fn one() -> Self { Self { int: FactMap::one(), sqrt: FactMap::one() } }
    pub fn from_int(int: usize) -> Result<Self> { Self::new(int, 1) }
    pub fn from_sqrt(sqrt: usize) -> Result<Self> { Self::new(1, sqrt) }
    pub fn from_facts(int: FactMap, sqrt: FactMap) -> Self {
        let mut ret = Self { int, sqrt };
        ret.organize();
        ret
    }
    pub fn organize(&mut self) {
        for (&k, &v) in self.sqrt.clone().iter() {
            let _ = self.int.mul_fact(k, v/2);
            let _ = self.sqrt.div_fact(k, v/2*2);
        }
    }
    pub fn is_zero(&self) -> bool {
        self.int.is_zero() || self.sqrt.is_zero()
    }
    pub fn is_one(&self) -> bool {
        self.int.is_one() && self.sqrt.is_one()
    }
    pub fn split(&self) -> (FactMap, FactMap) {
        (self.int.clone(), self.sqrt.clone())
    }
    pub fn to_mathjax(&self) -> String {
        if self.is_zero() { return "0".to_string(); }
        if self.is_one() { return "1".to_string(); }
        let int = self.int.to_int();
        let sqrt = self.sqrt.to_int();
        let mut ans = if int > 1 { int.to_string() } else { "".to_string() };
        if sqrt > 1 { ans = ans + "\\sqrt{" + &sqrt.to_string() + "}" }
        return ans;
    }
}

impl Mul for Sqrt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let int = self.int * rhs.int;
        let sqrt = self.sqrt * rhs.sqrt;
        let mut ans = Self { int, sqrt };
        ans.organize();
        ans
    }
}

impl MulAssign for Sqrt {
    fn mul_assign(&mut self, rhs: Self) {
        self.int *= rhs.int;
        self.sqrt *= rhs.sqrt;
        self.organize();
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SqrtFrac {
    num: Sqrt,
    den: Sqrt,
}

impl SqrtFrac {
    pub fn new(num_int: usize, num_sqrt: usize, den: usize) -> Result<Self> {
        if den == 0 { return Err(NumError::ZeroDivision); }
        let num = Sqrt::new(num_int, num_sqrt)?;
        let den = Sqrt::from_int(den)?;
        let mut ret = Self { num, den };
        ret.reduct();
        Ok(ret)
    }
    pub fn organize(&mut self) {
        self.num.sqrt *= self.den.sqrt.clone();
        self.den.int *= self.den.sqrt.clone();
        self.den.sqrt = FactMap::one();
        self.reduct();
    }
    pub fn reduct(&mut self) {
        self.num.int.reduct(&mut self.den.int);
    }
    pub fn zero() -> Self { Self { num: Sqrt::zero(), den: Sqrt::one() } }
    pub fn is_zero(&self) -> bool { self.num.is_zero() }
    pub fn is_one(&self) -> bool { self.num.is_one() }
    pub fn to_mathjax(&mut self) -> String {
        if !self.den.sqrt.is_one() { self.organize(); }
        //let num_int = self.num.int.to_int().to_string();
        //let num_sqrt = self.num.sqrt.to_int().to_string();
        //let den = self.den.int.to_int().to_string();
        "".to_string()
    }
}

impl Div for Sqrt {
    type Output = Result<SqrtFrac>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.is_zero() { return Ok(SqrtFrac::zero()); }
        if rhs.is_zero() { return Err(NumError::ZeroDivision); }
        let mut ans = SqrtFrac { num: self, den: rhs };
        ans.organize();
        Ok(ans)
    }
}

impl Mul for SqrtFrac {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        let den = self.den * rhs.den;
        (num / den).unwrap()
    }
}

impl MulAssign for SqrtFrac {
    fn mul_assign(&mut self, rhs: Self) {
        self.num *= rhs.num;
        self.den *= rhs.den;
        self.organize();
    }
}

impl Div for SqrtFrac {
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.den;
        let den = self.den * rhs.num;
        num / den
    }
}

impl DivAssign for SqrtFrac {
    fn div_assign(&mut self, rhs: Self) {
        self.num *= rhs.den;
        self.den *= rhs.num;
        self.organize();
    }
}