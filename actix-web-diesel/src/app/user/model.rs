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
    pub password_hash: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    /// Find a user by user_id
    pub fn find(conn: &mut PgConnection, user_id: Uuid) -> Result<User, AppError> {
        let user = users::table.find(user_id).first::<User>(conn)?;
        Ok(user)
    }

    /// Authenticate a user
    pub fn authenticate(
        conn: &mut PgConnection,
        email: &str,
        password: &str,
    ) -> Result<(User, Token), AppError> {
        let user = users::table
            .filter(users::email.eq(email))
            .limit(1)
            .first::<User>(conn)?;

        hasher::verify(password, &user.password_hash)?;
        let token = user.generate_token()?;
        Ok((user, token))
    }

    /// Create a new user
    pub fn create<'a>(
        conn: &mut PgConnection,
        username: &'a str,
        email: &'a str,
        password: &'a str,
    ) -> Result<(User, Token), AppError> {
        let password_hash = hasher::hash_password(password)?;
        let user = diesel::insert_into(users::table)
            .values(&SignupUser {
                username,
                email,
                password_hash: &password_hash,
            })
            .get_result::<User>(conn)?;

        let token = user.generate_token()?;
        Ok((user, token))
    }

    /// Update a user
    pub fn update(
        conn: &mut PgConnection,
        user_id: Uuid,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
        image: Option<String>,
        bio: Option<String>,
    ) -> Result<User, AppError> {
        let password_hash = match password {
            Some(ref password) => Some(hasher::hash_password(password)?),
            None => None,
        };

        let target_user = users::table.filter(users::id.eq(user_id));
        let user = diesel::update(target_user)
            .set(&UpdateUser {
                username,
                email,
                password_hash,
                image,
                bio,
            })
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanoseconds to seconds
        let token = token::encode(self.id, now)?;
        Ok(token)
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
struct SignupUser<'a> {
    username: &'a str,
    email: &'a str,
    password_hash: &'a str,
}

#[derive(AsChangeset, Debug, Deserialize, Clone)]
#[diesel(table_name = users)]
struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    password_hash: Option<String>,
    image: Option<String>,
    bio: Option<String>,
}
