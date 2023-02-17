use risc0_zkp::core::sha::Digest;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use chrono::{DateTime, Utc};


#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub hash: Digest,
    pub hash2: Digest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub gender: Option<AdministrativeGender>,
    pub birth_date: Option<Date>,
    pub deceased: Option<Deceased>,
    pub address: Vec<Address>,
    pub marital_status: Option<CodeableConcept>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub status: ClaimStatus,
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    pub sub_type: Option<Vec<CodeableConcept>>,
    #[serde(rename = "use")]
    pub use_: Use,
    pub patient: Reference,
    pub created: String,
    pub insurer: Reference,
    pub provider: Reference,
    pub diagnosis: Option<Vec<Diagnosis>>,
    pub item: Option<Vec<Item>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coverage {
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub status: FinancialResourceStatusCodes,
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    pub policy_holder: Reference,
    pub subscriber: Reference,
    pub subscriber_id: Option<Identifier>,
    pub beneficiary: Reference,
    pub relationship: Option<CodeableConcept>,
    pub period: Option<Period>,
    pub payor: Vec<Reference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Insurance {
    pub id: String,
    pub coverage: Reference,
    pub pre_auth_ref: Option<String>,
    pub coverage_period: Option<Period>,
    pub authorization: Option<Vec<InsuranceAuthorization>>,
    pub benefit: Option<Vec<InsuranceBenefit>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub active: Option<bool>,
    pub name: HumanName,
    pub telecom: Vec<ContactPoint>,
    pub address: Vec<Address>,
    pub gender: Option<AdministrativeGender>,
    pub specialty: Option<Vec<CodeableConcept>>,
    pub qualification: Option<Vec<Qualification>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(rename = "use")]
    pub use_: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    pub system: Option<String>,
    pub value: Option<String>,
    pub period: Option<Period>,
    pub assigner: Option<Box<Reference>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumanName {
    #[serde(rename = "use")]
    pub use_: Option<String>,
    pub text: Option<String>,
    pub family: Option<Vec<String>>,
    pub given: Option<Vec<String>>,
    pub prefix: Option<Vec<String>>,
    pub suffix: Option<Vec<String>>,
    pub period: Option<Period>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPoint {
    pub system: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "use")]
    pub use_: Option<String>,
    pub rank: Option<u32>,
    pub period: Option<Period>,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize)]
pub enum AdministrativeGender {
    Male,
    Female,
    Other,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deceased {
    pub deceased_boolean: Option<bool>,
    pub deceased_date: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "use")]
    pub use_: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub text: Option<String>,
    pub line: Option<Vec<String>>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub period: Option<Period>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeableConcept {
    pub coding: Option<Vec<Coding>>,
    pub text: Option<String>,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize)]
pub enum ClaimStatus {
    Active,
    Cancelled,
    Draft,
    EnteredInError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub reference: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub identifier: Option<Identifier>,
    pub display: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diagnosis {
    pub condition: Option<Reference>,
    #[serde(rename = "use")]
    pub use_: Option<CodeableConcept>,
    pub rank: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub sequence: u32,
    pub care_team_sequence: Option<Vec<u32>>,
    pub diagnosis_sequence: Option<Vec<u32>>,
    pub procedure_sequence: Option<Vec<u32>>,
    pub information_sequence: Option<Vec<u32>>,
    pub revenue: Option<CodeableConcept>,
    pub category: Option<CodeableConcept>,
    pub product_or_service: Option<CodeableConcept>,
    pub modifier: Option<Vec<CodeableConcept>>,
    pub program_code: Option<Vec<CodeableConcept>>,
    pub serviced_date: Option<Date>,
    pub serviced_period: Option<Period>,
    pub location: Option<Reference>,
    pub quantity: Option<Quantity>,
    pub unit_price: Option<Money>,
    pub factor: Option<f64>,
    pub net: Option<Money>,
    pub body_site: Option<CodeableConcept>,
    pub sub_site: Option<Vec<CodeableConcept>>,
    pub note_number: Option<Vec<u32>>,
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,
    pub detail: Option<Vec<ItemDetail>>,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize)]
pub enum FinancialResourceStatusCodes {
    Draft,
    Active,
    Cancelled,
    EnteredInError,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    pub start: Option<Date>,
    pub end: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceAuthorization {
    pub reference: Option<Vec<Reference>>,
    pub pre_auth_ref: Option<String>,
    pub coverage: Option<Reference>,
    pub benefit: Option<Vec<InsuranceBenefit>>,
    pub pre_auth_period: Option<Period>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceBenefit {
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    pub allowed: Option<Money>,
    pub used: Option<Money>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    pub value: f64,
    pub currency: String,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize)]
pub enum Use {
    Claim,
    Usual,
    Official,
    Temp,
    Secondary,
    Old,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Qualification {
    pub identifier: Option<Vec<Identifier>>,
    pub code: CodeableConcept,
    pub period: Option<Period>,
    pub issuer: Option<Reference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coding {
    pub system: Option<String>,
    pub version: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
    pub user_selected: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub comparator: Option<String>,
    pub unit: Option<String>,
    pub system: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimResponseItemAdjudication {
    pub category: CodeableConcept,
    pub reason: Option<CodeableConcept>,
    pub amount: Option<Money>,
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDetail {
    pub sequence: u32,
    pub revenue: Option<CodeableConcept>,
    pub category: Option<CodeableConcept>,
    pub product_or_service: Option<CodeableConcept>,
    pub modifier: Option<Vec<CodeableConcept>>,
    pub program_code: Option<Vec<CodeableConcept>>,
    pub quantity: Option<Quantity>,
    pub unit_price: Option<Money>,
    pub factor: Option<f64>,
    pub net: Option<Money>,
    pub note_number: Option<Vec<u32>>,
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,
}






