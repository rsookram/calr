use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidYear(i32),
    InvalidMonth(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::InvalidYear(year) => write!(f, "year `{}' is not in range 1..9999", year),
            Error::InvalidMonth(month) => write!(f, "{} is not a month number (1..12)", month),
        }
    }
}

impl std::error::Error for Error {}
