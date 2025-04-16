use std::fmt;

pub type Result<T> = std::result::Result<T, NumError>;

#[derive(Debug)]
pub enum NumError {
    NegativePower,
    TimeOut,
    ZeroDivision,
}

impl fmt::Display for NumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NumError::NegativePower => write!(f, "Degree is less than 0!"),
            NumError::TimeOut => write!(f, "Calculation time is too long!"),
            NumError::ZeroDivision => write!(f, "Divided by 0!")
        }
    }
}