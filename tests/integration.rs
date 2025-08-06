
use smart_contract_tester::{ContractInput, validate_transaction};

#[test]
fn test_valid_transaction() {
    let input = ContractInput {
        caller: "0xabc".to_string(),
        value: 100,
    };
    assert!(validate_transaction(&input).is_ok());
}

#[test]
fn test_invalid_transaction_zero_value() {
    let input = ContractInput {
        caller: "0xabc".to_string(),
        value: 0,
    };
    assert!(validate_transaction(&input).is_err());
}
