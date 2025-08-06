<<<<<<< HEAD
=======

>>>>>>> 03c2d51 (Initial commit)
# 🛡️ Smart Contract Tester

A Rust-based framework for validating smart contract logic using async testing, fuzzing infrastructure, and modular mocks. Designed to simulate edge-case behaviors and enhance testability of decentralized systems.

## 🚀 Why This Project?

As part of my interest in cybersecurity, blockchain systems, and secure infrastructure design, this project explores how automated testing frameworks can help detect vulnerabilities and increase confidence in smart contract behavior. It reflects the skills required in security research, decentralized infrastructure, and threat modeling.

## 🔍 Features

<<<<<<< HEAD
- ✅ **Async testing** using the [tokio](https://tokio.rs/) runtime
- 🔁 **Fuzz testing** for boundary and input validation
- 🔧 **Mocked drivers** to simulate real-world contract input/output
=======
- ✅ Async testing using the [tokio](https://tokio.rs/) runtime
- 🔁 Fuzz testing for boundary and input validation
- 🔧 Mocked drivers to simulate real-world contract input/output
>>>>>>> 03c2d51 (Initial commit)
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
<<<<<<< HEAD
=======
```

## 🧠 Skills Demonstrated

| Skill Category          | Description |
|------------------------|-------------|
| Rust Programming       | Wrote safe, modular, and async Rust code using industry tooling |
| Cybersecurity Awareness| Simulated edge cases to catch unexpected behavior or failure states |
| Research & Analysis    | Designed test cases based on real-world vulnerabilities in smart contracts |
| Fuzz Testing           | Integrated fuzzing to automate detection of corner-case bugs |
| Detail-Oriented Dev    | Created comprehensive testing coverage and clean documentation |
| Collaboration-Ready    | Clear structure, comments, and mock environments for cross-team usability |

## ⚙️ Tech Stack

- Language: Rust
- Runtime: Tokio
- Testing Tools: cargo test, cargo-fuzz
- Serialization: Serde
- Async/Mocks: tokio, custom modules

## 📦 Getting Started

```bash
git clone https://github.com/ShreyasB1/smart-contract-tester.git
cd smart-contract-tester
cargo test
```

## 📁 Folder Structure

```
smart-contract-tester/
├── src/
│   ├── lib.rs
├── fuzz/
│   └── fuzz_target_1.rs
├── tests/
│   └── integration.rs
├── Cargo.toml
├── README.md
```

## ✍️ Author

**Shreyas Balawatri**  
📧 sbalawatri7@gatech.edu  
[LinkedIn](https://www.linkedin.com/in/shreyas-balawatri-579615200) • [GitHub](https://github.com/ShreyasB1)
>>>>>>> 03c2d51 (Initial commit)
