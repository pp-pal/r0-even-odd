// CHECK_ODD_ELF is the ELF binary for the check_odd.
// Reference : https://dev.risczero.com/terminology#elf-binary
// CHECK_ODD_ID is the image ID for the check_odd.
// Reference : https://dev.risczero.com/terminology#image-id
use methods::{
    CHECK_ODD_ELF, CHECK_ODD_ID, CHECK_EVEN_ELF, CHECK_EVEN_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Obtain the default prover.
    let prover = default_prover();

    // It will fail since one of the input is not odd
    let input_check_odd_a: u32 = 17;
    let input_check_odd_b: u32 = 24;

    // Add the inputs to the environment.
    let env_check_odd = ExecutorEnv::builder()
        .write(&input_check_odd_a)
        .unwrap()
        .write(&input_check_odd_b)
        .unwrap()
        .build()
        .unwrap();

    // Produce a receipt by proving the specified ELF binary.
    // Reference : https://dev.risczero.com/terminology#receipt
    match prover.prove(env_check_odd, CHECK_ODD_ELF) {
        Ok(receipt_check_odd) => {
            // Get the journal from the receipt and decode it.
            // Reference : https://dev.risczero.com/terminology#journal
            let journal_check_odd: u32 = receipt_check_odd.journal.decode().unwrap();
            info!("Journal check odd : {:?}", journal_check_odd);
            info!("ImageID check odd : {:#?}", CHECK_ODD_ID);

            // The receipt was verified at the end of proving.
            // Typically, you provide the receipt to a third party for verification
            // Reference : https://dev.risczero.com/terminology#image-id
            receipt_check_odd
                .verify(CHECK_ODD_ID)
                .unwrap();
        },
        Err(err) => error!("Failed to generate `check_odd` receipt: {:?}", err)
    }

    // It will succeed since both inputs are even
    let input_check_even_a: u32 = 16;
    let input_check_even_b: u32 = 24;

    // Add the inputs to the environment.
    let env_check_even = ExecutorEnv::builder()
        .write(&input_check_even_a)
        .unwrap()
        .write(&input_check_even_b)
        .unwrap()
        .build()
        .unwrap();

    // Produce a receipt by proving the specified ELF binary.
    // Reference : https://dev.risczero.com/terminology#receipt
    match prover.prove(env_check_even, CHECK_EVEN_ELF) {
        Ok(receipt_check_even) => {
            // Get the journal from the receipt and decode it.
            // Reference : https://dev.risczero.com/terminology#journal
            let journal_check_even: u32 = receipt_check_even.journal.decode().unwrap();
            info!("Journal check even : {:?}", journal_check_even);
            info!("ImageId check even : {:#?}", CHECK_EVEN_ID);

            // The receipt was verified at the end of proving.
            // Typically, you provide the receipt to a third party for verification
            // Reference : https://dev.risczero.com/terminology#image-id
            receipt_check_even
                .verify(CHECK_EVEN_ID)
                .unwrap();
        },
        Err(err) => error!("Failed to generate `check_even` receipt : {:?}", err),
    };
}
