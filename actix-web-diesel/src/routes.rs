use crate::app::article::handler::{
    create_article, delete_article, get_article_by_slug, get_articles, get_articles_feed,
    update_article,
};
use crate::app::comment::handler::{
    create_article_comment, delete_article_comment, get_article_comments,
};
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
            .route("/articles", get().to(get_articles))
            .route("/articles", post().to(create_article))
            .route("/articles/feed", get().to(get_articles_feed))
            .route("/articles/{slug}", get().to(get_article_by_slug))
            .route("/articles/{slug}", put().to(update_article))
            .route("/articles/{slug}", delete().to(delete_article))
            .route("/articles/{slug}/comments", get().to(get_article_comments))
            .route(
                "/articles/{slug}/comments",
                post().to(create_article_comment),
            )
            .route(
                "/articles/{slug}/comments/{id}",
                delete().to(delete_article_comment),
            ),
        // TODO: not implemented below apis
        // .route("/articles/{slug}/farovite", post().to(create_farovite))
        // .route("/articles/{slug}/farovite", delete().to(delete_farovite))
        // .route("/tags", get().to(get_tags),
    );
}
