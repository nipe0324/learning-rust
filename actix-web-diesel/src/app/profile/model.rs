use crate::app::follow::model::Follow;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::follows;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl User {
    pub fn get_profile(
        &self,
        conn: &mut PgConnection,
        followee_id: &Uuid,
    ) -> Result<Profile, AppError> {
        let is_following = &self.is_following(conn, followee_id);
        let profile = Profile {
            username: self.username.to_owned(),
            bio: self.bio.to_owned(),
            image: self.image.to_owned(),
            following: is_following.to_owned(),
        };
        Ok(profile)
    }

    /// Check if a user is following another user
    fn is_following(&self, conn: &mut PgConnection, followee_id: &Uuid) -> bool {
        let follow = follows::table
            .filter(follows::follower_id.eq(&self.id))
            .filter(follows::followee_id.eq(followee_id))
            .limit(1)
            .first::<Follow>(conn);
        follow.is_ok()
    }
}
