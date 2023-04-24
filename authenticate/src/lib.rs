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
    Prover,
};
use tokio;


#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    password: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
}

async fn fetch_password() -> Result<String, Error> {
    let response = reqwest::get("http://localhost:8080/odata/password").await?;

    let json: Value = response.json().await?;
    let password = json["value"][0]["P001"].as_str().unwrap_or("");

    Ok(password.to_string())
}


pub async fn authenticate_institution(input: web::Json<Value>) -> impl Responder {
    println!("Inside authenticate_institution function");

    let provider_password = match fetch_password().await {
        Ok(password) => password,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse { success: false });
        }
    };
    let patient_password = input["value"][0]["password"].as_str().unwrap_or("");

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

    let outputs: Outputs = from_slice(&journal).expect("Journal should contain an Outputs object");

    if outputs.success {
        HttpResponse::Ok().json(ApiResponse { success: true })
    } else {
        HttpResponse::Unauthorized().json(ApiResponse { success: false })
    }
}
