use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::favorites;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Favorite {
    pub fn create(conn: &mut PgConnection, record: &CreateFavorite) -> Result<Favorite, AppError> {
        let favorite = diesel::insert_into(favorites::table)
            .values(record)
            .get_result::<Favorite>(conn)?;
        Ok(favorite)
    }

    pub fn delete(conn: &mut PgConnection, record: &DeleteFavorite) -> Result<(), AppError> {
        diesel::delete(
            favorites::table
                .filter(favorites::article_id.eq(record.article_id))
                .filter(favorites::user_id.eq(record.user_id)),
        )
        .execute(conn)?;
        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(table_name = favorites)]
pub struct CreateFavorite {
    pub article_id: Uuid,
    pub user_id: Uuid,
}

pub struct DeleteFavorite {
    pub article_id: Uuid,
    pub user_id: Uuid,
}

pub struct FavoriteInfo {
    pub user_id: Uuid,
}
