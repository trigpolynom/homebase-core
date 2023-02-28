use std::{io::prelude::*, fs::File, stringify};
use homebase_core::{Claim, Patient, Coverage};
use methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use risc0_zkvm::{Prover, serde::to_vec};
fn main() {

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

    let mut prover = Prover::new(VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID).unwrap();

    // let claim_bytes = serde_json::to_vec(&claim.unwrap()).expect("Error serializing claim");
    // prover.add_input_u8_slice(&claim_bytes);

    // let patient_bytes = serde_json::to_vec(&patient.unwrap()).expect("Error serializing patient");
    // prover.add_input_u8_slice(&patient_bytes);

    // let coverage_bytes = serde_json::to_vec(&coverage.unwrap()).expect("Error serializing coverage");
    // prover.add_input_u8_slice(&coverage_bytes);

    let claim_to_add = &claim.unwrap();
    let patient_to_add = &patient.unwrap();
    let coverage_to_add = &coverage.unwrap();

    prover.add_input_u32_slice(&to_vec(claim_to_add).expect("Error serializing claim"));
    prover.add_input_u32_slice(&to_vec(patient_to_add).expect("Error serializing patient"));
    prover.add_input_u32_slice(&to_vec(coverage_to_add).expect("Error serializing coverage"));

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

        let patientInsurance = &coverage.unwrap().id;

        let binding = claimInsurance.coverage.reference.as_ref().unwrap();
        let split = binding.split('/');

        let vec = split.collect::<Vec<&str>>();

        let claimInsuranceCoverage = vec.get(1);




        if &patientInsurance == &claimInsuranceCoverage.unwrap() {
            CORRECT_COVERAGE = true;
        }



    }

}