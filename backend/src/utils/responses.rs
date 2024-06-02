use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::utils::CustomAppError;

use serde::{Deserialize, Serialize};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(CustomAppError))]
pub struct CustomAppJson<T>(pub T);

impl<T> IntoResponse for CustomAppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
    pub status_code: u16,
    pub user_id: Option<uuid::Uuid>,
}

impl Default for SuccessResponse {
    fn default() -> Self {
        Self {
            message: "Success".to_string(),
            status_code: StatusCode::OK.as_u16(),
            user_id: None,
        }
    }
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::OK);
        let json_body = axum::Json(self);

        // Convert Json to Response
        let mut response = json_body.into_response();

        // Set the correct status code
        *response.status_mut() = status;

        response
    }
}

/// OAuthResponse is a struct that represents the response from the OAuth server
/// when the user logs in with a third-party service.
///
/// `access_token` is the access token that the frontend can use to make requests
/// to the backend.
/// `id_token` is the JWT token that the frontend can use to authenticate the user.
#[derive(Debug, Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}
