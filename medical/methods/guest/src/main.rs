//methods/guest/src/main.rs

#![no_main]

use medical_core::{Outputs, Inputs};
use risc0_zkvm::{
    guest::env,
    serde::from_slice,
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let proof_input: Inputs = env::read();

    let sha = match proof_input.to_digest() {
        Ok(digest) => digest,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    let patient_id_match = &proof_input.patient_id_from_patient == &proof_input.patient_id_from_claim;

    let calculated_coinsurance = (proof_input.eligible_amount as f64) * (proof_input.coinsurance_pecentage as f64) / 100.0;        
    let expected_payment = (proof_input.eligible_amount as f64) * (1.0 - (proof_input.coinsurance_pecentage as f64) / 100.0);

    let epsilon = 0.01;
    let coinsurance_match = (calculated_coinsurance - (proof_input.coinsurance_amount as f64)).abs() < epsilon;     

    let payment_match = (expected_payment - (proof_input.payment as f64)).abs() < epsilon;

    let validated = patient_id_match && coinsurance_match && payment_match;

    let out = Outputs {
        success: validated,
        hash: sha,
        final_payment: Some(proof_input.payment),
    };

    env::commit(&out);
}

