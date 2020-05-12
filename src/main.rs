#[macro_use]
extern crate log;
use dotenv::dotenv;

mod interface{
    pub mod api_server;
    pub mod operation_handlers;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting order-risk-assessment-ms ...");

    interface::api_server::create_server().await

}