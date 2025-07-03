use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidYear(u16),
    InvalidMonth(u16),
    UnknownOffset,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::InvalidYear(year) => write!(f, "year `{year}' is not in range 1..9999"),
            Error::InvalidMonth(month) => write!(f, "{month} is not a month number (1..12)"),
            Error::UnknownOffset => write!(f, "failed to determine current timezone offset"),
        }
    }
}

impl std::error::Error for Error {}
