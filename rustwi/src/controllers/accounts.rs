use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::database::RepoProvider;
use crate::services;

pub fn accounts() -> Router {
    Router::new()
        .route("/sign_up", routing::post(sign_up))
        .route("/sign_in", routing::post(sign_in))
}

#[derive(Deserialize)]
struct SignUpForm {
    email: String,
    password: String,
    display_name: String,
}

async fn sign_up(
    Extension(repo_provider): Extension<RepoProvider>,
    form: Form<SignUpForm>,
) -> impl IntoResponse {
    let account_repo = repo_provider.accounts();
    services::create_account(
        &account_repo,
        &form.email,
        &form.password,
        &form.display_name,
    )
    .await;

    Redirect::to("/").into_response()
}

#[derive(Deserialize)]
struct SignInForm {
    email: String,
    password: String,
}

async fn sign_in(
    Extension(repo_provider): Extension<RepoProvider>,
    form: Form<SignInForm>,
) -> impl IntoResponse {
    let account_repo = repo_provider.accounts();
    let is_signed_in = services::authenticate(&account_repo, &form.email, &form.password).await;

    is_signed_in
        .then(|| Redirect::to("/").into_response())
        .ok_or(Redirect::to("/sign_in?error=invalid").into_response())
}
