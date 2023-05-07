// authenticate/src/lib.rs
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde_json::Value;
use reqwest::Error;
use authenticate_core::Outputs;
use serde::{Deserialize, Serialize};
use authenticate_methods::{SEARCH_JSON_ELF, SEARCH_JSON_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover, Receipt,
};
use tokio;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    password: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    hash_of_seal: Option<u64>,
}

async fn fetch_password() -> Result<String, Error> {
    let response = reqwest::get("https://ec2-3-142-221-10.us-east-2.compute.amazonaws.com/odata/homebase.svc/provider/Passwords?$top=11&$filter=(Id eq '01')").await?;

    let json: Value = response.json().await?;
    println!("JSON response: {:?}", json);
    let password = json["value"][0]["Password"].as_str().unwrap_or("").to_owned();

    Ok(password)
}


pub async fn authenticate_institution(input: web::Json<Value>) -> impl Responder {
    println!("Inside authenticate_institution function");

    let fail_to_parse_message = format!("Parsing the password failed.");


    let provider_password = match fetch_password().await {
        Ok(password) => password,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse { success: false, message: fail_to_parse_message, hash_of_seal: None});
        }
    };
    println!("JSON response: {:?}", input);
    let patient_password = input["value"][0]["TrustedInstitutionPassword"].as_str().unwrap_or("").to_owned();

    let auth_request = AuthRequest {
        password: patient_password.to_string(),
    };
    // Make the prover.
    let mut prover =
        Prover::new(SEARCH_JSON_ELF).expect("Prover should be constructed from valid ELF binary");

    prover.add_input_u32_slice(&to_vec(&provider_password).expect("should be serializable"));
    // prover.add_input_u32_slice(&to_vec(&auth_request.username).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&auth_request.password).expect("should be serializable"));
   
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    receipt
        .verify(&SEARCH_JSON_ID)
        .expect("Proven code should verify");

    let journal = &receipt.journal;

    let mut hasher = DefaultHasher::new();
    receipt.seal.hash(&mut hasher);
    let hash = hasher.finish();

    let outputs: Outputs = from_slice(&journal).expect("Journal should contain an Outputs object");
    let success_message = format!("Passwords are the same, you are a trusted institution of the patient.");
    let fail_message = format!("Passwords are not the same.");

    if outputs.success {
        HttpResponse::Ok().json(ApiResponse { success: true, message: success_message, hash_of_seal: Some(hash)})
    } else {
        HttpResponse::Unauthorized().json(ApiResponse { success: false, message: fail_message, hash_of_seal: Some(hash)})
    }
}
