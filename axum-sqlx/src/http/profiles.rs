use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

// use crate::http::error::ResultExt;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::ApiContext;
use crate::http::{Error, Result};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/api/profiles/:username", get(get_user_profile))
    // .route("/api/profiles/:username/follow",
    //     post(follow_user).delete(unfollow_user),
    // )
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileBody {
    profile: Profile,
}

#[derive(serde::Serialize)]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool,
}

async fn get_user_profile(
    maybe_auth_user: MaybeAuthUser,
    ctx: State<ApiContext>,
    Path(username): Path<String>,
) -> Result<Json<ProfileBody>> {
    println!("username: {}", username);
    let profile = sqlx::query_as!(
        Profile,
        r#"
            select
                username,
                bio,
                image,
                exists(
                    select 1 from follow
                    where followed_user_id = "user".user_id and following_user_id = $2
                ) "following!" -- This tells SQLx that this column will never be null
            from "user"
            where username = $1
        "#,
        username,
        maybe_auth_user.user_id()
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(ProfileBody { profile }))
}
