use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    routing, Router,
};
use serde::Deserialize;

use crate::controllers::{accounts, tweets};
use crate::database::{self, RepoProvider};
use crate::request::UserContext;
use crate::response;
use crate::services;
use crate::views::{SignIn, SignUp};

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .route("/sign_in", routing::get(sign_in))
        .route("/sign_up", routing::get(sign_up))
        .nest("/tweets", tweets::tweets())
        .nest("/accounts", accounts::accounts())
        .layer(database_layer)
}

async fn get(
    Extension(repo_provider): Extension<RepoProvider>,
    _: UserContext,
) -> impl IntoResponse {
    let tweet_repo = repo_provider.tweets();
    let home = services::list_tweets(&tweet_repo).await;
    response::from_template(home)
}

#[derive(Deserialize)]
struct SignInQuery {
    error: Option<String>,
}

async fn sign_in(query: Query<SignInQuery>) -> impl IntoResponse {
    response::from_template(SignIn {
        error: query.error.is_some(),
    })
}

async fn sign_up() -> impl IntoResponse {
    response::from_template(SignUp)
}
