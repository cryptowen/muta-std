use crate::error::SysError;
use crate::syscalls;
use alloc::string::String;
use alloc::vec::Vec;

pub const BUF_SIZE: usize = 1024;

pub fn load_args_bytes() -> Result<Vec<u8>, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    let len = syscalls::load_args(&mut buf)?;
    Ok(buf[..len as usize].to_vec())
}

pub fn load_args_string() -> Result<String, SysError> {
    Ok(String::from_utf8(load_args_bytes()?)?)
}

pub fn load_extra_bytes() -> Result<Vec<u8>, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    let len = syscalls::load_extra(&mut buf)?;
    Ok(buf[..len as usize].to_vec())
}

pub fn load_extra_string() -> Result<String, SysError> {
    Ok(String::from_utf8(load_extra_bytes()?)?)
}

pub fn emit_event_string(name: &str, event: &str) {
    syscalls::emit_event(name.as_bytes(), event.as_bytes())
}

pub fn get_storage_bytes(key: &[u8]) -> Result<Vec<u8>, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    let len = syscalls::get_storage(key, &mut buf)?;
    Ok(buf[..len as usize].to_vec())
}

pub fn get_storage_string(key: &str) -> Result<String, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    let len = syscalls::get_storage(key.as_bytes(), &mut buf)?;
    Ok(String::from_utf8(buf[..len as usize].to_vec())?)
}

pub fn set_storage_string(key: &str, value: &str) {
    syscalls::set_storage(key.as_bytes(), value.as_bytes())
}
