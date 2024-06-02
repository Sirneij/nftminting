use crate::startup::AppState;

use crate::utils::{CustomAppError, ErrorContext};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

#[axum::debug_handler]
#[tracing::instrument(name = "google_oauth", skip(state, query))]
pub async fn google_oauth(
    Query(query): Query<crate::models::QueryCode>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, CustomAppError> {
    let code = query.code.as_str();
    let _query_state = query.state.as_str();

    tracing::info!("Google OAuth code: {}", code);

    if code.is_empty() {
        return Err(CustomAppError::from((
            "Authorization code not provided!".to_string(),
            ErrorContext::UnauthorizedAccess,
        )));
    }

    let settings = crate::settings::get_settings().map_err(|_| {
        CustomAppError::from((
            "Failed to read settings".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    let token_response = crate::utils::google_request_token(code, &settings)
        .await
        .map_err(|e| e)?;

    let google_user = crate::utils::get_google_user(
        &token_response.access_token,
        &token_response.id_token,
        &settings,
    )
    .await
    .map_err(|e| e)?;

    let email = google_user.email.to_lowercase();
    let user_from_db = state.db_store.get_user_by_email(email.as_str()).await;

    let user_reg = crate::models::UserRegistration {
        email,
        name: google_user.name,
        thumbnail: Some(google_user.picture),
        provider: "Google".to_string(),
        is_active: Some(true),
        is_staff: Some(false),
        is_superuser: Some(false),
    };

    let user_id: uuid::Uuid;

    // Create a new user if the user does not exist
    if user_from_db.is_err() {
        let created_user = state.db_store.create_user(&user_reg).await.map_err(|e| e)?;
        user_id = created_user.id;
    } else {
        user_id = user_from_db.as_ref().unwrap().id;

        // Update user data if not up-to-date
        if user_from_db.as_ref().unwrap().name != user_reg.name
            || user_from_db.unwrap().thumbnail != user_reg.thumbnail
        {
            state
                .db_store
                .update_user(&user_id, &user_reg)
                .await
                .map_err(|e| e)?;
        }
    }

    let redirect_url = format!(
        "{}/manage-redirect?state={}&user_id={}",
        settings.frontend_url, _query_state, user_id
    );

    let response = axum::http::Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", redirect_url)
        .body(axum::body::Body::empty())
        .map_err(|_| {
            CustomAppError::from((
                "Failed to create response".to_string(),
                ErrorContext::InternalServerError,
            ))
        })?;

    Ok(response)
}
