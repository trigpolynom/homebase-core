use std::fs::File;

use fhir_rs::{fhir_parse, model};

#[test]
fn parse_claim() {
    let mut file = std::fs:File::open("res/provider_resources/claim.json").expect("Example file should be accessible");

    let mut data = String::new();

    file.read_to_string(&mut data).expect("Should not have I/O errors");


}