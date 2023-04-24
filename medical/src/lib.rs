// medical/src/lib.rs
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use medical_core::{Outputs, Data, ClaimWrapper};
use medical_methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use serde::{Deserialize, Serialize};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};
use serde_json::Value;
use tokio;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::fmt;

#[derive(Debug)]
enum FetchClaimError {
    ReqwestError(ReqwestError),
    SerdeJsonError(SerdeJsonError),
}

impl fmt::Display for FetchClaimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchClaimError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            FetchClaimError::SerdeJsonError(e) => write!(f, "Serde JSON error: {}", e),
        }
    }
}

impl std::error::Error for FetchClaimError {}

impl From<ReqwestError> for FetchClaimError {
    fn from(error: ReqwestError) -> Self {
        FetchClaimError::ReqwestError(error)
    }
}

impl From<SerdeJsonError> for FetchClaimError {
    fn from(error: SerdeJsonError) -> Self {
        FetchClaimError::SerdeJsonError(error)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
}


async fn fetch_claim() -> Result<ClaimWrapper, FetchClaimError> {
    let response = reqwest::get("http://localhost:8080/odata/claim").await?;

    let json: Value = response.json().await?;
    println!("Raw JSON: {}", json); // Debug print the raw JSON

    let deserialized_claim: Result<ClaimWrapper, SerdeJsonError> = serde_json::from_value(json);

    deserialized_claim.map_err(FetchClaimError::from)
}

pub async fn validate_medical_data(input: web::Json<Value>) -> impl Responder {
    println!("Inside validate_medical_data function");
    match serde_json::from_value::<Data>(input.into_inner()) {
        Ok(data) => {
            // Work with the deserialized data
            let patient_id = &data.patientDetails.value[0].id;
            let patient_name = &data.patientDetails.value[0].name[0].given[0];
            let salesforce_in_network_coinsurance_percentage = data.salesforceResult.records[0].InNetworkCoinsurancePercentage;

            println!("Patient ID: {}", patient_id);
            println!("Patient Name: {}", patient_name);
            println!("Salesforce In-Network Coinsurance Percentage: {}", salesforce_in_network_coinsurance_percentage);

            // Fetch the claim data
            match fetch_claim().await {
                Ok(claim_wrapper) => {
                    let claim = &claim_wrapper.claim;

                    // Use the fetched claim data in the Prover

                    // ...
                }
                Err(e) => {
                    eprintln!("Error fetching claim data: {}", e);
                    return HttpResponse::InternalServerError().body("Failed to fetch claim data");
                }
            }

            let mut prover =
                Prover::new(VALIDATE_CLAIM_ELF).expect("Prover should be constructed from valid ELF binary");

            // ...

            HttpResponse::Ok().body("Successfully deserialized data")
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to deserialize data: {}", e)),
    }
}
