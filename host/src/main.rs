// use std::{io::prelude::*, fs::File};
// use homebase_core::{Claim, Patient, Coverage};
use methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use json::parse;
use risc0_zkvm::{Prover,
    serde::{from_slice, to_vec}
};

fn main() {

        let claim_contents = include_str!("../../res/provider_resources/claim.json");    
        let patient_contents = include_str!("../../res/patient_resources/patient_details.json");
        let coverage_contents = include_str!("../../res/patient_resources/patient_coverage.json");        

        let claim = parse(&claim_contents).unwrap();
        let patient = parse(&patient_contents).unwrap();
        let coverage = parse(&coverage_contents).unwrap();

        let claim_patient_reference = &claim["patient"];

        let claim_patient = claim_patient_reference["reference"].as_str().unwrap();

        let patient_id = patient["id"].as_str().unwrap();

        let mut prover = Prover::new(VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID).unwrap();

        prover.add_input_u32_slice(&to_vec(&claim_patient).expect("should be serializable")); 
        // prover.add_input_u32_slice(&to_vec(&claim_contents).expect("Error serializing claim"));
        // prover.add_input_u32_slice(&to_vec(&patient_contents).expect("Error serializing patient"));
        // prover.add_input_u32_slice(&to_vec(&coverage_contents).expect("Error serializing coverage"));

        let receipt = prover.run().expect("Code should be provable");

        let journal = &receipt.journal;

        let boolean_output: bool = from_slice(&journal).expect("Journal should contain a boolean showing whether the claim had correct patient.");

        print!("{}", &boolean_output);

}


#[cfg(test)]
mod tests {
    use json::parse;
    // use std::{io::prelude::*, fs::File, ops::Deref};
    // use homebase_core::{Claim, Patient, Coverage, Reference, Insurance};

    #[test]
    fn main() {

        let mut CORRECT_PATIENT: bool = false;

        let claim_contents = include_str!("../../res/provider_resources/claim.json");

        let claim = parse(&claim_contents).unwrap();

        let patient_reference = &claim["patient"];

        let patient = patient_reference["reference"].as_str().unwrap();

        println!("{}", patient);

        let patient_string = "Patient/pat1";

        if &patient == &patient_string {
            CORRECT_PATIENT = true;
        }

        println!("{}", CORRECT_PATIENT);




    }

}