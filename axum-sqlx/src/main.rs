use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

use axum_sqlx::config::Config;
use axum_sqlx::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read .env file
    dotenvy::dotenv().ok();

    // init the logger
    env_logger::init();

    // parse our configuration from the environment;
    let config = envy::from_env::<Config>().context("Failed to parse environment")?;

    // create a single connection pool
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("Failed to create connection pool")?;

    // run the migrations
    sqlx::migrate!().run(&db).await?;

    // run http server
    http::serve(config, db).await?;

    Ok(())
}
