use crate::utils::validate_authentication_session;
use axum::{
    routing::{get, post},
    Router,
};

mod current_user;
mod github;
mod google;
mod logout;
mod set_cookie;

pub fn auth_routes(state: crate::startup::AppState) -> Router<crate::startup::AppState> {
    Router::new()
        .route("/logout", post(logout::logout_user))
        .route("/current", get(current_user::get_current_user))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            validate_authentication_session,
        ))
        .route("/google", get(google::google_oauth))
        .route("/set-cookie", post(set_cookie::set_browser_cookie))
}
