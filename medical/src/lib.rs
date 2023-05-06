use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use medical_core::{Outputs, Data, Claims, Inputs, SalesforceRecord};
use medical_methods::{VALIDATE_CLAIM_ELF, VALIDATE_CLAIM_ID};
use serde::{Deserialize, Serialize};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover, sha::{self, Digest}
};
use serde_json::{Value, from_str};
use tokio;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::error::Error;
use std::fmt;
use cupcake::traits::*;

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
    journal: Vec<u8>,
}

struct FetchedClaim {
    claim_coinsurance: i64,
    claim_patient_id: String,
    claim_payment: i64,
    eligible_amount: i64,
}


async fn fetch_claim() -> Result<FetchedClaim,  Box<dyn Error>> {
    let url = "http://localhost:8080/odata/homebase.svc/provider/Claims?$top=11&$filter=(Id eq '01')";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        // response.json().await?;
        // let body = response.text().await?;

        // let json_data: Value = serde_json::from_str(&body)?;

        let json_data: Value = response.json().await?;

        // Access fields from the deserialized JSON data
        println!("Input JSON: {:?}", json_data.to_string());
        let claim_coinsurance = json_data["value"][0]["ClaimCoinsurance"].as_f64().unwrap() as i64;
       
        let claim_patient_id = json_data["value"][0]["ClaimPatientId"].as_str().unwrap().to_string();
        let claim_payment = json_data["value"][0]["ClaimPayment"].as_f64().unwrap() as i64;
        let eligible_amount = json_data["value"][0]["EligibleAmount"].as_f64().unwrap() as i64;

        Ok(FetchedClaim {
            claim_coinsurance,
            claim_patient_id,
            claim_payment,
            eligible_amount,
        })
    } else {
        let error_msg = format!("Unexpected status code: {}", response.status());
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)))
    }
}


pub async fn validate_medical_data(input: web::Json<Value>) -> impl Responder {

    println!("host - init fv context");
    let fv = cupcake::default();


    println!("host - init fv keys");
    let (pk, sk) = fv.generate_keypair();

    println!("Inside validate_medical_data function");
    let input_value = input.into_inner();
    println!("Input JSON: {:?}", input_value); 

    let encrypted_input = fv.encrypt(&input_value, &pk);

    // Access fields from the deserialized JSON data
    let patient_id = input_value["patientDetails"]["value"][0]["Id"].as_str().unwrap();
    let patient_name = input_value["patientDetails"]["value"][0]["PatientName"].as_str().unwrap();
    let salesforce_in_network_coinsurance_percentage = input_value["salesforceResult"]["records"][0]["InNetworkCoinsurancePercentage"].as_f64().unwrap();

    println!("Patient ID: {}", patient_id);
    println!("Patient Name: {}", patient_name);
    println!("Salesforce In-Network Coinsurance Percentage: {}", salesforce_in_network_coinsurance_percentage);

            // Fetch the claim data
            match fetch_claim().await {
                Ok(claim) => {
                  

                let proof_input = Inputs {
                        patient_id_from_patient: patient_id.to_owned(),
                        patient_id_from_claim: claim.claim_patient_id,
                        eligible_amount: claim.eligible_amount,
                        coinsurance_amount: claim.claim_coinsurance,
                        coinsurance_pecentage: salesforce_in_network_coinsurance_percentage.round() as i64,
                        payment: claim.claim_payment,

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

                let success_message = format!("Healthcare Claim successfully validated. Attached is the hash of the data. Payment is {:?}", outputs.final_payment.to_owned());
                let fail_message = format!("Healthcare claim unsuccesfully validated. Will resubmit.");
                if outputs.success {
                    return HttpResponse::Ok().json(ApiResponse { 
                        success: true,
                        hash: outputs.hash,
                        message: success_message,
                        journal: journal.to_owned(),
                    });
                } else {
                    return HttpResponse::Unauthorized().json(ApiResponse {
                        success: false,
                        hash: outputs.hash,
                        message: fail_message,
                        journal: journal.to_owned(),
                    });
                }
                
                
                }
                Err(e) => {
                    eprintln!("Error fetching claim data: {}", e);
                    return HttpResponse::InternalServerError().body("Failed to fetch claim data");
                }
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

