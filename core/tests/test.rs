use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use homebase_core::{Claim, Patient, Coverage};
use methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use risc0_zkvm::{Prover, serde::to_vec};


#[test]
fn parse_claim() {

    let mut file = File::open("res/provider_resources/claim.json")?;
    let mut file2 = File::open("res/patient_resources/patient_details.json")?;
    let mut file3 = File::open("res/patient_resources/patient_coverage.json")?;

    let mut claim_contents = String::new();
    file.read_to_string(&mut claim_contents)?;

    let mut patient_contents = String::new();
    file2.read_to_string(&mut patient_contents)?;

    let mut coverage_contents = String::new();
    file3.read_to_string(&mut coverage_contents)?;

    let claim: Result<Claim, serde_json::Error> = serde_json::from_str(&claim_contents);
    match claim {
        Ok(c) => println!("{:?}", c),
        Err(e) => eprintln!("Error deserializing claim: {}", e),
    }

    let patient: Result<Patient, serde_json::Error> = serde_json::from_str(&patient_contents);
    match patient {
        Ok(p) => println!("{:?}", p),
        Err(e) => eprintln!("Error deserializing patient: {}", e),
    }

    let coverage: Result<Coverage, serde_json::Error> = serde_json::from_str(&coverage_contents);
    match coverage {
        Ok(c) => println!("{:?}", c),
        Err(e) => eprintln!("Error deserializing coverage: {}", e),
    }

    let mut prover = Prover::new(VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID)
                                                        .expect("Prover should be constructed from matching method code & ID");

    let vec1 = to_vec(&claim).unwrap();

    let vec2 = to_vec(&patient).unwrap();

    let vec3 = to_vec(&coverage).unwrap();

    prover.add_input_u32_slice(&vec1);
    prover.add_input_u32_slice(&vec2);
    prover.add_input_u32_slice(&vec3);


    let receipt = prover.run().unwrap();

    // Ok(())

}