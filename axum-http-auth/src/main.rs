use argon2::{self, Config};
use async_redis_session::RedisSessionStore;
use axum::{
    async_trait,
    error_handling::HandleErrorLayer,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, get_service},
    Router,
};
use axum_sessions::{
    extractors::{ReadableSession, WritableSession},
    SessionLayer,
};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use rand::Rng;
use serde::Deserialize;
use std::{net::SocketAddr, time::Duration};
use tokio::signal;
use tokio_postgres::NoTls;
use tower::{BoxError, ServiceBuilder};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use validator::Validate;

use axum_http_auth::config::AppConfig;
use axum_http_auth::views::*;

// Types /////////////////////////////

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

// Structs //////////////////////////

struct DatabaseConnection(PooledConnection<'static, PostgresConnectionManager<NoTls>>);

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSignupInput {
    // #[validate(email)]
    pub email: String,
    // #[validate(must_match = "confirm_password")]
    // #[validate(length(min = 6))]
    pub password: String,
    // #[validate(must_match(other = "confirm_password"))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLoginInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

// Traits ///////////////////////////////////////////

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    ConnectionPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = ConnectionPool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;
        tracing::debug!("{:#?}", conn);
        Ok(Self(conn))
    }
}

// Main /////////////////////////////

const SALT: &str = "salt_goes_here";

#[tokio::main]
async fn main() {
    match envy::from_env::<AppConfig>() {
        Ok(config) => {
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::new(&config.log_level))
                .with(tracing_subscriber::fmt::layer())
                .init();
            tracing::debug!("ENVIRONMENT: {:#?}", config.environment);
            tracing::debug!("LOG_LEVEL: {:#?}", config.log_level);
            tracing::debug!("POSTGRES_HOST: {:#?}", config.postgres_host);
            tracing::debug!("POSTGRES_PORT: {:#?}", config.postgres_port);
            tracing::debug!("POSTGRES_DB: {:#?}", config.postgres_db);
            tracing::debug!("POSTGRES_USER: {:#?}", config.postgres_user);
            tracing::debug!("REDIS_HOST: {:#?}", config.redis_host);
            tracing::debug!("REDIS_PORT: {:#?}", config.redis_port);

            let store = RedisSessionStore::new(format!(
                "redis://{}:{}/",
                config.redis_host, config.redis_port,
            ))
            .unwrap();
            // let store = MemoryStore::new();
            let secret = rand::thread_rng().gen::<[u8; 128]>();
            let session_layer = SessionLayer::new(store, &secret);

            let connection_string = format!(
                "host={} port={} user={} password={} dbname={} connect_timeout=10",
                config.postgres_host,
                config.postgres_port,
                config.postgres_user,
                config.postgres_password,
                config.postgres_db,
            );
            let manager =
                PostgresConnectionManager::new_from_stringlike(connection_string, NoTls).unwrap();

            let pool = Pool::builder().max_size(30).build(manager).await.unwrap();

            let app = Router::new()
                .route("/", get(handle_root))
                .route("/signup", get(handle_signup).post(handle_create_signup))
                .route("/login", get(handle_login).post(handle_create_login))
                .route("/logout", get(handle_logout))
                .route("/account", get(handle_account_protected))
                .nest_service("/public", get_service(ServeDir::new("public")))
                // Add middleware to all routes
                .layer(session_layer)
                .layer(
                    ServiceBuilder::new()
                        .layer(HandleErrorLayer::new(|error: BoxError| async move {
                            if error.is::<tower::timeout::error::Elapsed>() {
                                Ok(StatusCode::REQUEST_TIMEOUT)
                            } else {
                                Err((
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    format!("Unhandled internal error: {}", error),
                                ))
                            }
                        }))
                        .timeout(Duration::from_secs(10))
                        .layer(TraceLayer::new_for_http())
                        .into_inner(),
                )
                .with_state(pool);

            let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
            tracing::debug!("listening on http://{}", addr);
            if config.environment == "production" {
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .with_graceful_shutdown(shutdown_signal())
                    .await
                    .unwrap();
            } else {
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            }
        }
        Err(error) => panic!("{:#?}", error),
    }
}

// Route Handlers ////////////////////////////////

async fn handle_root() -> impl IntoResponse {
    let template = IndexTemplate {
        title: "App".to_string(),
    };
    HtmlTemplate(template)
}

async fn handle_login() -> impl IntoResponse {
    let template = LoginTemplate {
        title: "App - Login".to_string(),
        errors: validator::ValidationErrors::new(),
    };
    HtmlTemplate(template)
}

async fn handle_signup() -> impl IntoResponse {
    let template = SignupTemplate {
        title: "App - Signup".to_string(),
        errors: validator::ValidationErrors::new(),
    };
    HtmlTemplate(template)
}

async fn handle_create_login(
    State(pool): State<ConnectionPool>,
    mut session: WritableSession,
    ValidatedLoginForm(input): ValidatedLoginForm<CreateLoginInput>,
) -> impl IntoResponse {
    let config = Config::default();
    let hash = argon2::hash_encoded(input.password.as_bytes(), SALT.as_bytes(), &config).unwrap();
    let conn = pool.get().await.map_err(internal_error).unwrap();
    let query = conn
        .query_one(
            "select * FROM accounts where active = true AND email = $1 AND password = $2 LIMIT 1",
            &[&input.email, &hash],
        )
        .await;
    match query {
        Ok(_) => {
            session
                .insert("email", input.email)
                .expect("Session could not be created.");
            Redirect::to("/account").into_response()
        }
        Err(_) => {
            let p = validator::ValidationError::new("password");
            let mut vs = validator::ValidationErrors::new();
            vs.add("password", p);
            let template = LoginTemplate {
                title: "App - Login|Error".to_string(),
                errors: vs,
            };
            HtmlTemplate(template).into_response()
        }
    }
}

async fn handle_create_signup(
    State(pool): State<ConnectionPool>,
    mut session: WritableSession,
    ValidatedSignupForm(input): ValidatedSignupForm<CreateSignupInput>,
) -> impl IntoResponse {
    let config = Config::default();
    let hash = argon2::hash_encoded(input.password.as_bytes(), SALT.as_bytes(), &config).unwrap();
    let conn = pool.get().await.map_err(internal_error).unwrap();
    let query = conn
        .execute(
            "INSERT into accounts (email, password) VALUES($1,$2)",
            &[&input.email, &hash],
        )
        .await;
    match query {
        Ok(_) => {
            session
                .insert("email", input.email)
                .expect("Session could not be created.");
            Redirect::to("/account").into_response()
        }
        Err(_) => {
            let p = validator::ValidationError::new("password");
            let mut vs = validator::ValidationErrors::new();
            vs.add("password", p);
            let template = SignupTemplate {
                title: "App - Signup|Error".to_string(),
                errors: vs,
            };
            HtmlTemplate(template).into_response()
        }
    }
}

async fn handle_logout(mut session: WritableSession) -> impl IntoResponse {
    session.destroy();
    Redirect::to("/")
}

async fn handle_account_protected(session: ReadableSession) -> impl IntoResponse {
    let email = session
        .get::<String>("email")
        .map_or(String::from(""), |s| s);

    if email != "" {
        let authed_template = AccountTemplate {
            title: "App - Account".to_string(),
            email: format!("{}", email),
        };
        HtmlTemplate(authed_template).into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

// async fn handle_error(_err: io::Error) -> impl IntoResponse {
//     (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
// }

// Utility Functions ////////////////////////////////

/// for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// Graceful shutdown //////////////////////////////////

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    //#[cfg(not(unix))]
    //let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::debug!("signal received, starting graceful shutdown");
}
