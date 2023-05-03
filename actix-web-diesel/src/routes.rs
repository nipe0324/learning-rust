use crate::app;
use actix_web::web::{self, get};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/healthcheck").route("", get().to(app::healthcheck::api::index))),
    );
}
