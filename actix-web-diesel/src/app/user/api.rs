use super::model::User;
use super::{request, response::UserResponse};
// use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpResponse};

pub async fn signin(
    state: web::Data<AppState>,
    form: web::Json<request::SigninForm>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let (user, token) = User::signin(conn, &form.user.email, &form.user.password)?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<request::SignupForm>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let (user, token) = User::signup(
        conn,
        &form.user.username,
        &form.user.email,
        &form.user.password,
    )?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}
