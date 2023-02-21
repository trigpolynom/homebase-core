use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use homebase_core::{Claim, Patient, Coverage};
use risc0_zkvm::host::Receipt;
use risc0_zkvm::host::Prover;


pub struct ClaimPolicy{}

impl ClaimPolicy {
    fn adjudicate(&self, claim: &Claim, patient: &Patient, coverage: &Coverage) -> Result<(Receipt)> {
        Ok(())
    }
}

#[test]
fn parse_claim() -> Result<()> {

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


    Ok(())

}