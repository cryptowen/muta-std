#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::string::{String, ToString};
use muta_std::{debug, default_alloc, entry, error::SysError, high_level, syscalls};

#[derive(Debug, Clone)]
pub enum Error {
    SysError(SysError),
    // user defined errors here
    ArgsErr(String),
}

impl Error {
    fn to_error(&self) -> (u64, String) {
        let msg = alloc::format!("{:?}", self);
        let code = match self {
            Self::SysError(_e) => 1,
            Self::ArgsErr(_e) => 2,
        };
        (code, msg)
    }
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        Self::SysError(err)
    }
}

pub fn main() -> Result<String, Error> {
    let dbg_msg = "This is a debug message";
    debug!("{:?}", dbg_msg);
    let args = high_level::load_args_string()?;
    debug!("args: {:?}", args);

    // env
    let cycle_limit = syscalls::get_cycle_limit();
    debug!("cycle_limit: {:?}", cycle_limit);
    let cycle_price = syscalls::get_cycle_price();
    debug!("cycle_price: {:?}", cycle_price);
    let cycle_used = syscalls::get_cycle_used();
    debug!("cycle_used: {:?}", cycle_used);
    let origin = syscalls::get_origin();
    debug!("origin: {:?}", origin);
    let caller = syscalls::get_caller();
    debug!("caller: {:?}", caller);
    let address = syscalls::get_address();
    debug!("address: {:?}", address);
    let is_init = syscalls::get_is_init();
    debug!("is_init: {:?}", is_init);
    let block_height = syscalls::get_block_height();
    debug!("block_height: {:?}", block_height);
    let timestamp = syscalls::get_timestamp();
    debug!("timestamp: {:?}", timestamp);
    let tx_hash = syscalls::get_tx_hash();
    debug!("tx_hash: {:?}", tx_hash);
    let tx_nonce = syscalls::get_tx_nonce();
    debug!("tx_nonce: {:?}", tx_nonce);
    let extra = high_level::load_extra_string()?;
    debug!("extra: {:?}", extra);

    // storage
    let key = "key";
    let value = high_level::get_storage_string(key)?;
    debug!("value before set: {:?}", value);
    high_level::set_storage_string(key, "value");
    let value = high_level::get_storage_string(key)?;
    debug!("value after set: {:?}", value);

    Ok("".to_string())
}

pub fn _main() -> Result<String, (u64, String)> {
    main().map_err(|e| e.to_error())
}

entry!(_main);
default_alloc!();
