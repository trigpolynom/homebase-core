#![no_main]

use risc0_zkvm::guest::env;
use json::parse;

// // use homebase_core::{Claim, Patient, Coverage};


risc0_zkvm::guest::entry!(main);

pub fn main() {

    let mut CORRECT_PATIENT: bool = false;

    let claim_contents: String = env::read();

    let claim = parse(&claim_contents).unwrap();

    let patient_reference = &claim["patient"];

    let patient = patient_reference["reference"].as_str().unwrap();

    println!("{}", patient);

    let patient_string = "Patient/pat1";

    if &patient == &patient_string {
        CORRECT_PATIENT = true;
    }
    env::commit(&CORRECT_PATIENT);



//     // let patient_contents: String = env::read();
//     // let coverage_contents: String = env::read();

//     // let claim: Result<Claim, serde_json::Error> = serde_json::from_str(&claim_contents);
//     //     match claim {
//     //         Ok(ref c) => println!("{:?}", c),
//     //         Err(ref e) => eprintln!("Error deserializing claim: {}", e),
//     //     }
    
//         // let patient: Result<Patient, serde_json::Error> = serde_json::from_str(&patient_contents);
//         // match patient {
//         //     Ok(ref p) => println!("{:?}", p),
//         //     Err(ref e) => eprintln!("Error deserializing patient: {}", e),
//         // }
    
//         // let coverage: Result<Coverage, serde_json::Error> = serde_json::from_str(&coverage_contents);
//         // match coverage {
//         //     Ok(ref c) => println!("{:?}", c),
//         //     Err(ref e) => eprintln!("Error deserializing coverage: {}", e),
//         // }


//     let claimInsuranceVec = &claim.unwrap().insurance.ok_or("some error");

//     let claimInsurance = &claimInsuranceVec.as_ref().unwrap()[0];

//     let patientInsurance = &coverage.unwrap().id;

//     let binding = claimInsurance.coverage.reference.as_ref().unwrap();
//     let split = binding.split('/');

//     let vec = split.collect::<Vec<&str>>();

//     let claimInsuranceCoverage = vec.get(1);

//     if &patientInsurance == &claimInsuranceCoverage.unwrap() {
//         CORRECT_COVERAGE = true;
//     }

}
