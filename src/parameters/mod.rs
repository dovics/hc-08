pub mod addr;
pub mod baudrate;
pub mod connectable;
pub mod role;
pub mod uuid;

use core::{num::ParseIntError, str::Utf8Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parameters {
    pub role: role::Role,
    pub baud_rate: baudrate::BaudRate,
    pub addr: addr::Addr,
}

#[derive(Debug)]
pub enum ParseError {
    PrefixError,
    WithoutNewline,
    WrongValue,
    Uft8Error(Utf8Error),
    ParseIntError(ParseIntError),
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> Self {
        Self::Uft8Error(err)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}
