use crate::parameters::{connectable::IsConnectable, role::Role, uuid::UUID};

pub const OK_QUERY: [u8; 2] = *b"AT";
pub const OK_RESPONSE: [u8; 2] = *b"OK";

pub const QUERY_PARAMS_COMMAND: [u8; 5] = *b"AT+RX";

pub const RESET_SETTINGS_COMMAND: [u8; 10] = *b"AT+DEFAULT";
pub const QUERY_VERSION: [u8; 10] = *b"AT+VERSION";

pub const ROLE_BASE: [u8; 8] = *b"AT+ROLE=";
pub const QUERY_ROLE: [u8; 9] = *b"AT+ROLE=?";

pub fn build_change_role_command(role: Role) -> [u8; 9] {
    let mut result = [0; 9];
    result[..8].clone_from_slice(&ROLE_BASE);
    result[8] = match role {
        Role::Master => 'M',
        Role::Slave => 'S',
    } as u8;

    result
}

pub const CONNECTABLE_BASE: [u8; 8] = *b"AT+CONT=";
pub const QUERY_CONNECTABLE: [u8; 9] = *b"AT+CONT=?";

pub fn build_change_connectable_command(c: IsConnectable) -> [u8; 10] {
    let mut result = [0; 10];
    result[..9].clone_from_slice(&CONNECTABLE_BASE);
    result[9] = if c.0 { b'0' } else { b'1' };

    result
}

pub const CHANGE_BROADCAST_BASE: [u8; 8] = *b"AT+AVDA=";

pub const CHANGE_NAME_BASE: [u8; 8] = *b"AT+NAME=";
pub const QUERY_NAME: [u8; 9] = *b"AT+NAME=?";

pub const CLEAR_ADDR: [u8; 8] = *b"AT+CLEAR";

pub const CHANGE_CONNECT_INTERNAL_BASE: [u8; 8] = *b"AT+CINT=";
pub const CHANGE_CONNECT_INTERNAL_RESPONSE: [u8; 8] = *b"OK+CINT=";
pub fn build_change_connect_internal_command<'a>(
    min: u32,
    max: u32,
    buffer: &'a mut [u8],
) -> &'a [u8] {
    buffer.clone_from_slice(&CHANGE_CONNECT_INTERNAL_BASE);
    let mut n = CHANGE_BROADCAST_BASE.len();
    let (min, max) = (min / 4 * 5, max / 4 * 5);
    if min == max {
        n += num2hex(min, &mut buffer[n..]);
    } else {
        n += num2hex(min, &mut buffer[n..]);
        buffer[n] = b',';
        n += 1;
        n += num2hex(max, &mut buffer[n..]);
    }

    &buffer[..n]
}

pub fn build_change_connect_internal_response<'a>(
    min: u32,
    max: u32,
    buffer: &'a mut [u8],
) -> &'a [u8] {
    buffer.clone_from_slice(&CHANGE_CONNECT_INTERNAL_BASE);
    let mut n = CHANGE_BROADCAST_BASE.len();
    let (min, max) = (min / 4 * 5, max / 4 * 5);

    n += num2hex(min, &mut buffer[n..]);
    buffer[n] = b',';
    n += 1;
    n += num2hex(max, &mut buffer[n..]);

    &buffer[..n]
}

pub const CHANGE_CONNECT_TIMEOUT_BASE: [u8; 9] = *b"AT+CTOUT=";
pub const CHANGE_CONNECT_TIMEOUT_RESPONSE: [u8; 9] = *b"OK+CTOUT=";
pub fn build_change_connect_timeout_command<'a>(time: u32, buffer: &'a mut [u8]) -> &'a [u8] {
    buffer.clone_from_slice(&CHANGE_CONNECT_TIMEOUT_BASE);
    let mut n = CHANGE_CONNECT_TIMEOUT_BASE.len();
    let time = time / 10;

    n += num2hex(time, &mut buffer[n..]);

    &buffer[..n]
}

pub fn build_change_connect_timeout_response<'a>(time: u32, buffer: &'a mut [u8]) -> &'a [u8] {
    buffer.clone_from_slice(&CHANGE_CONNECT_TIMEOUT_RESPONSE);
    let mut n = CHANGE_CONNECT_TIMEOUT_RESPONSE.len();
    let time = time / 10;

    n += num2hex(time, &mut buffer[n..]);

    &buffer[..n]
}

pub const SET_CONNECT_UUID_BASE: [u8; 9] = *b"AT+LUUID=";
pub const CONNECT_UUID_RESPONSE: [u8; 9] = *b"OK+LUUID=";
pub const QUERY_CONNECT_UUID: [u8; 10]= *b"AT+LUUID=?";
pub fn build_set_connect_uuid_command(uuid: UUID) -> [u8; 13] {
    let mut result = [0; 13];
    result.clone_from_slice(&SET_CONNECT_UUID_BASE);

    let uuid: [u8; 4] = uuid.into();
    result[SET_CONNECT_UUID_BASE.len()..].clone_from_slice(&uuid);
    result
}

pub fn build_set_connect_uuid_response(uuid: UUID) -> [u8; 13] {
    let mut result = [0; 13];
    result.clone_from_slice(&CONNECT_UUID_RESPONSE);

    let uuid: [u8; 4] = uuid.into();
    result[CONNECT_UUID_RESPONSE.len()..].clone_from_slice(&uuid);
    result
}

pub const SET_SERVICE_UUID_BASE: [u8; 9] = *b"AT+SUUID=";
pub const SERVICE_UUID_RESPONSE: [u8; 9] = *b"OK+SUUID=";
pub const QUERY_SERVICE_UUID: [u8; 10]= *b"AT+SUUID=?";
pub fn build_set_service_uuid_command(uuid: UUID) -> [u8; 13] {
    let mut result = [0; 13];
    result.clone_from_slice(&SET_SERVICE_UUID_BASE);

    let uuid: [u8; 4] = uuid.into();
    result[SET_SERVICE_UUID_BASE.len()..].clone_from_slice(&uuid);
    result
}

pub fn build_set_service_uuid_response(uuid: UUID) -> [u8; 13] {
    let mut result = [0; 13];
    result.clone_from_slice(&SERVICE_UUID_RESPONSE);

    let uuid: [u8; 4] = uuid.into();
    result[SERVICE_UUID_RESPONSE.len()..].clone_from_slice(&uuid);
    result
}

pub const SET_CHARACTERISTIC_UUID_BASE: [u8; 9] = *b"AT+TUUID=";
pub const CHARACTERISTIC_UUID_RESPONSE: [u8; 9] = *b"OK+TUUID=";
pub const QUERY_CHARACTERISTIC_UUID: [u8; 10]= *b"AT+TUUID=?";
pub fn build_set_characteristic_uuid_command(uuid: UUID) -> [u8; 13] {
    let mut result = [0; 13];
    result.clone_from_slice(&SET_CHARACTERISTIC_UUID_BASE);

    let uuid: [u8; 4] = uuid.into();
    result[SET_CHARACTERISTIC_UUID_BASE.len()..].clone_from_slice(&uuid);
    result
}

pub fn build_set_characteristic_uuid_response(uuid: UUID) -> [u8; 13] {
    let mut result = [0; 13];
    result.clone_from_slice(&CHARACTERISTIC_UUID_RESPONSE);

    let uuid: [u8; 4] = uuid.into();
    result[CHARACTERISTIC_UUID_RESPONSE.len()..].clone_from_slice(&uuid);
    result
}

fn num2hex<'a>(mut num: u32, buffer: &'a mut [u8]) -> usize {
    let n = 0;
    while num > 0 {
        buffer[n] = (num % 10) as u8;
        num /= 10;
    }

    buffer[..n].reverse();
    n
}
