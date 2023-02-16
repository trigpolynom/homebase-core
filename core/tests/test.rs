// use std::io::prelude::*;
// use serde::ser::Error;
// use serde_json::Result;
// use models::model::Claim::Claim;
// use std::io::BufReader;

use std::fs::File;
use std::io::prelude::*;
use homebase_core::Claim;

#[test]
fn parse_claim() -> std::io::Result<()> {

    let mut file = File::open("res/provider_resources/claim.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let claim: Claim = serde_json::from_str(&contents)?;
    println!("{:?}", claim);

    Ok(())

}