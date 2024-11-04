use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    Server(String),
    Database(String),
    EnvVarNotSet(&'static str),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let res = match self {
            Self::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal error: {}", msg),
            ),
            Self::Server(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("server error: {}", msg),
            ),
            Self::Database(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {}", msg),
            ),
            Self::EnvVarNotSet(name) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("environment variable: {} must be set", name),
            ),
        };
        res.into_response()
    }
}
