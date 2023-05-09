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
use threshold_secret_sharing as tss;

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


pub async fn encrypt__and_expose(input: web::Json<String>) -> impl Responder {
    println!("JSON response: {:?}", input);

    let input_json: Value = match serde_json::from_str(&input) {
        Ok(json) => json,
        Err(_) => return HttpResponse::BadRequest().body("Failed to deserialize the input JSON string"),
    };

    let patient_name = input_json["value"][0]["PatientName"].as_str().unwrap_or("").to_owned();
    let simpson_string = "Simpson";

    let n = 3;
    let k = 2;

    let shares: Vec<Vec<u32>> = patient_name
        .as_bytes()
        .iter()
        .zip(simpson_string.as_bytes().iter())
        .map(|(&a, &b)| {
            let diff = i32::from(a) - i32::from(b);
            tss::generate_shares(n, k, diff as u32)
        })
        .collect();

    let reconstructed_diff: Vec<i32> = shares
        .iter()
        .map(|share_set| tss::reconstruct_secret(share_set) as i32)
        .collect();

    let is_equal = reconstructed_diff.iter().all(|&diff| diff == 0);

    if is_equal {
        HttpResponse::Ok().body("Strings are equal")
    } else {
        HttpResponse::Ok().body("Strings are not equal")
    }

}
