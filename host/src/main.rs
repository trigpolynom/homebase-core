use homebase_core::{Claim, Patient, Coverage};
use methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use risc0_zkvm::Prover;

fn main() {

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

    let mut prover = Prover::new(VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID).unwrap();

    prover.add_input_u32_slice(&to_vec(&claim).unwrap()).unwrap();
    prover.add_input_u32_slice(&to_vec(&patient).unwrap()).unwrap();
    prover.add_input_u32_slice(&to_vec(&coverage).unwrap()).unwrap();

    let receipt = prover.run().expect("Code should be provable");


}