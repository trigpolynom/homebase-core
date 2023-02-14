use std::io::prelude::*;
use serde_json::Result;
use models::model::Claim::Claim;


#[test]
fn parse_claim() -> Result<()> {
    let mut file = std::fs::File::open("res/provider_resources/claim.json").expect("Example file should be accessible");

    let mut data = String::new();

    file.read_to_string(&mut data).expect("Should not have I/O errors");

    // let data = parse(&data).unwrap();

    let claim: Claim = serde_json::from_str(&data)?;

    // let resource_type = data["resourceType"].as_str().unwrap();

    // println!("{}", resource_type);

    Ok(())
}