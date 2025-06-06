use actix_web::{web, App, HttpServer};
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/test-api", web::post().to(handlers::test_api))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
