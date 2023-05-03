use axum::extract::{Query, State};
use axum::Json;
use futures::TryStreamExt;

use crate::http::articles::{Article, ArticleFromQuery};
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::types::Timestamptz;
use crate::http::ApiContext;
use crate::http::Result;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesBody {
    articles: Vec<Article>,
    articles_count: usize,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct ListArticlesQuery {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    // for pagination
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct FeedArticleQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub(in crate::http) async fn list_articles(
    maybe_auth_user: MaybeAuthUser,
    ctx: State<ApiContext>,
    query: Query<ListArticlesQuery>,
) -> Result<Json<MultipleArticlesBody>> {
    let articles: Vec<_> = sqlx::query_as!(
        ArticleFromQuery,
        r#"
            select
                slug,
                title,
                description,
                body,
                tag_list,
                article.created_at "created_at: Timestamptz",
                article.updated_at "updated_at: Timestamptz",
                exists(select 1 from article_favorite where user_id = $1) "favorited!",
                coalesce(
                    (select count(*) from article_favorite fav where fav.article_id = article.article_id),
                    0
                ) "favorites_count!",
                author.username author_username,
                author.bio author_bio,
                author.image author_image,
                exists(select 1 from follow where followed_user_id = author.user_id and following_user_id = $1) "following_author!"
            from article
            inner join "user" author using (user_id)
            -- the current way to do conditional filtering in SQLx
            where ($2::text is null or tag_list @> array[$2])
            and ($3::text is null or author.username = $3)
            and (
                $4::text is null or exists(
                    select 1
                    from "user"
                    inner join article_favorite af using (user_id)
                    where username = $4
                )
            )
            order by article.created_at desc
            limit $5
            offset $6
        "#,
        maybe_auth_user.user_id(),
        query.tag,
        query.author,
        query.favorited,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0),
    )
    .fetch(&ctx.db)
    .map_ok(ArticleFromQuery::into_article)
    .try_collect()
    .await?;

    Ok(Json(MultipleArticlesBody {
        // This is probably increcct.
        articles_count: articles.len(),
        articles,
    }))
}

pub(in crate::http) async fn feed_articles(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
    query: Query<FeedArticleQuery>,
) -> Result<Json<MultipleArticlesBody>> {
    let articles: Vec<_> = sqlx::query_as!(
        ArticleFromQuery,
        r#"
            select
                slug,
                title,
                description,
                body,
                tag_list,
                article.created_at "created_at: Timestamptz",
                article.updated_at "updated_at: Timestamptz",
                exists(select 1 from article_favorite where user_id = $1) "favorited!",
                coalesce(
                    (select count(*) from article_favorite fav where fav.article_id = article.article_id),
                    0
                ) "favorites_count!",
                author.username author_username,
                author.bio author_bio,
                author.image author_image,
                true "following_author!"
            from follow
            inner join article on followed_user_id = article.user_id
            inner join "user" author using (user_id)
            where following_user_id = $1
            limit $2
            offset $3
        "#,
        auth_user.user_id,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0),
    )
    .fetch(&ctx.db)
    .map_ok(ArticleFromQuery::into_article)
    .try_collect()
    .await?;

    Ok(Json(MultipleArticlesBody {
        articles_count: articles.len(),
        articles,
    }))
}
