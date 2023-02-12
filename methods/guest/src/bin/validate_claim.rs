#![no_main]

use json::parse;
use homebase_core::Outputs;
use risc0_zkvm_guest::{env, sha};
use fhir_rs::{fhir_parse, model};

risc0_zkvm_guest::entry!(main);

pub fn main() {


    let data: String = env::read();
    let data2: String = env::read();

    let sha = sha::digest(&data.as_bytes());
    let sha2 = sha::digest(&data2.as_bytes());

    let data = parse(&data).unwrap();
    let data2 = parse(&data2).unwrap();
}


pub struct ClaimPolicy {
    patient: Patient,
    claim: Claim,
    coverage: Coverage,
}


impl ClaimPolicy {

    pub fn is_valid(&self, claim: Claim, patient: Patient, coverage: Coverage) -> bool {
        self.correct_patient(claim, patient);
    }

    fn correct_patient(&self, claim: Claim, patient: Patient) -> bool {
        
    }
}