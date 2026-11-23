use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use show360_core::CoreError;
use show360_media::TokenError;

#[derive(Debug)]
pub enum ApiError {
    Core(CoreError),
    Token(TokenError),
    BadRequest(String),
    NotFound(String),
}

impl From<CoreError> for ApiError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}

impl From<TokenError> for ApiError {
    fn from(value: TokenError) -> Self {
        Self::Token(value)
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Core(CoreError::RoomNotFound(id)) => {
                (StatusCode::NOT_FOUND, format!("salle introuvable: {id}"))
            }
            Self::Core(CoreError::RoomFull { max }) => {
                (StatusCode::CONFLICT, format!("salle pleine (max {max})"))
            }
            Self::Core(CoreError::PresenterAlreadyAssigned { current }) => (
                StatusCode::CONFLICT,
                format!("présentateur déjà assigné: {current}"),
            ),
            Self::Core(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Self::Token(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m.clone()),
            Self::NotFound(m) => (StatusCode::NOT_FOUND, m.clone()),
        };

        (status, Json(ErrorBody { error: message })).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
