use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorHandler {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for ErrorHandler {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Sqlx(err) => match err {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
                _ => {
                    println!("{}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                }
            },
        }
    }
}
