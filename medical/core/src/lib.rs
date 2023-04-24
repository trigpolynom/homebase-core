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

use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub success: bool,
    pub hash: Digest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
    pub system: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub r#use: String,
    pub family: String,
    pub given: Vec<String>,
}

// Define other structs for other fields as needed

#[derive(Serialize, Deserialize, Debug)]
pub struct Patient {
    pub resourceType: String,
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub name: Vec<Name>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatientDetails {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    pub value: Vec<Patient>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SalesforceRecord {
    pub attributes: serde_json::Value,
    pub InNetworkCoinsurancePercentage: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SalesforceResult {
    pub totalSize: u32,
    pub done: bool,
    pub records: Vec<SalesforceRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub patientDetails: PatientDetails,
    pub salesforceResult: SalesforceResult,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimWrapper {
    pub claim: Option<Claim>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    resourceType: Option<String>,
    id: Option<String>,
    status: Option<String>,
    r#use: Option<String>,
    patient: Option<Patient>,
    provider: Option<Provider>,
    insurer: Option<Insurer>,
    created: Option<String>,
    diagnosis: Option<Vec<Diagnosis>>,
    item: Option<Vec<Item>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Insurer {
    reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diagnosis {
    sequence: Option<u32>,
    diagnosisCodeableConcept: Option<DiagnosisCodeableConcept>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosisCodeableConcept {
    coding: Option<Vec<Coding>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    sequence: Option<u32>,
    productOrService: Option<ProductOrService>,
    servicedDate: Option<String>,
    quantity: Option<Quantity>,
    unitPrice: Option<UnitPrice>,
    adjudication: Option<Vec<Adjudication>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOrService {
    coding: Option<Vec<Coding>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    value: Option<f64>,
    currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Adjudication {
    category: Option<Category>,
    amount: Option<Amount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    coding: Option<Vec<Coding>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    value: Option<f64>,
    currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coding {
    system: Option<String>,
    code: Option<String>,
    display: Option<String>,
}