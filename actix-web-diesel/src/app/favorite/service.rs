use crate::app::article::model::Article;
use crate::app::article::service::{fetch_article_by_slug, FetchArticleBySlug};
use crate::app::favorite::model::{CreateFavorite, DeleteFavorite, Favorite, FavoriteInfo};
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub fn fetch_favorite_info(
    conn: &mut PgConnection,
    article_id: &Uuid,
    user_id: &Uuid,
) -> Result<FavoriteInfo, AppError> {
    let is_favorited = Favorite::is_favorited_article_by_user_id(conn, article_id, user_id)?;
    let favorites_count = Favorite::find_favorites_count_by_article_id(conn, article_id)?;
    let favorite_info = FavoriteInfo {
        is_favorited,
        favorites_count,
    };
    Ok(favorite_info)
}

pub fn create_favorite(
    conn: &mut PgConnection,
    current_user: User,
    slug: String,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
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
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
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
