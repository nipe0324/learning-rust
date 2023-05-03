use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use itertools::Itertools;
// use sqlx::{Executor, Postgres};
// use uuid::Uuid;

use crate::http::error::{Error, ResultExt};
use crate::http::extractor::AuthUser; // , MaybeAuthUser};
use crate::http::profiles::Profile;
use crate::http::types::Timestamptz;
use crate::http::{ApiContext, Result};

// mod comments;
mod listing;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route(
            "/api/articles",
            post(create_article).get(listing::list_articles),
        )
        .route("/api/articles/feed", get(listing::feed_articles))
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ArticleBody<T = Article> {
    article: T,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Article {
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
    created_at: Timestamptz,
    updated_at: Timestamptz,
    favorited: bool,
    favorites_count: i64,
    author: Profile,
}

struct ArticleFromQuery {
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
    created_at: Timestamptz,
    updated_at: Timestamptz,
    favorited: bool,
    favorites_count: i64,
    author_username: String,
    author_bio: String,
    author_image: Option<String>,
    following_author: bool,
}

impl ArticleFromQuery {
    fn into_article(self) -> Article {
        Article {
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            tag_list: self.tag_list,
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited: self.favorited,
            favorites_count: self.favorites_count,
            author: Profile {
                username: self.author_username,
                bio: self.author_bio,
                image: self.author_image,
                following: self.following_author,
            },
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateArticle {
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}

async fn create_article(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
    Json(mut req): Json<ArticleBody<CreateArticle>>,
) -> Result<Json<ArticleBody>> {
    let slug = slugify(&req.article.title);
    req.article.tag_list.sort();

    let article = sqlx::query_as!(
        ArticleFromQuery,
        r#"
            with inserted_article as (
                insert into article (user_id, slug, title, description, body, tag_list)
                values ($1, $2, $3, $4, $5, $6)
                returning
                    slug,
                    title,
                    description,
                    body,
                    tag_list,
                    -- This is how you can override the inferred type of a column.
                    created_at "created_at: Timestamptz",
                    updated_at "updated_at: Timestamptz"
            )

            select
                inserted_article.*,
                false "favorited!",
                0::int8 "favorites_count!",
                username author_username,
                bio author_bio,
                image author_image,
                -- user is forbidden to follow themselves
                false "following_author!"
            from inserted_article
            inner join "user" on user_id = $1
        "#,
        auth_user.user_id,
        slug,
        req.article.title,
        req.article.description,
        req.article.body,
        &req.article.tag_list[..],
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("article_slug_key", |_| {
        Error::unprocessable_entity([("slug", format!("duplicate article slug: {}", slug))])
    })?;

    Ok(Json(ArticleBody {
        article: article.into_article(),
    }))
}

/// Convert a title string to a slug for identifing an article.
///
/// E.g. `slugify("Doctests are the Bee's Knees") == "doctests-are-the-bees-knees`"
fn slugify(string: &str) -> String {
    const QUOTE_CHARS: &[char] = &['\'', '"'];

    string
        .split(|c: char| !(QUOTE_CHARS.contains(&c) || c.is_alphanumeric()))
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s.replace(QUOTE_CHARS, "");
            s.make_ascii_lowercase();
            s
        })
        .join("-")
}

#[test]
fn test_slugify() {
    assert_eq!(
        slugify("Segfaults and You: When Raw Pointers Go Wrong"),
        "segfaults-and-you-when-raw-pointers-go-wrong"
    );

    assert_eq!(
        slugify("Why are DB Admins Always Shouting?"),
        "why-are-db-admins-always-shouting"
    );

    assert_eq!(
        slugify("Converting to Rust from C: It's as Easy as 1, 2, 3!"),
        "converting-to-rust-from-c-its-as-easy-as-1-2-3"
    )
}
