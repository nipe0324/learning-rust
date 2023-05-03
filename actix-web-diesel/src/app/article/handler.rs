use super::model::Article;
use super::request::{ArticlesListQueryParameter, CreateArticleRequest};
use super::response::{MultipleArticlesResponse, SingleArticleResponse};
use super::service;
use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::handler::ApiResponse;
use actix_web::{web, HttpRequest, HttpResponse};

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

pub async fn create_article(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateArticleRequest>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req)?;

    let (article, profile) = service::create_article(
        conn,
        &service::CreateArticleService {
            title: form.article.title.clone(),
            slug: Article::convert_title_to_slug(&form.article.title),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
            current_user,
        },
    )?;

    let res = SingleArticleResponse::from((article, profile));
    Ok(HttpResponse::Ok().json(res))
}
