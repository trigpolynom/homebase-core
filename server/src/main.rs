// server/src/main.rs
mod server;

#[tokio::main]
async fn main() {
    server::run_server().await;
}
