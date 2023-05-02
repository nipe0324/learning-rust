use crate::config::Config;
use anyhow::Context;
use axum::{extract::Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod error;
mod extractor;
// mod types;

mod users;
// mod articles;
// mod profiles;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                db,
            }))
            .layer(TraceLayer::new_for_http()),
    );

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("Failed to run server")
}

fn api_router() -> Router {
    users::router()
    // .merge(profiles::router())
    // .merge(articles::router())
}
