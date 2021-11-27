use super::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IsConnectable(pub bool);

pub const CONNECTABLE: [u8; 11] = *b"Connectable";
pub const NO_CONNECTABLE: [u8; 15] = *b"Non-Connectable";

impl TryFrom<&[u8]> for IsConnectable {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value == CONNECTABLE {
            Ok(Self(true))
        } else if value == NO_CONNECTABLE {
            Ok(Self(false))
        } else {
            Err(ParseError::WrongValue)
        }
    }
}
