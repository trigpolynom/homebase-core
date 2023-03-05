// use std::{io::prelude::*, fs::File};
// use homebase_core::{Claim, Patient, Coverage};
use methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use risc0_zkvm::{Prover,
    serde::{from_slice, to_vec}
};

fn main() {

        // let mut file = File::open("res/provider_resources/claim.json").unwrap();
        // let mut file2 = File::open("res/patient_resources/patient_details.json").unwrap();
        // let mut file3 = File::open("res/patient_resources/patient_coverage.json").unwrap();
    
        let claim_contents = include_str!("../../res/provider_resources/claim.json");
        // let mut claim_contents = String::new();
        // file.read_to_string(&mut claim_contents).unwrap();
    
        // let patient_contents = include_str!("../../res/patient_resources/patient_details.json");
        // let mut patient_contents = String::new();
        // file2.read_to_string(&mut patient_contents).unwrap();
    
        // let coverage_contents = include_str!("../../res/patient_resources/patient_coverage.json");        
        // let mut coverage_contents = String::new();
        // file3.read_to_string(&mut coverage_contents).unwrap();
    

        let mut prover = Prover::new(VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID).unwrap();


        prover.add_input_u32_slice(&to_vec(&claim_contents).expect("Error serializing claim"));
        // prover.add_input_u32_slice(&to_vec(&patient_contents).expect("Error serializing patient"));
        // prover.add_input_u32_slice(&to_vec(&coverage_contents).expect("Error serializing coverage"));

        let receipt = prover.run().expect("Code should be provable");

        let journal = &receipt.journal;

        let boolean_output: bool = from_slice(&journal).expect("Journal should contain a boolean showing whether the claim had correct patient.");

        print!("{}", &journal[0]);

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