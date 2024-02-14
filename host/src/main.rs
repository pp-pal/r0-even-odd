// CHECK_ODD_ELF is the ELF binary for the check_odd.
// Reference : https://dev.risczero.com/terminology#elf-binary
// CHECK_ODD_ID is the image ID for the check_odd.
// Reference : https://dev.risczero.com/terminology#image-id
use methods::{
    CHECK_ODD_ELF, CHECK_ODD_ID,
    CHECK_EVEN_ELF, CHECK_EVEN_ID,
    CHECK_ADULT_ELF, CHECK_ADULT_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::Local;

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    is_adult: bool,
}

fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} <method_name> <input_val>", args[0]);
    }
    let method = &args[1];
    let input_val = &args[2];

    info!("Method: {}", method);
    info!("Input: {}", input_val);

    let method_name = method.as_str();
    let(method_elf, method_id, method_input) = match method_name {
        "check_odd" => {
            (CHECK_ODD_ELF, CHECK_ODD_ID, input_val.clone())
        },
        "check_even" => {
            (CHECK_EVEN_ELF, CHECK_EVEN_ID, input_val.clone())
        },
        "check_adult" => {
            let today = Local::now();
            let today_str = today.format("%Y-%m-%d").to_string();
            let input_check_adult = format!("{}|{}", input_val, today_str);
            (CHECK_ADULT_ELF, CHECK_ADULT_ID, input_check_adult)
        },
        _ => panic!("Unknown method: {}", method),
    };

    // Obtain the default prover.
    let prover = default_prover();

    // Add the inputs to the environment.
    let method_env = ExecutorEnv::builder()
        .write(&method_input)
        .unwrap()
        .build()
        .unwrap();

    // Produce a receipt by proving the specified ELF binary.
    // Reference : https://dev.risczero.com/terminology#receipt
    match prover.prove(method_env, method_elf) {
        Ok(method_receipt) => {
            // Get the journal from the receipt and decode it.
            // Reference : https://dev.risczero.com/terminology#journal
            if method_name == "check_adult" {
                let journal: Result = method_receipt.journal.decode().unwrap();
                info!("Journal {} : {:#?}", method_name, journal);
                info!("ImageID {} : {:#?}", method_name, method_id);
            } else {
                let journal: u32 = method_receipt.journal.decode().unwrap();
                info!("Journal {} : {:?}", method_name, journal);
                info!("ImageID {} : {:#?}", method_name, method_id);
            }

            // The receipt was verified at the end of proving.
            // Typically, you provide the receipt to a third party for verification
            // Reference : https://dev.risczero.com/terminology#image-id
            method_receipt
                .verify(method_id)
                .unwrap();
        },
        Err(err) => error!("Failed to generate `{}` receipt: {:?}", method_name, err)
    }
}
