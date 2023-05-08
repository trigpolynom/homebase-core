// authenticate/src/lib.rs
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use concrete::{LWESecretKey, LWE128_630, Encoder, LWE};
use serde_json::Value;
use reqwest::Error;
// use authenticate_core::Outputs;
use serde::{Deserialize, Serialize};
// use authenticate_methods::{SEARCH_JSON_ELF, SEARCH_JSON_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover, Receipt,
};
use tokio;
use std::{hash::{Hash, Hasher}, collections::HashMap};
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



pub async fn encrypt__and_expose(input: web::Json<Value>) -> impl Responder {
    println!("JSON response: {:?}", input);

    // Extract the String value from the input JSON
    let input_string = match input.0.as_str() {
        Some(value) => value,
        None => return HttpResponse::BadRequest().body("Invalid input: expected a string"),
    };

    let secret_key = LWESecretKey::new(&LWE128_630);
    let encoder = Encoder::new(0., 1024., 8, 24).unwrap();

    // Convert the input string to a Vec<f64> using UTF-8 encoding
    let input_f64_vec: Vec<f64> = input_string.as_bytes().iter().map(|&byte| byte as f64).collect();

    // Encrypt each f64 value in the input_f64_vec
    let encrypted_vec: Vec<_> = input_f64_vec
        .into_iter()
        .map(|input_f64| LWE::encode_encrypt(&secret_key, input_f64, &encoder).unwrap())
        .collect();

    // Serialize the encrypted data to a JSON string
    let serialized = serde_json::to_string(&encrypted_vec).unwrap();

    HttpResponse::Ok().json(serialized)

}
