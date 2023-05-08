use super::model::Tag;
use super::response::TagsResponse;
use crate::middleware::state::AppState;
use crate::utils::handler::ApiResponse;
use actix_web::{web, HttpResponse};

pub async fn get_tags(state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.conn()?;
    let tags = Tag::find_tags(conn)?;
    let res = TagsResponse::from(tags);
    Ok(HttpResponse::Ok().json(res))
}
