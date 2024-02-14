#![no_main]

use core::panic;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::guest::entry!(main);

fn main() {
    // Read input from the host
    let input: String = env::read();
    let number: u32 = input.parse::<u32>().unwrap();

    if !is_odd(number) {
        // Panic in the guest code won't cause the host to panic
        panic!("One or more input is not odd");
    }

    let result = *Impl::hash_bytes(format!("{}", number).as_bytes());

    // Write public output to the journal.
    // Reference : https://dev.risczero.com/terminology#journal
    env::commit(&result);
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
