use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("error starting server: {0}")]
    Server(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
