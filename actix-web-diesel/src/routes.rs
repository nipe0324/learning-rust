use crate::app::{healthcheck, profile, user};
use actix_web::web::{get, post, put, scope, ServiceConfig};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .route("/healthcheck", get().to(healthcheck::api::index))
            .route("/users/login", post().to(user::api::signin))
            .route("/users", post().to(user::api::signup))
            .route("/user", get().to(user::api::get_user))
            .route("/user", put().to(user::api::update_user))
            .route("/profiles/{username}", get().to(profile::api::get_profile)),
    );
}
