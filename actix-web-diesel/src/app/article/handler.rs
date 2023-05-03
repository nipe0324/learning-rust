use super::{request::ArticlesListQueryParameter, response::MultipleArticlesResponse, service};
use crate::middleware::state::AppState;
use crate::utils::handler::ApiResponse;
use actix_web::{web, HttpResponse};

type ArticleTitleSlug = String;

pub async fn get_articles(
    state: web::Data<AppState>,
    params: web::Query<ArticlesListQueryParameter>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);

    let (articles_list, articles_count) = service::fetch_articles_list(
        conn,
        service::FetchArticlesList {
            tag: params.tag.clone(),
            author: params.author.clone(),
            favorited: params.favorited.clone(),
            offset,
            limit,
        },
    )?;

    let res = MultipleArticlesResponse::from((articles_list, articles_count));
    Ok(HttpResponse::Ok().json(res))
}
