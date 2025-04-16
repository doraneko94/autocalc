use std::collections::{HashMap, hash_map::{Entry, Iter, IterMut}};
use std::ops::{Mul, MulAssign};

use crate::error::{Result, NumError};

const COUNT_LIM: usize = 1_000_000_000;

pub enum FactResult {
    Zero,
    One,
    Prime,
    Factor(usize),
    Unknown,
}

fn _isqrt(x: usize) -> usize {
    let mut now = (x as f64).sqrt() as usize;
    if x > usize::MAX / 2 { return now; }
    let mut lower = now / 10;
    let mut upper = now * 10;
    loop {
        if now*now <= x && (now+1)*(now+1) > x { break; }
        if now*now > x {
            let n_tmp = (now + lower) / 2;
            lower = now;
            now = n_tmp;
        } else {
            let n_tmp = (now + upper) / 2;
            upper = now;
            now = n_tmp;
        }
    }
    now
    
}

pub fn factor(x: usize) -> (FactResult, usize) {
    if x == 0 { return (FactResult::Zero, 0); }
    if x == 1 { return (FactResult::One, 0); }
    if x == 2 { return (FactResult::Prime, 0); }
    let mut cand = 2;
    let lim = _isqrt(x);
    let count_lim = 10_usize.pow(9);
    for i in 0..count_lim {
        if x % cand == 0 { return (FactResult::Factor(cand), i); }
        if cand == 2 { cand += 1 } else { cand += 2 };
        if cand > lim { return (FactResult::Prime, i); }
    }
    (FactResult::Unknown, count_lim) 
}

#[derive(PartialEq, Eq, Clone)]
pub struct FactMap {
    is_zero: bool,
    fact: HashMap<usize, usize>,
}

fn _factorize(x: usize) -> Result<FactMap> {
    let mut x = x;
    let mut fact = HashMap::new();
    let mut count = 0;
    loop {
        match factor(x) {
            (FactResult::Zero, _) => { return Ok(FactMap { is_zero: true, fact }); }
            (FactResult::One, _) => { return Ok(FactMap { is_zero: false, fact }); }
            (FactResult::Prime, _) => {
                let c = fact.entry(x).or_insert(0);
                *c += 1;
                break;
            }
            (FactResult::Factor(val), i) => {
                let c = fact.entry(val).or_insert(0);
                *c += 1;
                x /= val;
                count += i;
            }
            (FactResult::Unknown, _) => {
                return Err(NumError::TimeOut);
            }
        };
        if count > COUNT_LIM { return Err(NumError::TimeOut); }
    }
    Ok(FactMap { is_zero: false, fact })
}

impl FactMap  {
    pub fn new(x: usize) -> Result<Self> {
        _factorize(x)
    }
    pub fn zero() -> Self { Self::new(0).unwrap() }
    pub fn one() -> Self { Self::new(1).unwrap() }
    pub fn is_zero(&self) -> bool {
        self.is_zero
    }
    pub fn is_one(&self) -> bool {
        !self.is_zero && self.fact.len() == 0
    }
    pub fn to_int(&self) -> usize {
        let mut ans = 1;
        for (&k, &v) in self.fact.iter() {
            ans *= k.pow(v as u32);
        }
        ans
    }
    pub fn into_zero(&mut self) {
        *self = Self::zero();
    }
    pub fn mul_fact(&mut self, k: usize, v: usize) -> Result<usize> {
        if k == 0 { self.into_zero(); return Ok(0); }
        if v == 0 {
            return match self.fact.entry(k) {
                Entry::Occupied(oe) => Ok(*oe.get()),
                Entry::Vacant(_) => Ok(0),
            }
        }
        let count = self.fact.entry(k).or_insert(0);
        *count += v;
        Ok(*count)
    }
    pub fn div_fact(&mut self, k: usize, v: usize) -> Result<usize> {
        if k == 0 { return Err(NumError::ZeroDivision); }
        if v == 0 {
            return match self.fact.entry(k) {
                Entry::Occupied(oe) => Ok(*oe.get()),
                Entry::Vacant(_) => Ok(0),
            }
        }
        match self.fact.entry(k) {
            Entry::Occupied(mut oe) => {
                let v0 = *oe.get();
                if v0 < v { Err(NumError::NegativePower) }
                else if v0 == v {
                    let _ = oe.remove_entry();
                    Ok(0)
                } else {
                    *oe.get_mut() -= v;
                    Ok(*oe.get())
                }
            }
            Entry::Vacant(_) => { Err(NumError::NegativePower) }
        }
    }
    pub fn reduct(&mut self, other: &mut Self) {
        for (k, v0) in self.fact.clone().iter() {
            if let Some(v1) = other.fact.get_mut(k) {
                let v = std::cmp::min(*v0, *v1);
                let _ = self.div_fact(*k, v);
                let _ = other.div_fact(*k, v);
            }
        }
    }
    pub fn extract_common(&mut self, other: &mut Self) -> Self {
        if self.is_zero() || other.is_zero() { return Self::one(); }
        if self.is_one() || other.is_one() { return Self::one(); }
        let mut ans = Self::one();
        for (&k, &v1) in self.clone().iter() {
            if let Some(&v2) = other.fact.get(&k) {
                let val = std::cmp::min(v1, v2);
                let _ = ans.mul_fact(k, val);
                let _ = self.div_fact(k, val);
                let _ = other.div_fact(k, val);
            }
        }
        ans
    }
    pub fn to_mathjax(&self) -> String {
        if self.is_zero() { return "0".to_string(); }
        if self.is_one() { return "1".to_string(); }
        
        let size = self.fact.len();
        let mut fv = self.fact.iter().map(|(&k, &v)| (k, v)).collect::<Vec<(usize, usize)>>();
        fv.sort_by(|a, b| a.0.cmp(&b.0));
        
        let mut s = "".to_string();
        for (i, &(k, v)) in fv.iter().enumerate() {
            s = s + &k.to_string() + "^{" + &v.to_string() + "}";
            if i < size - 1 {
                s = s + "\\times";
            }
        }
        s
    }
    pub fn iter(&self) -> Iter<'_, usize, usize> {
        self.fact.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, usize, usize> {
        self.fact.iter_mut()
    }
}

impl Mul for FactMap {
    type Output = FactMap;
    
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() { return Self::zero(); }
        let mut ans = self.clone();
        for (&k, &v) in rhs.iter() {
            let _ = ans.mul_fact(k, v);
        }
        ans
    }
}

impl MulAssign for FactMap {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs
    }
}