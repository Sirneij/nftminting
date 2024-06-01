use crate::utils::SuccessResponse;
use axum::{http::StatusCode, response::IntoResponse};

#[tracing::instrument]
pub async fn health_check() -> impl IntoResponse {
    SuccessResponse {
        message: "Server is running".to_string(),
        status_code: StatusCode::OK.as_u16(),
        ..Default::default()
    }
    .into_response()
}
