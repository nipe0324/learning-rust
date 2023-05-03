use crate::app::{healthcheck, profile, user};
use actix_web::web::{get, post, put, scope, ServiceConfig};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .route("/healthcheck", get().to(healthcheck::handler::index))
            .route("/users/login", post().to(user::handler::signin))
            .route("/users", post().to(user::handler::signup))
            .route("/user", get().to(user::handler::get_user))
            .route("/user", put().to(user::handler::update_user))
            .route(
                "/profiles/{username}",
                get().to(profile::handler::get_profile),
            ), // .route(
               //     "/profiles/{username}/follow",
               //     post().to(follow::handler::create_follow),
               // )
               // .route(
               //     "/profiles/{username}/follow",
               //     delete().to(follow::handler::delete_follow),
               // ),
    );
}
