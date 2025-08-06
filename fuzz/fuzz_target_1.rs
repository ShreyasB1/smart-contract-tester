
#![no_main]
use libfuzzer_sys::fuzz_target;
use smart_contract_tester::{ContractInput, validate_transaction};
use serde_json;

fuzz_target!(|data: &[u8]| {
    if let Ok(input_str) = std::str::from_utf8(data) {
        if let Ok(input) = serde_json::from_str::<ContractInput>(input_str) {
            let _ = validate_transaction(&input);
        }
    }
});
