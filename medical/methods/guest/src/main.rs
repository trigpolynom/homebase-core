//methods/guest/src/main.rs

#![no_main]

use medical_core::{Outputs, Inputs};
use risc0_zkvm::{
    guest::env,
    serde::from_slice,
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let serialized_inputs: Vec<u32> = env::read();

    let proof_input: Inputs = match from_slice(&serialized_inputs) {
        Ok(inputs) => inputs,
        Err(error) => {
            eprintln!("Error deserializing Inputs: {}", error);
            return;
        }
    };
    let sha = match proof_input.to_digest() {
        Ok(digest) => digest,
        Err(error) => {
            // Handle the error here, you can print the error message or take other appropriate action
            eprintln!("Error: {}", error);
            return;
        }
    };
    // Verify patient_id_from_patient equals patient_id_from_claim
    let patient_id_match = &proof_input.patient_id_from_patient == &proof_input.patient_id_from_claim;

    // Verify eligible_amount * coinsurance_percentage = eligible_amount - coinsurance_amount
    let calculated_coinsurance = &proof_input.eligible_amount * &proof_input.coinsurance_pecentage / 100.0;
    let expected_coinsurance = &proof_input.eligible_amount - &proof_input.coinsurance_amount;
    let epsilon = 0.01;
    let coinsurance_match = (calculated_coinsurance - expected_coinsurance).abs() < epsilon;

    let validated = patient_id_match && coinsurance_match;

    let out = Outputs {
        success: validated,
        hash: sha,
        final_payment: Some(*&proof_input.eligible_amount as i64),
    };
    
    env::commit(&out);
}
