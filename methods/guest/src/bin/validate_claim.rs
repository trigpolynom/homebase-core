#![no_main]

use json::parse;
use risc0_zkvm::
use homebase_core::{Claim, Patient, Coverage};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let claim: Claim = env::read();
    let patient: Patient = env::read();
    let coverage: Coverage = env::read();
    let data: String = env::read();
    let data2: String = env::read();

    let sha = sha::digest(&data.as_bytes());
    let sha2 = sha::digest(&data2.as_bytes());

    let data = parse(&data).unwrap();
    let data2 = parse(&data2).unwrap();
}
