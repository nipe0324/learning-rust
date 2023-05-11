use crate::app::article::model::{Article, CreateArticle, UpdateArticle};
use crate::app::favorite::{
    model::{Favorite, FavoriteInfo},
    service::fetch_favorite_info,
};
use crate::app::follow::model::Follow;
use crate::app::profile::model::Profile;
use crate::app::tag::model::{CreateTag, Tag};
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::articles::dsl::*;
use crate::schema::{articles, follows, tags, users};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

// Fetch articles

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

type ArticlesCount = i64;
type ArticlesListInner = (Article, Profile, FavoriteInfo);
type ArticlesList = Vec<(ArticlesListInner, Vec<Tag>)>;

pub fn fetch_articles_list(
    conn: &mut PgConnection,
    params: FetchArticlesList,
) -> Result<(ArticlesList, ArticlesCount), AppError> {
    let query = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let ids = Tag::find_ids_by_name(conn, tag_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(author_name) = &params.author {
            let ids = Article::find_ids_by_author_name(conn, author_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(username) = &params.favorited {
            let ids = Favorite::find_favorited_article_ids_by_username(conn, username)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        query
    };

    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)?;

    // TODO: refactor
    let query = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let ids = Tag::find_ids_by_name(conn, tag_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(author_name) = &params.author {
            let ids = Article::find_ids_by_author_name(conn, author_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(username) = &params.favorited {
            let ids = Favorite::find_favorited_article_ids_by_username(conn, username)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        query
    };

    let article_and_user_list = query
        .offset(params.offset)
        .limit(params.limit)
        .load::<(Article, User)>(conn)?;

    let tags_list = {
        let article_list = article_and_user_list
            .iter()
            .map(|(article, _)| article.to_owned())
            .collect::<Vec<_>>();

        let tags_list = Tag::belonging_to(&article_list)
            .order(tags::name.asc())
            .load::<Tag>(conn)?;

        let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&article_list);

        tags_list
    };

    let favorites_count_list = {
        let list: Result<Vec<i64>, _> = article_and_user_list
            .clone()
            .into_iter()
            .map(|(article, _)| Favorite::find_favorites_count_by_article_id(conn, &article.id))
            .collect();

        list?
    };

    let articles_list = article_and_user_list
        .into_iter()
        .zip(favorites_count_list)
        .map(|((article, user), favorites_count)| {
            (
                article,
                Profile {
                    username: user.username,
                    bio: user.bio,
                    image: user.image,
                    following: false, // NOTE: because not authz
                },
                FavoriteInfo {
                    is_favorited: false, // NOTE: because not authz
                    favorites_count,
                },
            )
        })
        .zip(tags_list)
        .collect::<Vec<_>>();

    Ok((articles_list, articles_count))
}

// Fetch following articles

pub struct FetchFollowingArticlesService {
    pub current_user: User,
    pub offset: i64,
    pub limit: i64,
}

pub fn fetch_following_articles(
    conn: &mut PgConnection,
    params: FetchFollowingArticlesService,
) -> Result<(ArticlesList, ArticlesCount), AppError> {
    let query = {
        let ids = Follow::find_folowee_ids_by_follower_id(conn, &params.current_user.id)?;
        articles.filter(articles::author_id.eq_any(ids))
    };

    let articles_count = query
        .to_owned()
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)?;

    let articles_list = {
        let article_and_user_list = query
            .inner_join(users::table)
            .limit(params.limit)
            .offset(params.offset)
            .order(articles::created_at.desc())
            .get_results::<(Article, User)>(conn)?;

        let follows_list = {
            let user_ids_list = article_and_user_list
                .clone() // TODO: avoid clone
                .into_iter()
                .map(|(_, user)| user.id)
                .collect::<Vec<_>>();

            let list = follows::table
                .filter(follows::follower_id.eq(params.current_user.id))
                .filter(follows::followee_id.eq_any(user_ids_list))
                .get_results::<Follow>(conn)?;

            list.into_iter()
        };

        let tags_list = {
            let article_list = article_and_user_list
                .iter()
                .map(|(article, _)| article.to_owned())
                .collect::<Vec<_>>();

            let tags_list = Tag::belonging_to(&article_list)
                .order(tags::name.asc())
                .load::<Tag>(conn)?;

            let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&article_list);

            tags_list
        };

        let favorites_count_list = {
            let list: Result<Vec<i64>, _> = article_and_user_list
                .clone()
                .into_iter()
                .map(|(article, _)| Favorite::find_favorites_count_by_article_id(conn, &article.id))
                .collect();

            list?
        };

        let favorited_article_ids = Favorite::find_favorited_article_ids_by_username(
            conn,
            params.current_user.username.as_str(),
        )?;
        let is_favorited_by_me = |article: &Article| {
            favorited_article_ids
                .iter()
                .copied()
                .any(|_id| _id == article.id)
        };

        article_and_user_list
            .into_iter()
            .zip(favorites_count_list)
            .map(|((article, user), favorites_count)| {
                let following = follows_list.clone().any(|item| item.followee_id == user.id);
                let is_favorited = is_favorited_by_me(&article);
                (
                    article,
                    Profile {
                        username: user.username,
                        bio: user.bio,
                        image: user.image,
                        following: following.to_owned(),
                    },
                    FavoriteInfo {
                        is_favorited,
                        favorites_count,
                    },
                )
            })
            .zip(tags_list)
            .collect::<Vec<_>>()
    };

    Ok((articles_list, articles_count))
}

// Fetch an article by slug
pub struct FetchArticleBySlug {
    pub slug: String,
}

pub fn fetch_article_by_slug(
    conn: &mut PgConnection,
    params: &FetchArticleBySlug,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let (article, author) = Article::find_by_slug_with_author(conn, &params.slug)?;
    let profile = author.get_profile(conn, &author.id);
    let tags_list = Tag::find_tags_by_article_id(conn, &article.id)?;
    let favorite_info = fetch_favorite_info(conn, &article.id, &author.id)?;
    Ok((article, profile, favorite_info, tags_list))
}

// Create an article

pub struct CreateArticleService {
    pub current_user: User,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_name_list: Option<Vec<String>>,
}

pub fn create_article(
    conn: &mut PgConnection,
    params: &CreateArticleService,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let title_slug = Article::convert_title_to_slug(&params.title);

    let article = Article::create(
        conn,
        &CreateArticle {
            slug: title_slug,
            author_id: params.current_user.id,
            title: params.title.clone(),
            description: params.description.clone(),
            body: params.body.clone(),
        },
    )?;

    let tags_list = create_tags_list(conn, &article.id, &params.tag_name_list)?;

    let profile = params.current_user.get_profile(conn, &article.author_id);

    let favorite_info = fetch_favorite_info(conn, &article.id, &article.author_id)?;

    Ok((article, profile, favorite_info, tags_list))
}

fn create_tags_list(
    conn: &mut PgConnection,
    article_id: &Uuid,
    tag_name_list: &Option<Vec<String>>,
) -> Result<Vec<Tag>, AppError> {
    let list = tag_name_list
        .as_ref()
        .map(|tag_name_list| {
            let records = tag_name_list
                .iter()
                .map(|name| CreateTag { name, article_id })
                .collect();
            Tag::create_tags(conn, records)
        })
        .unwrap_or_else(|| Ok(vec![]));
    list
}

// Update an article

pub struct UpdateArticleServide {
    pub current_user: User,
    pub slug: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    // pub tag_name_list: Option<Vec<String>>,
}

pub fn update_artilce(
    conn: &mut PgConnection,
    params: &UpdateArticleServide,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    Article::find_by_slug_with_author(conn, &params.slug)?;

    let title_slug = params
        .title
        .as_ref()
        .map(|t| Article::convert_title_to_slug(t));

    let article = Article::update(
        conn,
        &params.slug,
        &params.current_user.id,
        &UpdateArticle {
            slug: title_slug,
            title: params.title.to_owned(),
            description: params.description.to_owned(),
            body: params.body.to_owned(),
        },
    )?;

    let tags_list = Tag::find_tags_by_article_id(conn, &article.id)?;

    let profile = params.current_user.get_profile(conn, &article.author_id);

    let favorite_info = fetch_favorite_info(conn, &article.id, &article.author_id)?;

    Ok((article, profile, favorite_info, tags_list))
}

// Delete an article

pub struct DeleteArticleService {
    pub current_user: User,
    pub slug: String,
}

pub fn delete_article(
    conn: &mut PgConnection,
    params: &DeleteArticleService,
) -> Result<(), AppError> {
    Article::find_by_slug_with_author(conn, &params.slug)?;

    Article::delete(conn, &params.slug, &params.current_user.id)?;

    Ok(())
}
