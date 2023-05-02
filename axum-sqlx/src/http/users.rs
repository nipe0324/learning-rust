use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::extract::Extension;
use axum::routing::post;
use axum::{Json, Router};

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use crate::http::{ApiContext, Result};

pub fn router() -> Router {
    Router::new().route("/api/users", post(create_user))
    // .route("/api/usres/login", post(login_usre)))
    // .route("/api/user", get(get_current_user)))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Deserialize)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    username: String,
    email: String,
    token: String,
    bio: String,
    image: Option<String>,
}

async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;

    let user_id = sqlx::query_scalar!(
        r#"
            insert into "user" (username, email, password_hash)
            values ($1, $2, $3)
            returning user_id
        "#,
        req.user.username,
        req.user.email,
        password_hash,
    )
    .fetch_one(&ctx.db)
    .await?;
    // .constraint("user_username_key", |_| {
    //     Error::unprocessable_entity([("username", "username taken")])
    // })
    // .constraint("user_email_key", |_| {
    //     Error::unprocessable_entity([("email", "email taken")])
    // })?;

    Ok(Json(UserBody {
        user: User {
            username: req.user.username,
            email: req.user.email,
            token: AuthUser { user_id }.to_jwt(&ctx),
            bio: "".to_string(),
            image: None,
        },
    }))
}

async fn hash_password(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string())
    })
    .await
    .context("panic in geratating password hash")??)
}
