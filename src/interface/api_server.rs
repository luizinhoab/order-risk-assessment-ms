use super::operation_handlers as handlers;
use crate::interface::operation_handlers::Pool;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::env::var;

pub async fn init_app(pool: Pool) -> std::io::Result<()> {
    let host = var("SERVER_HOST").unwrap_or_else(|_| String::from("localhost"));
    let port = var("SERVER_PORT").unwrap_or_else(|_| String::from("8000"));

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(web::JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .configure(route)
    })
    .bind(&format!("{}:{}", host, port))?
    .run()
    .await
}

fn route(app: &mut web::ServiceConfig) {
    app.route(
        "/risk/doc",
        web::get()
            .to(|r| handlers::handle_statics_by_path("doc/swaggerui/index.html".to_string(), r)),
    )
    .route(
        "/risk/risk-api.yaml",
        web::get().to(|r| handlers::handle_statics_by_path("doc/risk-api.yaml".to_string(), r)),
    )
    .route(
        "/risk/{filename:.*}",
        web::get().to(|r| handlers::handle_statics_by_path("doc/swaggerui/".to_string(), r)),
    )
    .service(handlers::handle_assessment_risk);
}
