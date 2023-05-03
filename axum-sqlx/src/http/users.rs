use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use crate::http::{ApiContext, Result};

use super::error::ResultExt;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/login", post(login_user))
        .route("/api/user", get(get_current_user).put(update_user))
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

#[derive(serde::Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
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
    ctx: State<ApiContext>,
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
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

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

async fn login_user(
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>> {
    let user = sqlx::query!(
        r#"
            select user_id, email, username, bio, image, password_hash
            from "user"
            where email = $1
        "#,
        req.user.email,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(UserBody {
        user: User {
            username: user.username,
            email: user.email,
            token: AuthUser {
                user_id: user.user_id,
            }
            .to_jwt(&ctx),
            bio: user.bio,
            image: user.image,
        },
    }))
}

async fn get_current_user(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
) -> Result<Json<UserBody<User>>> {
    let user = sqlx::query!(
        r#"
            select user_id, email, username, bio, image
            from "user"
            where user_id = $1
        "#,
        auth_user.user_id,
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(UserBody {
        user: User {
            username: user.username,
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            bio: user.bio,
            image: user.image,
        },
    }))
}

async fn update_user(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == UpdateUser::default() {
        return get_current_user(auth_user, ctx).await;
    }

    let password_hash = if let Some(password) = req.user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query!(
        r#"
            update "user"
            set username = coalesce($1, "user".username),
                email = coalesce($2, "user".email),
                password_hash = coalesce($3, "user".password_hash),
                bio = coalesce($4, "user".bio),
                image = coalesce($5, "user".image)
            where user_id = $6
            returning email, username, bio, image
        "#,
        req.user.username,
        req.user.email,
        password_hash,
        req.user.bio,
        req.user.image,
        auth_user.user_id,
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(UserBody {
        user: User {
            username: user.username,
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            bio: user.bio,
            image: user.image,
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

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("failed to parse password hash: {}", e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow::anyhow!("password verification failed"))?;

        Ok(())
    })
    .await
    .context("panic in verifying password")??)
}
