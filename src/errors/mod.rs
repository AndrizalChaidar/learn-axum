use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorHandler {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
   #[error("Uuid error: {0}")] 
    Uuid(#[from] uuid::Error),
    #[error("Troop type/tribe error: {0}")]
    TroopTypeTribe(String)
}

impl IntoResponse for ErrorHandler {
    fn into_response(self) -> axum::response::Response {
        println!("{:?}", self.to_string());
        match self {
            Self::Sqlx(err) => match err {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
                _ => 
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            },
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response() 
        }
    }
}

