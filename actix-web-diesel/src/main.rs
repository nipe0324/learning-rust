#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::{middleware::Logger, App, HttpServer};

// mod app;
mod constants;
mod error;
mod middleware;
// mod routes;
// mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start server ...");
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        middleware::state::AppState { pool }
    };

    Ok(())
}
