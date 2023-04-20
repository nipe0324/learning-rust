use dotenvy::dotenv;
use std::env;

pub const AXUM_SESSION_COOKIE_NAME: &str = "rustwi_session";
pub const AXUM_SESSION_USER_ID_KEY: &str = "uid";

pub fn database_url() -> String {
    dotenv().expect(".env file not found");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
