use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::{articles, users};
use crate::utils::converter;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Article {
    pub fn convert_title_to_slug(title: &str) -> String {
        converter::to_kebab(title)
    }

    pub fn find_by_slug_and_author_id(
        conn: &mut PgConnection,
        slug: &str,
        author_id: &Uuid,
    ) -> Result<Self, AppError> {
        let item = articles::table
            .filter(articles::slug.eq(slug))
            .filter(articles::author_id.eq(author_id))
            .first::<Self>(conn)?;
        Ok(item)
    }

    pub fn find_ids_by_author_name(
        conn: &mut PgConnection,
        author_name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        let ids = users::table
            .inner_join(articles::table)
            .filter(users::username.eq(author_name))
            .select(articles::id)
            .load::<Uuid>(conn)?;
        Ok(ids)
    }

    pub fn find_by_slug_with_author(
        conn: &mut PgConnection,
        slug: &str,
    ) -> Result<(Self, User), AppError> {
        let result = articles::table
            .inner_join(users::table)
            .filter(articles::slug.eq(slug))
            .get_result::<(Self, User)>(conn)?;
        Ok(result)
    }

    pub fn create(conn: &mut PgConnection, record: &CreateArticle) -> Result<Self, AppError> {
        let article = diesel::insert_into(articles::table)
            .values(record)
            .get_result::<Article>(conn)?;

        Ok(article)
    }

    pub fn update(
        conn: &mut PgConnection,
        slug: &str,
        author_id: &Uuid,
        record: &UpdateArticle,
    ) -> Result<Self, AppError> {
        let article = diesel::update(
            articles::table
                .filter(articles::slug.eq(slug))
                .filter(articles::author_id.eq(author_id)),
        )
        .set(record)
        .get_result::<Article>(conn)?;

        Ok(article)
    }

    pub fn delete(conn: &mut PgConnection, slug: &str, author_id: &Uuid) -> Result<(), AppError> {
        diesel::delete(
            articles::table
                .filter(articles::slug.eq(slug))
                .filter(articles::author_id.eq(author_id)),
        )
        .execute(conn)?;
        // NOTE: references tag rows are deleted automatically by DELETE CASCADE

        Ok(())
    }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = articles)]
pub struct CreateArticle {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = articles)]
pub struct UpdateArticle {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
