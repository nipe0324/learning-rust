use super::{
    request::CreateArticleCommentRequest,
    response::{MultipleCommentsResponse, SingleCommentResponse},
    service,
};
use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::handler::ApiResponse;
use actix_web::{web, HttpRequest, HttpResponse};

type ArticleTitleSlug = String;
type CommentIdSlug = String;

pub async fn get_article_comments(
    state: web::Data<AppState>,
    path: web::Path<ArticleTitleSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let slug = path.into_inner();

    let list =
        service::fetch_article_comments(conn, &service::FetchArticleCommentsService { slug })?;

    let res = MultipleCommentsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn create_article_comment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleTitleSlug>,
    form: web::Json<CreateArticleCommentRequest>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req)?;
    let slug = path.into_inner();

    let (comment, profile) = service::create_article_comment(
        conn,
        &service::CreateArticleCommentService {
            slug,
            body: form.comment.body.to_owned(),
            author: current_user,
        },
    )?;

    let res = SingleCommentResponse::from((comment, profile));
    Ok(HttpResponse::Ok().json(res))
}
