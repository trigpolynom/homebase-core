#![no_main]

use risc0_zkvm::guest::{env, sha};
use homebase_core::{Claim, Patient, Coverage};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let claim: Claim = env::read();
    let patient: Patient = env::read();
    let coverage: Coverage = env::read();

    let claimPolicy = ClaimPolicy{

    };

    if !claimPolicy.check_claim(&claim, &patient, &coverage) {
        panic!("Claim is invalid! You won't be getting reimbursed!");
    }

    env::commit(&claim);

}

struct ClaimPolicy{
    
}

impl ClaimPolicy {
    pub fn check_claim(&self, claim: &Claim, patient: &Patient, coverage: &Coverage) -> bool {
            // Check that the patient ID in the claim matches the ID in the patient resource
    if let Some(patient_ref) = &claim.patient.reference {
        let patient_id = patient_ref.split('/').last().unwrap();
        if patient_id != &patient.identifier {
            return false;
        }
    } else {
        return false;
    }

    // Check that the coverage ID in the claim matches the ID in the coverage resource
    if let Some(coverage_ref) = &claim.insurer.reference {
        let coverage_id = coverage_ref.split('/').last().unwrap();
        if coverage_id != &coverage.id {
            return false;
        }
    } else {
        return false;
    }

    true
    }
}