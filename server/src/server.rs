use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use actix_cors::Cors;
use medical::validate_medical_data;
use authenticate::authenticate_institution;

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // Allow all origins

        App::new()
            .wrap(Logger::new("%t %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(cors)
            .service(
                web::resource("/medical")
                    .route(web::post().to(validate_medical_data)),
            )
            .service(
                web::resource("/authenticate")
                    .route(web::post().to(authenticate_institution)),
            )
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
