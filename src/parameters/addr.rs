use super::ParseError;
use core::str::from_utf8;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Addr([u8; 6]);

impl TryFrom<&[u8]> for Addr {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if &value[0..5] != b"Addr:" {
            return Err(ParseError::PrefixError);
        }

        if &value[value.len() - 2..value.len()] != b"\r\n" {
            return Err(ParseError::WithoutNewline);
        }

        let s = from_utf8(&value[5..value.len() - 2])?;

        let mut addr = [0; 6];
        (0..s.len())
            .step_by(3)
            .enumerate()
            .for_each(|(ai, i)| addr[ai] = u8::from_str_radix(&s[i..i + 2], 16).unwrap());

        Ok(Addr(addr))
    }
}
