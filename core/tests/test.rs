// use std::io::prelude::*;
// use serde::ser::Error;
// use serde_json::Result;
// use models::model::Claim::Claim;
// use std::io::BufReader;

use std::fs::File;
use std::io::prelude::*;
// use homebase_core::Claim;
use fhir::r4::core::{
    claim::Claim,
    coverage::Coverage,
    patient::Patient,
};

#[test]
fn parse_claim() -> std::io::Result<()> {

    // let mut file = File::open("res/provider_resources/claim.json")?;

    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // let claim: Claim = serde_json::from_str(&contents)?;
    // println!("{:?}", claim);

    // Load the claim, patient, and coverage resources from some data source
    let claim = Claim::default();
    let patient = Patient::default();
    let coverage = Coverage::default();

    // Create a ClaimPolicy instance and auto-adjudicate the claim
    let policy = ClaimPolicy {};
    let eob = policy.auto_adjudicate(&claim, &patient, &coverage);

    // Do something with the EOB, such as write it to a file or send it to a downstream system
    println!("Auto-adjudication complete. EOB: {:?}", eob);

    Ok(())

}