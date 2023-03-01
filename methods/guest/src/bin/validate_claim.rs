#![no_main]

use risc0_zkvm::guest::env;
use homebase_core::{Claim, Patient, Coverage};


risc0_zkvm::guest::entry!(main);

pub fn main() {

    let mut CORRECT_COVERAGE: bool = false;

    let claim_contents: String = env::read();
    let patient_contents: String = env::read();
    let coverage_contents: String = env::read();

    let claim: Result<Claim, serde_json::Error> = serde_json::from_str(&claim_contents);
        match claim {
            Ok(ref c) => println!("{:?}", c),
            Err(ref e) => eprintln!("Error deserializing claim: {}", e),
        }
    
        let patient: Result<Patient, serde_json::Error> = serde_json::from_str(&patient_contents);
        match patient {
            Ok(ref p) => println!("{:?}", p),
            Err(ref e) => eprintln!("Error deserializing patient: {}", e),
        }
    
        let coverage: Result<Coverage, serde_json::Error> = serde_json::from_str(&coverage_contents);
        match coverage {
            Ok(ref c) => println!("{:?}", c),
            Err(ref e) => eprintln!("Error deserializing coverage: {}", e),
        }


    let claimInsuranceVec = &claim.unwrap().insurance.ok_or("some error");

    let claimInsurance = &claimInsuranceVec.as_ref().unwrap()[0];

    let patientInsurance = &coverage.unwrap().id;

    let binding = claimInsurance.coverage.reference.as_ref().unwrap();
    let split = binding.split('/');

    let vec = split.collect::<Vec<&str>>();

    let claimInsuranceCoverage = vec.get(1);

    if &patientInsurance == &claimInsuranceCoverage.unwrap() {
        CORRECT_COVERAGE = true;
    }

    // let claimPolicy = ClaimPolicy{

    // };

    // if !claimPolicy.check_claim(&claim, &patient, &coverage) {
    //     panic!("Claim is invalid! You won't be getting reimbursed!");
    // }

    env::commit(&CORRECT_COVERAGE);

}

// struct ClaimPolicy{
    
// }

// impl ClaimPolicy {
//     pub fn check_claim(&self, claim: &Claim, patient: &Patient, coverage: &Coverage) -> bool {
//             // Check that the patient ID in the claim matches the ID in the patient resource
//     if let Some(patient_ref) = &claim.patient.reference {
//         let patient_id = patient_ref.split('/').last().unwrap();
//         if patient_id != &patient.identifier {
//             return false;
//         }
//     } else {
//         return false;
//     }

//     // Check that the coverage ID in the claim matches the ID in the coverage resource
//     if let Some(coverage_ref) = &claim.insurer.reference {
//         let coverage_id = coverage_ref.split('/').last().unwrap();
//         if coverage_id != &coverage.id {
//             return false;
//         }
//     } else {
//         return false;
//     }

//     true
//     }
// }