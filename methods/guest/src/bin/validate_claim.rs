#![no_main]

use risc0_zkvm::guest::{env, sha};
use homebase_core::{Claim, Patient, Coverage};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let claim: Claim = env::read();
    let patient: Patient = env::read();
    let coverage: Coverage = env::read();

}
