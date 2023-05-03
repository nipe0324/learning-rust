use askama::Template;
use axum::{
    async_trait,
    extract::{rejection::FormRejection, Form, FromRequest},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "account.html")]
pub struct AccountTemplate {
    pub title: String,
    pub email: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub title: String,
    pub errors: validator::ValidationErrors,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {
    pub title: String,
    pub errors: validator::ValidationErrors,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedSignupForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedSignupForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = SignupFormError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedSignupForm(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedLoginForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedLoginForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = LoginFormError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedLoginForm(value))
    }
}

#[derive(Debug, Error)]
pub enum SignupFormError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for SignupFormError {
    fn into_response(self) -> Response {
        match self {
            SignupFormError::ValidationError(v) => {
                let template = SignupTemplate {
                    title: "App - Signup|Error".to_string(),
                    errors: v,
                };
                (StatusCode::BAD_REQUEST, HtmlTemplate(template))
            }
            SignupFormError::AxumFormRejection(_) => {
                let empty_errors = validator::ValidationErrors::new();
                let template = SignupTemplate {
                    title: "App - Signup|Error".to_string(),
                    errors: empty_errors,
                };
                (StatusCode::BAD_REQUEST, HtmlTemplate(template))
            }
        }
        .into_response()
    }
}

#[derive(Debug, Error)]
pub enum LoginFormError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for LoginFormError {
    fn into_response(self) -> Response {
        match self {
            LoginFormError::ValidationError(v) => {
                let template = LoginTemplate {
                    title: "App - Login|Error".to_string(),
                    errors: v,
                };
                (StatusCode::BAD_REQUEST, HtmlTemplate(template))
            }
            LoginFormError::AxumFormRejection(_) => {
                let empty_errors = validator::ValidationErrors::new();
                let template = LoginTemplate {
                    title: "App - Login|Error".to_string(),
                    errors: empty_errors,
                };
                (StatusCode::BAD_REQUEST, HtmlTemplate(template))
            }
        }
        .into_response()
    }
}
