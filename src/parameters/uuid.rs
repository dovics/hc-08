use super::ParseError;
use core::{char::from_digit, str::from_utf8};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UUID(pub u32);

impl Into<[u8; 4]> for UUID {
    fn into(mut self) -> [u8; 4] {
        let mut result = [0; 4];
        for i in 0..4 {
            result[i] = from_digit(self.0 % 16, 16).unwrap() as u8;
            self.0 >>= 4;
        }

        result
    }
}

impl TryFrom<&[u8]> for UUID {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > 4 {
            Err(ParseError::WrongValue)
        } else {
            Ok(UUID(u32::from_str_radix(from_utf8(&value)?, 16)?))
        }
    }
}
