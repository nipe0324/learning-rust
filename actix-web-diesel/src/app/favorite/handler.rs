use super::response::SingleArticleResponse;
use super::service;
use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::handler::ApiResponse;
use actix_web::{web, HttpRequest, HttpResponse};

type ArticleSlug = String;

pub async fn create_favorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req)?;
    let slug = path.into_inner();

    let (article, profile, tags_list) = service::create_favorite(conn, current_user, slug)?;

    let res = SingleArticleResponse::from((article, profile, tags_list));
    Ok(HttpResponse::Ok().json(&res))
}

pub async fn delete_favorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req)?;
    let slug = path.into_inner();

    let (article, profile, tags_list) = service::delete_favorite(conn, current_user, slug)?;

    let res = SingleArticleResponse::from((article, profile, tags_list));
    Ok(HttpResponse::Ok().json(&res))
}
