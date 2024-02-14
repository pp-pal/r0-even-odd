## Description

This is a simple example of RISC Zero code. It includes three methods, `check_odd`, `check_even` and `check_adult`, that will be executed inside the RISC Zero zkVM.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [RISC Zero](https://dev.risczero.com/api/zkvm/install)

## Directory Structure

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│   	└── main.rs                <-- [Host code goes here]
└── methods
	├── Cargo.toml                 <-- [Defines the guest methods here]
	├── build.rs
	├── check-adult
	│   ├── Cargo.toml
	│   └── src
	│   	└── main.rs              <-- [The check-adult guest code goes here]
	├── check-even
	│   ├── Cargo.toml
	│   └── src
	│   	└── main.rs              <-- [The check-even guest code goes here]
	├── check-odd
	│   ├── Cargo.toml
	│   └── src
	│   	└── main.rs              <-- [The check-odd guest code goes here]
	└── src
    	   └── lib.rs
```

## How To Run

- Build project
```bash
cargo build -r
```
- Run `check_even`
```bash
./target/release/host check_even 18
```
- Run `check_odd`
```bash
./target/release/host check_odd 19
```
- Run `check_adult`
```bash
./target/release/host check_adult 2006-02-18
```

## References

- [RISC Zero Host](https://dev.risczero.com/api/zkvm/guest-code-101)
- [RISC Zero Guest](https://dev.risczero.com/api/zkvm/host-code-101)
- [Generate RISC Zero New Project](https://dev.risczero.com/api/zkvm/quickstart)