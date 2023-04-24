// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use json::parse;
use medical_core::{Outputs, Inputs};
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let proof_input: Inputs = env::read();
    let sha = *Impl::hash_bytes(&proof_input.to_digest());

    // Verify patient_id_from_patient equals patient_id_from_claim
    let patient_id_match = &proof_input.patient_id_from_patient == &proof_input.patient_id_from_claim;

    // Verify eligible_amount * coinsurance_percentage = eligible_amount - coinsurance_amount
    let calculated_coinsurance = &proof_input.eligible_amount * &proof_input.coinsurance_pecentage / 100.0;
    let expected_coinsurance = &proof_input.eligible_amount - &proof_input.coinsurance_amount;
    let epsilon = 0.01;
    let coinsurance_match = (calculated_coinsurance - expected_coinsurance).abs() < epsilon;

    let validated = patient_id_match && coinsurance_match;

    let out = Outputs {
        success: validated,
        hash: sha,
        final_payment: Some(*&proof_input.eligible_amount as i64),
    };
    
    env::commit(&out);
}
