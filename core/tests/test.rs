use serde_json::Result;
use std::{fs::File, io::Read};
use fhir::r4::core::{Claim, Coverage, Patient};
use homebase_core::ClaimPolicy;

#[test]
fn parse_claim() -> Result<()> {

    let mut file = File::open("res/provider_resources/claim.json")?;
    let mut file2 = File::open("res/patient_resources/patient_details.json")?;
    let mut file3 = File::open("res/patient_resources/patient_coverage.json")?;

    let mut claimContents = String::new();
    file.read_to_string(&mut claimContents)?;

    let mut patientContents = String::new();
    file2.read_to_string(&mut patientContents)?;

    let mut coverageContents = String::new();
    file3.read_to_string(&mut coverageContents)?;

    let claim: Claim = serde_json::from_str(&claimContents)?;
    let patient: Patient = serde_json::from_str(&patientContents)?;
    let coverage: Coverage = serde_json::from_str(&coverageContents)?;

    // Validate that the claim has the correct fields
    assert_eq!(claim.type_, Some(fhir::r4::core::CodeableConcept::new_with_code("professional")));


    // println!("{:?}", claim);

    // Load the claim, patient, and coverage resources from some data source
    // let claim = Claim::default();
    // let patient = Patient::default();
    // let coverage = Coverage::default();

    // Create a ClaimPolicy instance and auto-adjudicate the claim
    let policy = ClaimPolicy {};
    let eob = policy.auto_adjudicate(&claim, &patient, &coverage);

    // Do something with the EOB, such as write it to a file or send it to a downstream system
    println!("Auto-adjudication complete. EOB: {:?}", eob);

    Ok(())

}