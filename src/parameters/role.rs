use super::ParseError;
use core::str::from_utf8;

pub const MASTER: &str = "Master";
pub const SLAVE: &str = "Slave";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Role {
    Master,
    Slave,
}

impl TryFrom<&str> for Role {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            MASTER => Ok(Self::Master),
            SLAVE => Ok(Self::Slave),
            _ => Err(ParseError::WrongValue),
        }
    }
}

impl TryFrom<&[u8]> for Role {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if &value[0..5] != b"Role:" {
            return Err(ParseError::PrefixError);
        }

        if &value[value.len() - 2..value.len()] != b"\r\n" {
            return Err(ParseError::WithoutNewline);
        }

        let s = from_utf8(&value[5..value.len() - 2])?;
        Role::try_from(s)
    }
}
