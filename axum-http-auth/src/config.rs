use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default = "default_environment")]
    pub environment: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_postgres_host")]
    pub postgres_host: String,
    #[serde(default = "default_postgres_port")]
    pub postgres_port: String,
    #[serde(default = "default_postgres_db")]
    pub postgres_db: String,
    #[serde(default = "default_postgres_user")]
    pub postgres_user: String,
    #[serde(default = "default_postgres_password")]
    pub postgres_password: String,
    #[serde(default = "dafault_redis_host")]
    pub redis_host: String,
    #[serde(default = "dafault_redis_port")]
    pub redis_port: String,
}

fn default_environment() -> String {
    "development".to_string()
}

fn default_log_level() -> String {
    "debug".to_string()
}

fn default_postgres_host() -> String {
    "localhost".to_string()
}

fn default_postgres_port() -> String {
    "5432".to_string()
}

fn default_postgres_db() -> String {
    "axum-http-auth".to_string()
}

fn default_postgres_user() -> String {
    "postgres".to_string()
}

fn default_postgres_password() -> String {
    "password123".to_string()
}

fn dafault_redis_host() -> String {
    "localhost".to_string()
}

fn dafault_redis_port() -> String {
    "6379".to_string()
}
