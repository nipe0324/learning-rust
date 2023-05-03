use crate::app::{healthcheck, user};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .route("/healthcheck", get().to(healthcheck::api::index))
            .route("/users/login", post().to(user::api::signin))
            .route("/users", post().to(user::api::signup)),
    );
}
