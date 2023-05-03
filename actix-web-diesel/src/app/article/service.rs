use crate::app::article::model::{Article, CreateArticle};
// use crate::app::favorite::model::{Favorite, FavoriteInfo};
// use crate::app::follow::model::Follow;
use crate::app::profile::model::Profile;
use crate::app::user::model::User;
// use crate::app::tag::model::{CreateTag, Tag};
// use crate::app::user::model::User;
use crate::error::AppError;
// use crate::schema::articles::dsl::*;
use crate::schema::{articles, users};
use actix_web::App;
use diesel::pg::PgConnection;
use diesel::prelude::*;
// use uuid::Uuid;

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

type ArticlesCount = i64;
type ArticlesListInner = (Article, Profile);
type ArticlesList = Vec<ArticlesListInner>;

pub fn fetch_articles_list(
    conn: &mut PgConnection,
    params: FetchArticlesList,
) -> Result<(ArticlesList, ArticlesCount), AppError> {
    let query = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        // TODO:
        // if let Some(tag_name) = &params.tag {}

        if let Some(author_name) = &params.author {
            let ids = Article::find_ids_by_author_name(conn, author_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        // TODO:
        // if let Some(username) = &params.favorited {}

        query
    };

    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)?;

    // TODO: refactor
    let query = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        // TODO:
        // if let Some(tag_name) = &params.tag {}

        if let Some(author_name) = &params.author {
            let ids = Article::find_ids_by_author_name(conn, author_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        // TODO:
        // if let Some(username) = &params.favorited {}

        query
    };

    let articles_list = query
        .order(articles::created_at.desc())
        .offset(params.offset)
        .limit(params.limit)
        .load::<(Article, User)>(conn)?
        .into_iter()
        .map(|(article, user)| {
            (
                article,
                Profile {
                    username: user.username,
                    bio: user.bio,
                    image: user.image,
                    following: false, // NOTE: because not authz
                },
            )
        })
        .collect::<Vec<_>>();

    Ok((articles_list, articles_count))
}

pub struct CreateArticleService {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    // pub tag_name_list: Option<Vec<String>>,
    pub current_user: User,
}

// Create an article
pub fn create_article(
    conn: &mut PgConnection,
    params: &CreateArticleService,
) -> Result<(Article, Profile), AppError> {
    let article = Article::create(
        conn,
        &CreateArticle {
            author_id: params.current_user.id,
            slug: params.slug.clone(),
            title: params.title.clone(),
            description: params.description.clone(),
            body: params.body.clone(),
        },
    )?;

    // TODO
    // let tag_list = create_tag_list(...)

    let profile = params.current_user.get_profile(conn, &article.author_id)?;

    // TODO
    // let favorite_info

    Ok((article, profile))
}
