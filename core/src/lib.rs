use risc0_zkp::core::sha::Digest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub hash: Digest,
    pub hash2: Digest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claim {
    pub resourceType: String,
    pub id: String,
    pub text: Text,
    pub identifier: Vec<Identifier>,
    pub status: String,
    pub type_: Type,
    pub use_: String,
    pub patient: Patient,
    pub billablePeriod: Option<BillablePeriod>,
    pub created: String,
    pub enterer: Enterer,
    pub insurer: Insurer,
    pub provider: Provider,
    pub priority: Priority,
    pub payee: Payee,
    pub facility: Option<Facility>,
    pub careTeam: Vec<CareTeam>,
    pub supportingInfo: Vec<SupportingInfo>,
    pub diagnosis: Vec<Diagnosis>,
    pub insurance: Vec<Insurance>,
    pub accident: Option<Accident>,
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub status: String,
    pub div: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
    pub system: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
    pub coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coding {
    pub system: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Patient {
    pub reference: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BillablePeriod {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Enterer {
    pub identifier: Identifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Insurer {
    pub reference: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    pub identifier: Identifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Priority {
    pub coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payee {
    pub type_: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Facility {
    pub identifier: Identifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CareTeam {
    pub sequence: i32,
    pub provider: Provider,
    pub responsible: bool,
    pub role: Role,
    pub qualification: Qualification,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    pub coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Qualification {
    pub coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportingInfo {
    pub sequence: i32,
    pub category: Category,
    pub timingPeriod: TimingPeriod,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimingPeriod {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diagnosis {
    pub sequence: i32,
    pub diagnosisCodeableConcept: DiagnosisCodeableConcept,
    pub type_: Vec<Type>,
    pub packageCode: PackageCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosisCodeableConcept {
    pub coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageCode {
    pub coding: Vec<Coding>,
}


#[derive(Serialize, Deserialize)]
pub struct Text {
    status: String,
    div: String,
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    system: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coding {
    system: String,
    code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Type {
    coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize)]
pub struct Reference {
    reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct Period {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize)]
pub struct Enterer {
    identifier: Identifier,
}

#[derive(Serialize, Deserialize)]
pub struct Provider {
    identifier: Identifier,
}

#[derive(Serialize, Deserialize)]
pub struct PayeeType {
    coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize)]
pub struct Facility {
    identifier: Identifier,
}

#[derive(Serialize, Deserialize)]
pub struct Role {
    coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize)]
pub struct Qualification {
    coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize)]
pub struct CareTeam {
    sequence: i32,
    provider: Reference,
    responsible: bool,
    role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualification: Option<Qualification>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    coding: Vec<Coding>,
}

#[derive(Serialize, Deserialize)]
pub struct TimingPeriod {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize)]
pub struct SupportingInfo {
    sequence: i32,
    category: Category,
    timingPeriod: TimingPeriod,
}

#[derive(Serialize, Deserialize)]
pub struct DiagnosisCodeableConceptCoding {
    code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DiagnosisCodeableConcept {
    coding: Vec<DiagnosisCodeableConceptCoding>,
}

#[derive(Serialize, Deserialize)]
pub struct DiagnosisTypeCoding {
    system: String,
    code: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiagnosisType {
    coding: Vec<DiagnosisTypeCoding>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageCodeCoding {
    system: String,
    code: String,
    display: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackageCode {
    coding: Vec<PackageCodeCoding>,
}
