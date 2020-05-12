use super::operation_handlers as handlers;
use actix_web::{web, App, HttpServer};
use std::env::var;

pub async fn create_server() -> std::io::Result<()> {
    let host = var("SERVER_HOST").unwrap_or_else(|_| String::from("localhost"));
    let port = var("SERVER_PORT").unwrap_or_else(|_| String::from("8000"));

    HttpServer::new(|| {
        App::new()
            .route(
                "/api/doc",
                web::get().to(|r| {
                    handlers::handle_statics_by_path("doc/swaggerui/index.html".to_string(), r)
                }),
            )
            .route(
                "/api/risk-api.yaml",
                web::get()
                    .to(|r| handlers::handle_statics_by_path("doc/risk-api.yaml".to_string(), r)),
            )
            .route(
                "/api/{filename:.*}",
                web::get()
                    .to(|r| handlers::handle_statics_by_path("doc/swaggerui/".to_string(), r)),
            )
    })
    .bind(&format!("{}:{}", host, port))?
    .run()
    .await
}
