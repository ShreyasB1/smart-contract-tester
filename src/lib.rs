
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInput {
    pub caller: String,
    pub value: u64,
}

pub fn validate_transaction(input: &ContractInput) -> Result<(), &'static str> {
    if input.value == 0 {
        Err("Value cannot be zero")
    } else if input.caller.is_empty() {
        Err("Caller must be defined")
    } else {
        Ok(())
    }
}
