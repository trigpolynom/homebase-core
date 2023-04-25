// medical/src/lib.rs
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use medical_core::{Outputs, Data, Claims, Inputs, SalesforceRecord};
use medical_methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use serde::{Deserialize, Serialize};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover, sha::{self, Digest}
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
    hash: Digest,
    message: String,
}


async fn fetch_claim() -> Result<Claims, FetchClaimError> {
    let response = reqwest::get("http://localhost:8080/odata/claim").await?;

    let json: Value = response.json().await?;
    println!("Raw JSON: {}", json); // Debug print the raw JSON

    let deserialized_claim: Result<Claims, SerdeJsonError> = serde_json::from_value(json);

    deserialized_claim.map_err(FetchClaimError::from)
}

pub async fn validate_medical_data(input: web::Json<Value>) -> impl Responder {
    println!("Inside validate_medical_data function");
    let input_value = input.into_inner();
    println!("Input JSON: {:?}", input_value); 
    match serde_json::from_value::<Data>(input_value) {
        Ok(mut data) => {
            
            // Work with the deserialized data
            let patient_id = &data.patientDetails.value[0].id;
            let patient_name = &data.patientDetails.value[0].name[0].given[0];
            let salesforce_in_network_coinsurance_percentage = data.salesforceResult.records[0].InNetworkCoinsurancePercentage;

            println!("Patient ID: {}", patient_id);
            println!("Patient Name: {}", patient_name);
            println!("Salesforce In-Network Coinsurance Percentage: {}", salesforce_in_network_coinsurance_percentage);

            // Fetch the claim data
            match fetch_claim().await {
                Ok(claims) => {
                    let claim = &claims.value[0];

                    // Fetch the eligible amount, coinsurance amount, and payment from adjudication
                let (mut eligible_amount, mut coinsurance_amount, mut payment) = (0.0, 0.0, 0.0);
                for adj in claim.item.as_ref().unwrap()[0].adjudication.as_ref().unwrap() {
                    match adj.category.as_ref().unwrap().coding.as_ref().unwrap()[0].code.as_ref().unwrap().as_str() {
                        "eligible" => eligible_amount = adj.amount.as_ref().unwrap().value.unwrap(),
                        "coinsurance" => coinsurance_amount = adj.amount.as_ref().unwrap().value.unwrap(),
                        "payment" => payment = adj.amount.as_ref().unwrap().value.unwrap(),
                        _ => (),
                    }
                }


                let proof_input = Inputs {
                        patient_id_from_patient: patient_id.clone(),
                        patient_id_from_claim: claim.patient.as_ref().unwrap().reference.as_ref().unwrap()[8..].to_string(),
                        eligible_amount: eligible_amount.round() as i64,
                        coinsurance_amount: coinsurance_amount.round() as i64,
                        coinsurance_pecentage: salesforce_in_network_coinsurance_percentage.round() as i64,
                        payment: payment.round() as i64,

                    };

                let serialized_inputs = to_vec(&proof_input).expect("should be serializable");

                // println!("Serialize patient_id_from_patient: {:?}", to_vec(&proof_input.patient_id_from_patient));
                // println!("Serialize patient_id_from_claim: {:?}", to_vec(&proof_input.patient_id_from_claim));
                // println!("Serialize eligible_amount: {:?}", to_vec(&proof_input.eligible_amount));
                // println!("Serialize coinsurance_amount: {:?}", to_vec(&proof_input.coinsurance_amount));
                // println!("Serialize coinsurance_pecentage: {:?}", to_vec(&proof_input.coinsurance_pecentage));
                // println!("Serialize payment: {:?}", to_vec(&proof_input.payment));


                    // Make the prover.
                let mut prover =
                Prover::new(VALIDATE_CLAIM_ELF).expect("Prover should be constructed from valid ELF binary");
                prover.add_input_u32_slice(&serialized_inputs);

                // prover.add_input_u32_slice(&to_vec(&provider_password).expect("should be serializable"));
                // prover.add_input_u32_slice(&to_vec(&auth_request.username).expect("should be serializable"));
                // prover.add_input_u32_slice(&to_vec(&auth_request.password).expect("should be serializable"));

                let receipt = prover.run().expect(
                    "Code should be provable unless it had an error or exceeded the maximum cycle limit",
                );

                receipt
                    .verify(&VALIDATE_CLAIM_ID)
                    .expect("Proven code should verify");

                let journal = &receipt.journal;

                let outputs: Outputs = from_slice(&journal).expect("Journal should contain an Outputs object");

                let success_message = format!("Healthcare Claim successfully validated. Attached is the hash of the data. Payment is {:?}", outputs.final_payment);
                let fail_message = format!("Healthcare claim unsuccesfully validated. Will resubmit.");
                if outputs.success {
                    return HttpResponse::Ok().json(ApiResponse { 
                        success: true,
                        hash: outputs.hash,
                        message: success_message,
                    });
                } else {
                    return HttpResponse::Unauthorized().json(ApiResponse {
                        success: false,
                        hash: outputs.hash,
                        message: fail_message,
                    });
                }
                
                
                }
                Err(e) => {
                    eprintln!("Error fetching claim data: {}", e);
                    return HttpResponse::InternalServerError().body("Failed to fetch claim data");
                }
            }

            // ...

            HttpResponse::Ok().body("Successfully deserialized data")
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to deserialize data: {}", e)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let proof_input = Inputs {
            patient_id_from_patient: "P001".to_string(),
            patient_id_from_claim: "P001".to_string(),
            eligible_amount: 8500,
            coinsurance_amount: 1700,
            coinsurance_pecentage: 20,
            payment: 6800,
        };

        let patient_id_match = &proof_input.patient_id_from_patient == &proof_input.patient_id_from_claim;
        println!("patient_id_match: {}", patient_id_match);

        let calculated_coinsurance = (proof_input.eligible_amount as f64) * (proof_input.coinsurance_pecentage as f64) / 100.0;        
        let expected_payment = (proof_input.eligible_amount as f64) * (1.0 - (proof_input.coinsurance_pecentage as f64) / 100.0);

        let epsilon = 0.01;
        let coinsurance_match = (calculated_coinsurance - (proof_input.coinsurance_amount as f64)).abs() < epsilon;       
        println!("coinsurance_match: {}", coinsurance_match);
        
        let payment_match = (expected_payment - (proof_input.payment as f64)).abs() < epsilon;
        println!("payment_match: {}", payment_match);

        let validated = patient_id_match && coinsurance_match && payment_match;

        assert_eq!(validated, true);
    }
}

