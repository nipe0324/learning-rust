use crate::app::user::model::User;
// use crate::error::AppError;
use crate::schema::follows;
use chrono::NaiveDateTime;
// use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Associations, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = follower_id, foreign_key = followee_id))]
#[diesel(table_name = follows)]
pub struct Follow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
