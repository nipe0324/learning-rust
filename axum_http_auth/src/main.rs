use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum_http_auth::config::AppConfig;

fn main() {
    let config = envy::from_env::<AppConfig>().unwrap();

    init_tracing(&config.log_level);
    debug_config(&config);
}

fn init_tracing(log_level: &String) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn debug_config(config: &AppConfig) {
    tracing::debug!("ENVIRONMENT: {:#?}", config.environment);
    tracing::debug!("LOG_LEVEL: {:#?}", config.log_level);
    tracing::debug!("POSTGRES_HOST: {:#?}", config.postgres_host);
    tracing::debug!("POSTGRES_PORT: {:#?}", config.postgres_port);
    tracing::debug!("POSTGRES_DB: {:#?}", config.postgres_db);
    tracing::debug!("POSTGRES_USER: {:#?}", config.postgres_user);
    tracing::debug!("REDIS_HOST: {:#?}", config.redis_host);
    tracing::debug!("REDIS_PORT: {:#?}", config.redis_port);
}
