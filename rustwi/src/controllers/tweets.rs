use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::database::RepoProvider;
use crate::services;

pub fn tweets() -> Router {
    Router::new().route("/", routing::post(post))
}

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}

async fn post(_form: Form<TweetForm>) -> impl IntoResponse {
    Redirect::to("/").into_response()
}

// async fn post(
//     form: Form<TweetForm>,
//     Extension(repo_provider): Extension<RepoProvider>,
// ) -> impl IntoResponse {
//     let tweet_repo = repo_provider.tweets();
//     services::create_tweet(&tweet_repo, &form.message).await;
//     Redirect::to("/").into_response()
// }
