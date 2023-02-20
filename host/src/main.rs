use std::io::prelude::*;

// use homebase_core::Outputs;
use methods::{SEARCH_JSON_ID, SEARCH_JSON_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    let mut file =
        std::fs::File::open("res/patient_resources/patient_details.json").expect("Example file should be accessible");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Should not have I/O errors");

    let mut file2 =
        std::fs::File::open("res/provider_resources/claim.json").expect("Example file should be accessible");
    let mut data2 = String::new();
    file2.read_to_string(&mut data2)
        .expect("Should not have I/O errors");


    // Make the prover.
    let method_code = std::fs::read(SEARCH_JSON_PATH).expect("Method code should be at path");
    let mut prover = Prover::new(&method_code, SEARCH_JSON_ID)
        .expect("Prover should be constructed from matching method code & ID");

    prover.add_input(&to_vec(&data).unwrap()).unwrap();
    prover.add_input(&to_vec(&data2).unwrap()).unwrap();

    // Run prover & generate receipt
    let receipt = prover.run().expect("Code should be provable");

    receipt
        .verify(SEARCH_JSON_ID)
        .expect("Proven code should verify");

    let journal = &receipt
        .get_journal_vec()
        .expect("Receipt should have journal");
    // let outputs: Outputs = from_slice(&journal).expect("Journal should contain an Outputs object");

    // println!("\nThe JSON files with hashes\n{}, {}\nprovably contains the same critical value\n", outputs.hash, outputs.hash2);
}