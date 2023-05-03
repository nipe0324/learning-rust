use crate::app::article::handler::get_articles;
use crate::app::follow::handler::{create_follow, delete_follow};
use crate::app::healthcheck::handler::get_healthcheck;
use crate::app::profile::handler::get_profile;
use crate::app::user::handler::{get_user, signin, signup, update_user};
use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .route("/healthcheck", get().to(get_healthcheck))
            .route("/users/login", post().to(signin))
            .route("/users", post().to(signup))
            .route("/user", get().to(get_user))
            .route("/user", put().to(update_user))
            .route("/profiles/{username}", get().to(get_profile))
            .route("/profiles/{username}/follow", post().to(create_follow))
            .route("/profiles/{username}/follow", delete().to(delete_follow))
            .route("/articles", get().to(get_articles)),
        // .route("/articles", post().to(create_article))
        // .route("/articles/feed", get().to(get_articles_feed)),
    );
}
