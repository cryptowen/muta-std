use crate::error::SysError;
use crate::muta_constants::*;
use alloc::string::String;

#[link(name = "muta-syscall")]
extern "C" {
    fn syscall(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64;
}

/// Exit, this script will be terminated after the exit syscall.
/// exit code `0` represents verification is success, others represent error code.
pub fn exit(code: u64) -> ! {
    unsafe { syscall(code, 0, 0, 0, 0, 0, 0, SYS_EXIT) };
    loop {}
}

/// Output debug message
///
/// You should use the macro version syscall: `debug!`
///
/// # Arguments
///
/// * `s` - string to output
pub fn debug(mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    unsafe {
        syscall(c_str.as_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_DEBUG);
    }
}

pub fn assert(statement: bool, mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    unsafe {
        syscall(
            statement as u64,
            c_str.as_ptr() as u64,
            0,
            0,
            0,
            0,
            0,
            SYSCODE_ASSERT,
        );
    }
}

pub fn load_args(buf: &mut [u8]) -> Result<u64, SysError> {
    let buf_len = buf.len() as u64;
    let actual_data_len =
        unsafe { syscall(buf.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_LOAD_ARGS) };
    if actual_data_len > buf_len {
        Err(SysError::LengthNotEnough(actual_data_len))
    } else {
        Ok(actual_data_len)
    }
}

pub fn ret(data: &[u8]) {
    unsafe {
        syscall(
            data.as_ptr() as u64,
            data.len() as u64,
            0,
            0,
            0,
            0,
            0,
            SYSCODE_RET,
        );
    }
}

pub fn get_cycle_limit() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYSCODE_CYCLE_LIMIT) }
}

pub fn get_cycle_price() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYSCODE_CYCLE_PRICE) }
}

pub fn get_cycle_used() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYSCODE_CYCLE_USED) }
}

// TODO: right now the hex string of origin is written into the bytes
//       should change to the raw bytes
pub fn get_origin() -> String {
    let mut addr = [0u8; ADDRESS_HEX_LEN];
    unsafe { syscall(addr.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_ORIGIN) };
    String::from_utf8(addr.to_vec()).unwrap()
}

pub fn get_caller() -> String {
    let mut addr = [0u8; ADDRESS_HEX_LEN];
    unsafe { syscall(addr.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_CALLER) };
    String::from_utf8(addr.to_vec()).unwrap()
}

pub fn get_address() -> String {
    let mut addr = [0u8; ADDRESS_HEX_LEN];
    unsafe { syscall(addr.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_ADDRESS) };
    String::from_utf8(addr.to_vec()).unwrap()
}

pub fn get_is_init() -> bool {
    let ret = unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYSCODE_IS_INIT) };
    ret != 0
}

pub fn get_block_height() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYSCODE_BLOCK_HEIGHT) }
}

pub fn get_timestamp() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYSCODE_TIMESTAMP) }
}

pub fn get_tx_hash() -> String {
    let mut hash = [0u8; HASH_HEX_LEN];
    unsafe { syscall(hash.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_TX_HASH) };
    String::from_utf8(hash.to_vec()).unwrap()
}

pub fn get_tx_nonce() -> String {
    let mut hash = [0u8; HASH_HEX_LEN];
    unsafe { syscall(hash.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_TX_NONCE) };
    String::from_utf8(hash.to_vec()).unwrap()
}

pub fn load_extra(buf: &mut [u8]) -> Result<u64, SysError> {
    let buf_len = buf.len() as u64;
    let actual_data_len =
        unsafe { syscall(buf.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, SYSCODE_EXTRA) };
    if actual_data_len > buf_len {
        Err(SysError::LengthNotEnough(actual_data_len))
    } else {
        Ok(actual_data_len)
    }
}

pub fn get_storage(key: &[u8], value: &mut [u8]) -> Result<u64, SysError> {
    let buf_len = value.len() as u64;
    let actual_data_len = unsafe {
        syscall(
            key.as_ptr() as u64,
            key.len() as u64,
            value.as_mut_ptr() as u64,
            0,
            0,
            0,
            0,
            SYSCODE_GET_STORAGE,
        )
    };
    if actual_data_len > buf_len {
        Err(SysError::LengthNotEnough(actual_data_len))
    } else {
        Ok(actual_data_len)
    }
}

pub fn set_storage(key: &[u8], value: &[u8]) {
    unsafe {
        syscall(
            key.as_ptr() as u64,
            key.len() as u64,
            value.as_ptr() as u64,
            value.len() as u64,
            0,
            0,
            0,
            SYSCODE_SET_STORAGE,
        )
    };
}

pub fn contract_call(
    address: &[u8; ADDRESS_HEX_LEN],
    args: &str,
    ret: &mut [u8],
) -> Result<u64, SysError> {
    let buf_len = ret.len() as u64;
    let actual_data_len = unsafe {
        syscall(
            address.as_ptr() as u64,
            args.as_ptr() as u64,
            args.len() as u64,
            ret.as_mut_ptr() as u64,
            0,
            0,
            0,
            SYSCODE_CONTRACT_CALL,
        )
    };
    if actual_data_len > buf_len {
        Err(SysError::LengthNotEnough(actual_data_len))
    } else {
        Ok(actual_data_len)
    }
}

pub fn service_call(
    service: &str,
    method: &str,
    payload: &[u8],
    ret: &mut [u8],
) -> Result<u64, SysError> {
    let buf_len = ret.len() as u64;
    let actual_data_len = unsafe {
        syscall(
            service.as_ptr() as u64,
            method.as_ptr() as u64,
            payload.as_ptr() as u64,
            payload.len() as u64,
            ret.as_mut_ptr() as u64,
            0,
            0,
            SYSCODE_SERVICE_CALL,
        )
    };
    if actual_data_len > buf_len {
        Err(SysError::LengthNotEnough(actual_data_len))
    } else {
        Ok(actual_data_len)
    }
}

pub fn service_write(
    service: &str,
    method: &str,
    payload: &[u8],
    ret: &mut [u8],
) -> Result<u64, SysError> {
    let buf_len = ret.len() as u64;
    let actual_data_len = unsafe {
        syscall(
            service.as_ptr() as u64,
            method.as_ptr() as u64,
            payload.as_ptr() as u64,
            payload.len() as u64,
            ret.as_mut_ptr() as u64,
            0,
            0,
            SYSCODE_SERVICE_WRITE,
        )
    };
    if actual_data_len > buf_len {
        Err(SysError::LengthNotEnough(actual_data_len))
    } else {
        Ok(actual_data_len)
    }
}

pub fn service_read(
    service: &str,
    method: &str,
    payload: &[u8],
    ret: &mut [u8],
) -> Result<u64, SysError> {
    let buf_len = ret.len() as u64;
    let actual_data_len = unsafe {
        syscall(
            service.as_ptr() as u64,
            method.as_ptr() as u64,
            payload.as_ptr() as u64,
            payload.len() as u64,
            ret.as_mut_ptr() as u64,
            0,
            0,
            SYSCODE_SERVICE_READ,
        )
    };
    if actual_data_len > buf_len {
        Err(SysError::LengthNotEnough(actual_data_len))
    } else {
        Ok(actual_data_len)
    }
}

pub fn emit_event(name: &[u8], event: &[u8]) {
    unsafe {
        syscall(
            name.as_ptr() as u64,
            name.len() as u64,
            event.as_ptr() as u64,
            event.len() as u64,
            0,
            0,
            0,
            SYSCODE_SET_STORAGE,
        )
    };
}
