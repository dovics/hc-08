use super::ParseError;
use core::str::from_utf8;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BaudRate {
    /// 1200 bauds per second
    Bps1200 = 1200,
    /// 2400 bauds per second
    Bps2400 = 2400,
    /// 4800 bauds per second
    Bps4800 = 4800,
    /// 9600 bauds per second
    Bps9600 = 9600,
    /// 19200 bauds per second
    Bps19200 = 19200,
    /// 38400 bauds per second
    Bps38400 = 38400,
    /// 57600 bauds per second
    Bps57600 = 57600,
    /// 115200 bauds per second
    Bps115200 = 115200,
}

impl TryFrom<i32> for BaudRate {
    type Error = ParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1200 => Ok(BaudRate::Bps1200),
            2400 => Ok(BaudRate::Bps2400),
            4800 => Ok(BaudRate::Bps4800),
            9600 => Ok(BaudRate::Bps9600),
            19200 => Ok(BaudRate::Bps19200),
            38400 => Ok(BaudRate::Bps38400),
            57600 => Ok(BaudRate::Bps57600),
            115200 => Ok(BaudRate::Bps115200),
            _ => Err(ParseError::WrongValue),
        }
    }
}

impl TryFrom<&[u8]> for BaudRate {
    type Error = ParseError;
    // Baud:9600,NONE
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if &value[0..5] != b"Baud:" {
            return Err(ParseError::PrefixError);
        }

        if &value[value.len() - 2..value.len()] != b"\r\n" {
            return Err(ParseError::WithoutNewline);
        }

        let s = from_utf8(&value[5..value.len() - 2])?;
        let (buad, _crc) = s.split_once(',').ok_or(ParseError::WrongValue)?;

        let value = buad.parse::<i32>()?;
        return Self::try_from(value);
    }
}
