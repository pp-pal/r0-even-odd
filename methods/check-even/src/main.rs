#![no_main]

use std::format;
use core::panic;

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::guest::entry!(main);

fn main() {
    // Read input from the host
    let number_a: u32 = env::read();
    let number_b: u32 = env::read();

    if !is_even(number_a) || !is_even(number_b) {
        // Panic in the guest code won't cause the host to panic
        panic!("One or more input is not even");
    }

    let result = *Impl::hash_bytes(format!("{}_{}", number_a, number_b).as_bytes());

    // Write public output to the journal.
    // Reference : https://dev.risczero.com/terminology#journal
    env::commit(&result);
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}
