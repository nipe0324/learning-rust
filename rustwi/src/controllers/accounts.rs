use axum::{
    extract::{Extension, Form},
    http::header::HeaderMap,
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::database::RepoProvider;
use crate::services::{self, SessionToken};

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

    let session_token = services::authenticate(&account_repo, &form.email, &form.password).await;
    match redirect_with_session(session_token) {
        Ok(response) => response.into_response(), // fixme: it doesn't work
        Err(response) => response.into_response(),
    };
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
    let session_token = services::authenticate(&account_repo, &form.email, &form.password).await;
    redirect_with_session(session_token)
}

fn redirect_with_session(
    session: Option<SessionToken>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(session_token) = session {
        let mut headers = HeaderMap::new();
        headers.insert("Set-Cookie", session_token.cookie().parse().unwrap());
        let response = Redirect::to("/");
        Ok((headers, response).into_response())
    } else {
        Err(Redirect::to("/sign_in?error=invalid").into_response())
    }
}
