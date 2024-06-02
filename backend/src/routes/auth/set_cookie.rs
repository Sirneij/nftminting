use crate::startup::AppState;

use crate::utils::{CustomAppError, ErrorContext, SuccessResponse};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use time::{Duration, OffsetDateTime};

#[derive(Debug, serde::Deserialize)]
pub struct ID {
    pub user_id: uuid::Uuid,
}

#[axum::debug_handler]
#[tracing::instrument(name = "set_browser_cookie", skip(state))]
pub async fn set_browser_cookie(
    cookies: PrivateCookieJar,
    Query(query): Query<ID>,
    State(state): State<AppState>,
) -> Result<(PrivateCookieJar, impl IntoResponse), CustomAppError> {
    let user_id = query.user_id;

    // Check if user exists
    let _ = state.db_store.get_user_by_id(user_id).await.map_err(|_| {
        CustomAppError::from((
            "Failed to get user".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    // Generate a truly random session id for the user
    let session_id = uuid::Uuid::new_v4().to_string();

    // Save session id in redis
    let mut redis_con = state.redis_store.get().await.map_err(|_| {
        CustomAppError::from((
            "Failed to connect to session store".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    let settings = crate::settings::get_settings().map_err(|_| {
        CustomAppError::from((
            "Failed to read settings".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;
    let cookie_expiration = settings.secret.cookie_expiration;

    bb8_redis::redis::cmd("SET")
        .arg(session_id.clone())
        .arg(user_id.to_string())
        .arg("EX")
        .arg(cookie_expiration * 60)
        .query_async::<_, String>(&mut *redis_con)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "Failed to save session".to_string(),
                ErrorContext::InternalServerError,
            ))
        })?;

    let expires_at = OffsetDateTime::now_utc() + Duration::minutes(cookie_expiration);

    // Create cookie
    let cookie = Cookie::build(("sessionid", session_id))
        .expires(expires_at)
        .secure(true)
        .same_site(SameSite::None)
        .http_only(true)
        .path("/")
        .build();

    Ok((
        cookies.add(cookie),
        SuccessResponse {
            message: "The authentication process was successful.".to_string(),
            status_code: StatusCode::OK.as_u16(),
            ..Default::default()
        }
        .into_response(),
    ))
}
