use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = (
            StatusCode::INTERNAL_SERVER_ERROR,
            50000,
            "服务器错误".to_string(),
        );

        let body = Json(json!({
            "code": code,
            "message": message,
            "data": null
        }));

        (status, body).into_response()
    }
}
