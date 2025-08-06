# 🛡️ Smart Contract Tester

A Rust-based framework for validating smart contract logic using async testing, fuzzing infrastructure, and modular mocks. Designed to simulate edge-case behaviors and enhance testability of decentralized systems.

## 🚀 Why This Project?

As part of my interest in cybersecurity, blockchain systems, and secure infrastructure design, this project explores how automated testing frameworks can help detect vulnerabilities and increase confidence in smart contract behavior. It reflects the skills required in security research, decentralized infrastructure, and threat modeling.

## 🔍 Features

- ✅ **Async testing** using the [tokio](https://tokio.rs/) runtime
- 🔁 **Fuzz testing** for boundary and input validation
- 🔧 **Mocked drivers** to simulate real-world contract input/output
- 📊 Lightweight infrastructure for modular testing
- 📁 Designed with secure coding and test-first principles

## 🧪 Example Test

```rust
#[tokio::test]
async fn test_valid_transaction() {
    let input = ContractInput {
        caller: "0xabc".to_string(),
        value: 100,
    };
    assert!(validate_transaction(&input).is_ok());
}
