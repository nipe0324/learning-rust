use anyhow::Context;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::config::Config;

mod error;
mod extractor;
mod types;

mod articles;
mod profiles;
mod users;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = api_router(api_context);

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("Failed to run server")
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .merge(users::router())
        .merge(profiles::router())
        .merge(articles::router())
        .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}
