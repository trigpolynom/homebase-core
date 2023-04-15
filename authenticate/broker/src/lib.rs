use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[post("/auth")]
async fn authenticate(auth_request: web::Json<AuthRequest>) -> impl Responder {
    // Add your authentication logic here
    if auth_request.username == "test" && auth_request.password == "password" {
        HttpResponse::Ok().body("Authenticated")
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(authenticate))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
