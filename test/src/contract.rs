use protocol::{
    traits::{ServiceResponse, ServiceSDK},
    types::{Address, Hash, ServiceContext, ServiceContextParams},
    Bytes,
};
use riscv::{
    types::InterpreterType,
    vm::{ChainInterface, Interpreter, InterpreterParams},
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::helper::{init_logger, read_code, MockChain, SimpleMockChain};

fn get_test_service_context() -> ServiceContext {
    let params = ServiceContextParams {
        tx_hash: Some(
            Hash::from_hex("0x0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap(),
        ),
        nonce: Some(
            Hash::from_hex("0x0000000000000000000000000000000000000000000000000000000000000002")
                .unwrap(),
        ),
        cycles_limit: 1_000_000_000,
        cycles_price: 1,
        cycles_used: Rc::new(RefCell::new(0)),
        caller: Address::from_hex("0x0000000000000000000000000000000000000003").unwrap(),
        height: 100,
        timestamp: 1596260006,
        service_name: "service_name".to_owned(),
        service_method: "service_method".to_owned(),
        service_payload: "service_payload".to_owned(),
        extra: None,
        events: Rc::new(RefCell::new(vec![])),
    };
    let ctx = ServiceContext::new(params);
    ctx
}

#[test]
fn test_contract() {
    init_logger();
    let ctx = get_test_service_context();
    let code = read_code("contract/build/debug/contract");
    let args = "set key value";
    let interpreter_params = InterpreterParams {
        address: Address::from_hex("0x0000000000000000000000000000000000000004").unwrap(),
        code,
        args: Bytes::from(args),
        is_init: false,
    };
    let chain = SimpleMockChain::default();
    let interpreter = Interpreter::new(
        ctx,
        InterpreterType::Binary,
        interpreter_params,
        Rc::new(RefCell::new(chain)),
    );
    let res = interpreter.run();
    dbg!(&res);
    assert_eq!(res.is_ok(), true);
    let res = res.unwrap();
    assert_eq!(res.code, 0);
    assert_eq!(res.data, "");
}
