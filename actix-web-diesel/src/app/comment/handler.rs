use super::{response::MultipleCommentsResponse, service};
use crate::middleware::state::AppState;
use crate::utils::handler::ApiResponse;
use actix_web::{web, HttpResponse};

type ArticleTitleSlug = String;
type CommentIdSlug = String;

pub async fn get_article_comments(
    state: web::Data<AppState>,
    path: web::Path<ArticleTitleSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let slug = path.into_inner();

    let list = service::fetch_article_comments(conn, &service::FetchArticleComments { slug })?;

    let res = MultipleCommentsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
