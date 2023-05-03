use crate::app::user::model::User;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserResponse {
    pub user: AuthUser,
}

impl From<(User, String)> for UserResponse {
    fn from((user, token): (User, String)) -> Self {
        Self {
            user: AuthUser {
                username: user.username,
                email: user.email,
                bio: user.bio,
                image: user.image,
                token,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthUser {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
