use risc0_zkvm::sha::{Impl, Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bincode::{serialize, Error};
use std::result::Result;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub success: bool,
    pub hash: Digest,
    pub final_payment: Option<i64>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Inputs {
    pub patient_id_from_patient: String,
    pub patient_id_from_claim: String,
    pub eligible_amount: i64,
    pub coinsurance_amount: i64,
    pub coinsurance_pecentage: i64,
    pub payment: i64,
}

impl Inputs {
    pub fn to_digest(&self) -> Result<Digest, Error> {
        let bytes = serialize(self)?;
        let digest = *Impl::hash_bytes(&bytes);
        Ok(digest)
    }
}

// The rest of the code remains the same


#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub patientDetails: PatientDetails,
    pub salesforceResult: SalesforceResult,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PatientDetails {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    pub value: Vec<Patient>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Patient {
    pub resourceType: String,
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub name: Vec<Name>,
    // Add other fields as needed
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
pub struct Claims {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    pub value: Vec<Claim>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientReference {
    pub reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub resourceType: Option<String>,
    pub id: Option<String>,
    pub status: Option<String>,
    pub r#use: Option<String>,
    pub patient: Option<PatientReference>,
    pub provider: Option<Provider>,
    pub insurer: Option<Insurer>,
    pub created: Option<String>,
    pub diagnosis: Option<Vec<Diagnosis>>,
    pub item: Option<Vec<Item>>,
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
    pub sequence: Option<u32>,
    pub productOrService: Option<ProductOrService>,
    pub servicedDate: Option<String>,
    pub quantity: Option<Quantity>,
    pub unitPrice: Option<UnitPrice>,
    pub adjudication: Option<Vec<Adjudication>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOrService {
    pub coding: Option<Vec<Coding>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitPrice {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Adjudication {
    pub category: Option<Category>,
    pub amount: Option<Amount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub coding: Option<Vec<Coding>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}
