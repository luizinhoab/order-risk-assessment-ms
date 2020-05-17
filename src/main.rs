#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use crate::infra::risk_postgres::RiskDieselPg;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::{dotenv, var};
use std::borrow::Borrow;

mod app {
    pub mod domain {
        pub mod models;
        pub mod repository;
    }
    pub mod risk_assessment_service;
}

mod interface {
    pub mod api_server;
    pub mod documents;
    pub mod operation_handlers;
}

mod infra {
    pub mod database;
    pub mod entities;
    pub mod risk_postgres;
}
mod schema;

mod errors;

#[cfg_attr(tarpaulin, skip)]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting order-risk-assessment-ms ...");

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Pool initiliaztion error");

    interface::api_server::init_app(pool).await
}
