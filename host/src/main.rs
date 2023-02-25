use std::{io::prelude::*, fs::File};
use homebase_core::{Claim, Patient, Coverage};
use methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use risc0_zkvm::{Prover, serde::to_vec};

fn main() {

    let mut file = File::open("res/provider_resources/claim.json");
    let mut file2 = File::open("res/patient_resources/patient_details.json");
    let mut file3 = File::open("res/patient_resources/patient_coverage.json");

    let mut claim_contents = String::new();
    file.expect("can be read").read_to_string(&mut claim_contents);

    let mut patient_contents = String::new();
    file2.expect("can be read").read_to_string(&mut patient_contents);

    let mut coverage_contents = String::new();
    file3.expect("can be read").read_to_string(&mut coverage_contents);

    let claim: Claim = serde_json::from_str(&claim_contents).expect("Error deserializing claim");
    let patient: Patient = serde_json::from_str(&patient_contents).expect("Error deserializing patient");
    let coverage: Coverage = serde_json::from_str(&coverage_contents).expect("Error deserializing coverage");


    let mut prover = Prover::new(VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID).unwrap();

    prover.add_input_u32_slice(&to_vec(&claim).expect("Error serializing claim"));
    prover.add_input_u32_slice(&to_vec(&patient).expect("Error serializing patient"));
    prover.add_input_u32_slice(&to_vec(&coverage).expect("Error serializing coverage"));

    let receipt = prover.run().expect("Code should be provable");

    let journal = &receipt.journal;

}


#[cfg(test)]
mod tests {
    use std::{io::prelude::*, fs::File, ops::Deref};
    use homebase_core::{Claim, Patient, Coverage, Reference, Insurance};

    #[test]
    fn main() {

        let mut CORRECT_COVERAGE: bool = false;
    
        let mut file = File::open("res/provider_resources/claim.json").unwrap();
        let mut file2 = File::open("res/patient_resources/patient_details.json").unwrap();
        let mut file3 = File::open("res/patient_resources/patient_coverage.json").unwrap();
    
        let mut claim_contents = String::new();
        file.read_to_string(&mut claim_contents).unwrap();
    
        let mut patient_contents = String::new();
        file2.read_to_string(&mut patient_contents).unwrap();
    
        let mut coverage_contents = String::new();
        file3.read_to_string(&mut coverage_contents).unwrap();
    
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

        let patientInsurance = coverage.unwrap().id;

        if patientInsurance == claimInsurance.coverage.id {
            CORRECT_COVERAGE = true;
        }



    }

}