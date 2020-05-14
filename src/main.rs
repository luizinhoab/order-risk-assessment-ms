#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;

mod app {
    pub mod domain {
        pub mod models;
        pub mod repository;
    }
}

mod interface {
    pub mod api_server;
    pub mod operation_handlers;
}

mod infra {
    pub mod assesment_mapper;
    pub mod database;
    pub mod risk_postgres;
}
mod schema;

#[cfg_attr(tarpaulin, skip)]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting order-risk-assessment-ms ...");

    interface::api_server::create_server().await
}
