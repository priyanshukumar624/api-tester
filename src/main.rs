use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::handlers::handlers::test_api;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://0.0.0.0:8080");

    HttpServer::new(|| {
        // ✅ CORS configuration
        let cors = Cors::default()
            .allow_any_origin() // ⚠️ For development only
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors) // ✅ Apply CORS middleware
            .route("/test-api", web::post().to(test_api))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
