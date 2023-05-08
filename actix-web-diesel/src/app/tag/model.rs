use crate::app::article::model::Article;
use crate::error::AppError;
use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
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
    pub fn all(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let items = tags::table.get_results::<Self>(conn)?;
        Ok(items)
    }
}
