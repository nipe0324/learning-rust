use crate::constants::{database_url, AXUM_SESSION_COOKIE_NAME, AXUM_SESSION_USER_ID_KEY};
use async_session::SessionStore;
use async_sqlx_session::PostgresSessionStore;
use axum::extract::{FromRequest, TypedHeader};
use axum::headers::Cookie;
use axum::http::Request;
use axum::response::Redirect;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserContext {
    pub user_id: i32,
}

#[axum::async_trait]
impl<S, B> FromRequest<S, B> for UserContext
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let redirect = || Redirect::to("/sign_in");

        let database_url = database_url();
        let store = PostgresSessionStore::new(&database_url)
            .await
            .map_err(|_| redirect())?;
        let cookies = Option::<TypedHeader<Cookie>>::from_request(req, state)
            .await
            .unwrap()
            .ok_or(redirect())?;
        let session_str = cookies.get(AXUM_SESSION_COOKIE_NAME).ok_or(redirect())?;
        let session = store
            .load_session(session_str.to_string())
            .await
            .map_err(|_| redirect())?;
        let session = session.ok_or(redirect())?;
        let context = UserContext {
            user_id: session.get::<i32>(AXUM_SESSION_USER_ID_KEY).unwrap(),
        };
        Ok(context)
    }
}
