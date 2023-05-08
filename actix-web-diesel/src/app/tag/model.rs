use crate::app::article::model::Article;
use crate::error::AppError;
use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, Associations)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub article_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Tag {
    pub fn find_tags(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let items = tags::table.get_results::<Self>(conn)?;
        Ok(items)
    }

    pub fn find_tags_by_article_id(
        conn: &mut PgConnection,
        article_id: &Uuid,
    ) -> Result<Vec<Self>, AppError> {
        let tags = tags::table
            .filter(tags::article_id.eq(article_id))
            .get_results::<Self>(conn)?;
        Ok(tags)
    }

    pub fn find_ids_by_name(
        conn: &mut PgConnection,
        tag_name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        let tag_ids = tags::table
            .filter(tags::name.eq(tag_name))
            .select(tags::id)
            .get_results::<Uuid>(conn)?;
        Ok(tag_ids)
    }

    pub fn create_tags(
        conn: &mut PgConnection,
        records: Vec<CreateTag>,
    ) -> Result<Vec<Self>, AppError> {
        let new_tags = diesel::insert_into(tags::table)
            .values(records)
            .get_results::<Self>(conn)?;
        Ok(new_tags)
    }
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct CreateTag<'a> {
    pub article_id: &'a Uuid,
    pub name: &'a String,
}
