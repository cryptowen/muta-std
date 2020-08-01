use protocol::{traits::ServiceResponse, types::Address, Bytes};
use riscv::vm::ChainInterface;
use std::{collections::HashMap, fs::File, io::Read};

use mockall::predicate::*;
use mockall::*;

#[derive(Default)]
pub struct SimpleMockChain {
    pub storage: HashMap<Bytes, Bytes>,
}

impl ChainInterface for SimpleMockChain {
    fn get_storage(&self, key: &Bytes) -> Bytes {
        self.storage
            .get(key)
            .map_or_else(|| Bytes::new(), |v| v.clone())
    }

    fn set_storage(&mut self, key: Bytes, val: Bytes) -> ServiceResponse<()> {
        self.storage.insert(key, val);
        ServiceResponse::from_succeed(())
    }

    fn contract_call(
        &mut self,
        _address: Address,
        _args: Bytes,
        _current_cycle: u64,
    ) -> ServiceResponse<(String, u64)> {
        unimplemented!()
    }

    fn service_read(
        &mut self,
        _service: &str,
        _method: &str,
        _payload: &str,
        _current_cycle: u64,
    ) -> ServiceResponse<(String, u64)> {
        unimplemented!()
    }

    fn service_write(
        &mut self,
        _service: &str,
        _method: &str,
        _payload: &str,
        _current_cycle: u64,
    ) -> ServiceResponse<(String, u64)> {
        unimplemented!()
    }
}

mock! {
    pub Chain {}
    pub trait ChainInterface {
        fn get_storage(&self, key: &Bytes) -> Bytes;

        fn set_storage(&mut self, key: Bytes, val: Bytes) -> ServiceResponse<()>;

        fn contract_call(
            &mut self,
            address: Address,
            args: Bytes,
            current_cycle: u64,
        ) -> ServiceResponse<(String, u64)>;

        fn service_read(
            &mut self,
            service: &str,
            method: &str,
            payload: &str,
            current_cycle: u64,
        ) -> ServiceResponse<(String, u64)>;

        fn service_write(
            &mut self,
            service: &str,
            method: &str,
            payload: &str,
            current_cycle: u64,
        ) -> ServiceResponse<(String, u64)>;
    }
}

pub fn init_logger() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Debug)
        .try_init();
}

pub fn read_code(path: &str) -> Bytes {
    let mut file = File::open(path).expect("open code file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("read code file");
    let code = Bytes::from(buffer);
    code
}
