use crate::app::article::model::Article;
use crate::app::article::service::{fetch_article_by_slug, FetchArticleBySlug};
use crate::app::favorite::model::{CreateFavorite, DeleteFavorite, Favorite};
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;

pub fn create_favorite(
    conn: &mut PgConnection,
    current_user: User,
    slug: String,
) -> Result<(Article, Profile, Vec<Tag>), AppError> {
    let article = Article::find_by_slug_and_author_id(conn, slug.as_str(), &current_user.id)?;

    Favorite::create(
        conn,
        &CreateFavorite {
            user_id: current_user.id,
            article_id: article.id,
        },
    )?;

    let item = fetch_article_by_slug(conn, &FetchArticleBySlug { slug: article.slug })?;

    Ok(item)
}

pub fn delete_favorite(
    conn: &mut PgConnection,
    current_user: User,
    slug: String,
) -> Result<(Article, Profile, Vec<Tag>), AppError> {
    let article = Article::find_by_slug_and_author_id(conn, slug.as_str(), &current_user.id)?;

    Favorite::delete(
        conn,
        &DeleteFavorite {
            user_id: current_user.id,
            article_id: article.id,
        },
    )?;

    let item = fetch_article_by_slug(conn, &FetchArticleBySlug { slug: article.slug })?;

    Ok(item)
}
