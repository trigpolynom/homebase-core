// //medical/src/main.rs

// use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
// use actix_cors::Cors;
// use serde::{Deserialize, Serialize};
// use risc0_zkvm::{
//     serde::{from_slice, to_vec},
//     Prover,
// };
// use tokio;


// #[derive(Serialize, Deserialize)]
// struct AuthRequest {
//     username: String,
//     password: String,
// }

// #[derive(Serialize, Deserialize)]
// struct ApiResponse {
//     success: bool,
// }

// async fn validate_medical_data(auth_request: web::Json<AuthRequest>) -> impl Responder {

//     let correct_auth = include_str!("../res/passwords.json");
    
//     // // Make the prover.
//     // let mut prover =
//     //     Prover::new(SEARCH_JSON_ELF).expect("Prover should be constructed from valid ELF binary");

//     // prover.add_input_u32_slice(&to_vec(&correct_auth).expect("should be serializable"));
//     // prover.add_input_u32_slice(&to_vec(&auth_request.username).expect("should be serializable"));
//     // prover.add_input_u32_slice(&to_vec(&auth_request.password).expect("should be serializable"));
   
//     // let receipt = prover.run().expect(
//     //     "Code should be provable unless it had an error or exceeded the maximum cycle limit",
//     // );

//     // receipt
//     //     .verify(&SEARCH_JSON_ID)
//     //     .expect("Proven code should verify");

//     // let journal = &receipt.journal;

//     // let outputs: Outputs = from_slice(&correct_auth).expect("Journal should contain an Outputs object");

//     // if outputs.success {
//         HttpResponse::Ok().json(ApiResponse { success: true })
//     // } else {
//     //     HttpResponse::Unauthorized().json(ApiResponse { success: false })
//     // }
// }

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         let cors = Cors::permissive(); // Allow all origins

//         App::new()
//             .wrap(cors)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

fn main() {

}
