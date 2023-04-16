// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use authenticate_core::Outputs;
use serde::{Deserialize, Serialize};
use authenticate_methods::{SEARCH_JSON_ELF, SEARCH_JSON_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};
use tokio;


#[derive(Serialize, Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
}


#[post("/auth")]
async fn authenticate(auth_request: web::Json<AuthRequest>) -> impl Responder {

    let correct_auth = include_str!("../res/passwords.json");
    
    // Make the prover.
    let mut prover =
        Prover::new(SEARCH_JSON_ELF).expect("Prover should be constructed from valid ELF binary");

    prover.add_input_u32_slice(&to_vec(&correct_auth).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&auth_request.username).expect("should be serializable"));
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // Allow all origins

        App::new()
            .wrap(cors)
            .service(authenticate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App};

    #[actix_rt::test]
    async fn test_authenticate_success() {
        let auth_request = AuthRequest {
            username: "ebgordo2".to_string(),
            password: "P@$$word".to_string(),
        };
    
        let app = App::new().service(authenticate);
        let mut app = test::init_service(app).await;
    
        let req = test::TestRequest::post()
            .uri("/auth")
            .set_json(&auth_request)
            .to_request();
    
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    // Add more test cases here based on your authentication logic
}