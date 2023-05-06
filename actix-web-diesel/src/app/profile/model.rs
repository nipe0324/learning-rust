use crate::app::follow::model::Follow;
use crate::app::user::model::User;
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
    /// Get a user's profile
    pub fn get_profile(&self, conn: &mut PgConnection, followee_id: &Uuid) -> Profile {
        let is_following = Follow::is_following(conn, &self.id, followee_id);

        Profile {
            username: self.username.to_owned(),
            bio: self.bio.to_owned(),
            image: self.image.to_owned(),
            following: is_following.to_owned(),
        }
    }
}
