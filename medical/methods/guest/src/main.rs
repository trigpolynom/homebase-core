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
use authenticate_core::Outputs;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let data: String = env::read();
    let sha = *Impl::hash_bytes(&data.as_bytes());
    let data = parse(&data).unwrap();
    let stored_username = data["username"].as_str().unwrap();
    let stored_password = data["password"].as_str().unwrap();

    let input_username: String = env::read();
    let input_password: String = env::read();

    let authorized = stored_username == input_username && stored_password == input_password;

    let out = Outputs {
        success: authorized,
        hash: sha,
    };
    env::commit(&out);
}
