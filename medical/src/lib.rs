// medical/src/lib.rs

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};
use tokio;

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
}

pub async fn validate_medical_data(_auth_request: web::Json<AuthRequest>) -> impl Responder {
    println!("Inside validate_medical_data function");
    let correct_auth = include_str!("../res/passwords.json");

    HttpResponse::Ok().json(ApiResponse { success: true })

    // ... rest of the function implementation
}
