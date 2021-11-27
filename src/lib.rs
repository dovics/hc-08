#![no_std]
pub mod command;
pub mod parameters;

use command::{
    build_change_connect_internal_command, build_change_connect_internal_response,
    build_change_connect_timeout_command, build_change_connect_timeout_response,
    build_change_connectable_command, build_change_role_command,
    build_set_characteristic_uuid_command, build_set_characteristic_uuid_response,
    build_set_connect_uuid_command, build_set_connect_uuid_response,
    build_set_service_uuid_command, build_set_service_uuid_response, CHANGE_BROADCAST_BASE,
    CHANGE_NAME_BASE, CHARACTERISTIC_UUID_RESPONSE, CLEAR_ADDR, CONNECT_UUID_RESPONSE, OK_QUERY,
    OK_RESPONSE, QUERY_CHARACTERISTIC_UUID, QUERY_CONNECTABLE, QUERY_CONNECT_UUID, QUERY_NAME,
    QUERY_PARAMS_COMMAND, QUERY_ROLE, QUERY_SERVICE_UUID, QUERY_VERSION, RESET_SETTINGS_COMMAND,
    SERVICE_UUID_RESPONSE,
};
use parameters::uuid::UUID;

use core::marker::PhantomData;
use core::str::{from_utf8, Utf8Error};
use parameters::connectable::IsConnectable;

use parameters::ParseError;
use parameters::{addr::Addr, baudrate::BaudRate, role::Role, Parameters};

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::serial::{Read, Write};
#[derive(Debug)]
pub enum Error {
    Read,
    Write,
    InvalidBaudRate,
    InvalidChannel,
    WrongResponse,
    ParseError(ParseError),
    Utf8Error(Utf8Error),
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

pub struct Hc08<S, D, R, C> {
    serial: S,
    delay: D,
    role: PhantomData<R>,
    connectable: PhantomData<C>,
}

pub struct Master;
pub struct Slave;
pub struct Connectable;
pub struct NonConnectable;

impl<S, D> Hc08<S, D, Master, Connectable>
where
    S: Write<u8> + Read<u8>,
    D: DelayMs<u32>,
{
    pub fn new(serial: S, delay: D) -> Self {
        let mut result = Self {
            serial,
            delay,
            role: PhantomData::<Master>,
            connectable: PhantomData::<Connectable>,
        };

        result.reset_setting().unwrap();
        result
    }
}

type ToCentral<S, D, R, C> = Result<Hc08<S, D, Master, Connectable>, Hc08<S, D, R, C>>;
type ToPeripheral<S, D, R, C> = Result<Hc08<S, D, Slave, Connectable>, Hc08<S, D, R, C>>;
type ToObserver<S, D, R, C> = Result<Hc08<S, D, Master, NonConnectable>, Hc08<S, D, R, C>>;
type ToBroadcast<S, D, R, C> = Result<Hc08<S, D, Slave, NonConnectable>, Hc08<S, D, R, C>>;

impl<S, D, R, C> Hc08<S, D, R, C>
where
    S: Write<u8> + Read<u8>,
    D: DelayMs<u32>,
{
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), Error> {
        for ch in buffer {
            let _ = self.serial.write(*ch);
        }

        Ok(())
    }

    pub fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let mut n = 0;
        while n < buffer.len() {
            if let Ok(ch) = self.serial.read() {
                buffer[n] = ch;
                n += 1;
            }
        }
        Ok(())
    }

    fn send_command(&mut self, command: &[u8]) -> Result<(), Error> {
        self.write_buffer(command)?;

        if !self.wait_ok_response() {
            Err(Error::WrongResponse)
        } else {
            Ok(())
        }
    }

    fn wait_ok_response(&mut self) -> bool {
        let mut buffer = [0u8; 2];
        self.read_buffer(&mut buffer).unwrap();
        buffer == OK_RESPONSE
    }

    pub fn change_name(&mut self, name: &str) -> Result<(), Error> {
        self.write_buffer(&CHANGE_NAME_BASE)?;
        self.write_buffer(name.as_bytes())?;

        if !self.wait_ok_response() {
            Err(Error::WrongResponse)
        } else {
            Ok(())
        }
    }

    pub fn query_connectable(&mut self) -> Result<IsConnectable, Error> {
        self.write_buffer(&QUERY_CONNECTABLE)?;
        let mut buffer = [0u8; 15];
        let mut n = 0;
        while n < buffer.len() {
            if n == 11 && buffer[0] == b'C' {
                break;
            }

            if let Ok(ch) = self.serial.read() {
                buffer[n] = ch;
                n += 1;
            }
        }

        let c = IsConnectable::try_from(&buffer[0..n])?;

        Ok(c)
    }

    pub fn query_role(&mut self) -> Result<Role, Error> {
        self.write_buffer(&QUERY_ROLE)?;
        let mut buffer = [0u8; 6];
        let mut n = 0;
        while n < buffer.len() {
            if n == 5 && buffer[0] == b'S' {
                break;
            }

            if let Ok(ch) = self.serial.read() {
                buffer[n] = ch;
                n += 1;
            }
        }

        let s = from_utf8(&buffer[0..n])?;
        let role = Role::try_from(s.trim_end())?;

        Ok(role)
    }

    pub fn reset_setting(&mut self) -> Result<(), Error> {
        self.write_buffer(&RESET_SETTINGS_COMMAND)?;

        if !self.wait_ok_response() {
            Err(Error::WrongResponse)
        } else {
            Ok(())
        }
    }

    pub fn is_ok(&mut self) -> bool {
        self.write_buffer(&OK_QUERY).unwrap();
        self.wait_ok_response()
    }

    pub fn get_parameters(&mut self) -> Result<Parameters, Error> {
        self.write_buffer(&QUERY_PARAMS_COMMAND)?;

        let mut params = [[0u8; 24]; 8];
        let mut param_slices: [&[u8]; 8] = Default::default();
        for (pi, p) in &mut params.iter_mut().enumerate() {
            for (i, v) in p.iter_mut().enumerate() {
                if let Ok(ch) = self.serial.read() {
                    *v = ch;
                    if ch == b'\n' {
                        param_slices[pi] = &p[..=i];
                        break;
                    }
                }
            }
        }

        let role = Role::try_from(param_slices[1])?;
        let baud_rate = BaudRate::try_from(param_slices[2])?;
        let addr = Addr::try_from(param_slices[3])?;

        Ok(Parameters {
            role,
            baud_rate,
            addr,
        })
    }

    pub fn get_version<'a>(&mut self, buffer: &'a mut [u8; 21]) -> Result<&'a str, Error> {
        self.write_buffer(&QUERY_VERSION)?;
        self.read_buffer(buffer)?;

        match from_utf8(buffer) {
            Ok(s) => Ok(s),
            Err(_e) => Err(Error::WrongResponse),
        }
    }

    pub fn get_name<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a str, Error> {
        self.write_buffer(&QUERY_NAME)?;
        self.read_buffer(buffer)?;

        match from_utf8(buffer) {
            Ok(s) => Ok(s),
            Err(_e) => Err(Error::WrongResponse),
        }
    }

    fn change_role(&mut self, role: Role) -> Result<(), Error> {
        let cmd = build_change_role_command(role);
        self.send_command(&cmd)
    }

    fn change_connectable(&mut self, c: IsConnectable) -> Result<(), Error> {
        let cmd = build_change_connectable_command(c);
        self.send_command(&cmd)
    }

    pub fn into_central_mode(mut self) -> ToCentral<S, D, R, C> {
        if let Err(_) = self.change_role(Role::Master) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        if let Err(_) = self.change_connectable(IsConnectable(true)) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        Ok(Hc08 {
            serial: self.serial,
            delay: self.delay,
            role: PhantomData::<Master>,
            connectable: PhantomData::<Connectable>,
        })
    }

    pub fn into_peripheral_mode(mut self) -> ToPeripheral<S, D, R, C> {
        if let Err(_) = self.change_role(Role::Slave) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        if let Err(_) = self.change_connectable(IsConnectable(true)) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        Ok(Hc08 {
            serial: self.serial,
            delay: self.delay,
            role: PhantomData::<Slave>,
            connectable: PhantomData::<Connectable>,
        })
    }

    pub fn into_observer_mode(mut self) -> ToObserver<S, D, R, C> {
        if let Err(_) = self.change_role(Role::Master) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        if let Err(_) = self.change_connectable(IsConnectable(false)) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        Ok(Hc08 {
            serial: self.serial,
            delay: self.delay,
            role: PhantomData::<Master>,
            connectable: PhantomData::<NonConnectable>,
        })
    }

    pub fn into_broadcast_mode(mut self) -> ToBroadcast<S, D, R, C> {
        if let Err(_) = self.change_role(Role::Slave) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        if let Err(_) = self.change_connectable(IsConnectable(false)) {
            return Err(self);
        }
        self.delay.delay_ms(200);

        Ok(Hc08 {
            serial: self.serial,
            delay: self.delay,
            role: PhantomData::<Slave>,
            connectable: PhantomData::<NonConnectable>,
        })
    }
}

impl<S, D> Hc08<S, D, Slave, NonConnectable>
where
    S: Write<u8> + Read<u8>,
    D: DelayMs<u32>,
{
    pub fn change_broadcast_data(&mut self, data: &[u8]) -> Result<(), Error> {
        self.write_buffer(&CHANGE_BROADCAST_BASE)?;
        self.write_buffer(data)?;

        if !self.wait_ok_response() {
            Err(Error::WrongResponse)
        } else {
            Ok(())
        }
    }
}

impl<S, D, R> Hc08<S, D, R, Connectable>
where
    S: Write<u8> + Read<u8>,
    D: DelayMs<u32>,
{
    pub fn change_connect_internal(&mut self, min: u32, max: u32) -> Result<(), Error> {
        let mut buffer = [0; 20];
        let cmd = build_change_connect_internal_command(min, max, &mut buffer);
        self.write_buffer(cmd)?;

        let expect = build_change_connect_internal_response(min, max, &mut buffer);
        let mut response = [0u8; 20];
        self.read_buffer(&mut response[..expect.len()]).unwrap();
        if expect == response {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }

    pub fn change_connect_timeout(&mut self, time: u32) -> Result<(), Error> {
        let mut buffer = [0; 20];
        let cmd = build_change_connect_timeout_command(time, &mut buffer);
        self.write_buffer(cmd)?;

        let expect = build_change_connect_timeout_response(time, &mut buffer);
        let mut response = [0u8; 20];
        self.read_buffer(&mut response[..expect.len()]).unwrap();
        if expect == response {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }
}

impl<S, D> Hc08<S, D, Master, Connectable>
where
    S: Write<u8> + Read<u8>,
    D: DelayMs<u32>,
{
    pub fn clear_slave_addr(&mut self) -> Result<(), Error> {
        self.send_command(&CLEAR_ADDR)
    }

    pub fn query_connect_uuid(&mut self) -> Result<UUID, Error> {
        self.write_buffer(&QUERY_CONNECT_UUID)?;
        let mut buffer = [0; 13];
        self.read_buffer(&mut buffer)?;

        if buffer[..10] != CONNECT_UUID_RESPONSE {
            Err(Error::WrongResponse)
        } else {
            Ok(UUID::try_from(&buffer[10..])?)
        }
    }

    pub fn set_connect_uuid(&mut self, uuid: UUID) -> Result<(), Error> {
        let cmd = build_set_connect_uuid_command(uuid);
        self.write_buffer(&cmd)?;

        let expect = build_set_connect_uuid_response(uuid);
        let mut response = [0u8; 13];
        self.read_buffer(&mut response).unwrap();
        if expect == response {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }
}

impl<S, D> Hc08<S, D, Slave, Connectable>
where
    S: Write<u8> + Read<u8>,
    D: DelayMs<u32>,
{
    pub fn get_service_uuid(&mut self) -> Result<UUID, Error> {
        self.write_buffer(&QUERY_SERVICE_UUID)?;
        let mut buffer = [0; 13];
        self.read_buffer(&mut buffer)?;

        if buffer[..10] != SERVICE_UUID_RESPONSE {
            Err(Error::WrongResponse)
        } else {
            Ok(UUID::try_from(&buffer[10..])?)
        }
    }

    pub fn set_service_uuid(&mut self, uuid: UUID) -> Result<(), Error> {
        let cmd = build_set_service_uuid_command(uuid);
        self.write_buffer(&cmd)?;

        let expect = build_set_service_uuid_response(uuid);
        let mut response = [0u8; 13];
        self.read_buffer(&mut response).unwrap();
        if expect == response {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }

    pub fn get_characteristic_uuid(&mut self) -> Result<UUID, Error> {
        self.write_buffer(&QUERY_CHARACTERISTIC_UUID)?;
        let mut buffer = [0; 13];
        self.read_buffer(&mut buffer)?;

        if buffer[..10] != CHARACTERISTIC_UUID_RESPONSE {
            Err(Error::WrongResponse)
        } else {
            Ok(UUID::try_from(&buffer[10..])?)
        }
    }

    pub fn set_characteristic_uuid(&mut self, uuid: UUID) -> Result<(), Error> {
        let cmd = build_set_characteristic_uuid_command(uuid);
        self.write_buffer(&cmd)?;

        let expect = build_set_characteristic_uuid_response(uuid);
        let mut response = [0u8; 13];
        self.read_buffer(&mut response).unwrap();
        if expect == response {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }
}
