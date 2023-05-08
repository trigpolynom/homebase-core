use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use actix_cors::Cors;
use fhe::encrypt__and_expose;
pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // Allow all origins

        App::new()
            .wrap(Logger::new("%t %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(cors)
            .service(
                web::resource("/fhe")
                    .route(web::post().to(encrypt__and_expose)),
            )
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
