use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::database::RepoProvider;
use crate::services;

pub fn tweets() -> Router {
    Router::new()
        .route("/new", routing::post(post))
        .route("/:id/delete", routing::post(delete))
}

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}

async fn post(
    Extension(repo_provider): Extension<RepoProvider>,
    form: Form<TweetForm>,
) -> impl IntoResponse {
    let tweet_repo = repo_provider.tweets();
    services::create_tweet(&tweet_repo, &form.message).await;
    Redirect::to("/").into_response()
}

async fn delete(
    Path(id): Path<i32>,
    Extension(repo_provider): Extension<RepoProvider>,
) -> impl IntoResponse {
    let tweet_repo = repo_provider.tweets();
    services::delete_tweet(&tweet_repo, id).await;
    Redirect::to("/").into_response()
}
