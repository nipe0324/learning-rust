use crate::error::AppError;
use crate::schema::users;
use crate::utils::{hasher, token};
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Token = String;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn signup<'a>(
        conn: &mut PgConnection,
        username: &'a str,
        email: &'a str,
        password: &'a str,
    ) -> Result<(User, Token), AppError> {
        let password_hash = hasher::hash_password(password)?;

        let record = SignupUser {
            username,
            email,
            password_hash: &password_hash,
        };

        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;

        let token = user.generate_token()?;

        Ok((user, token))
    }

    fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanoseconds to seconds
        let token = token::encode(self.id, now)?;
        Ok(token)
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password_hash: &'a str,
}
