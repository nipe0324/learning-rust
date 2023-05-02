use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

use axum_sqlx::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read .env file
    dotenvy::dotenv().ok();

    // init the logger
    env_logger::init();

    // parse our configuration from the environment;
    let config = envy::from_env::<Config>().context("Failed to parse environment")?;

    println!("config: {:?}", config);
    Ok(())
}
