#[tracing::instrument(name = "google_request_token", skip(authorization_code, settings))]
pub async fn google_request_token(
    authorization_code: &str,
    settings: &crate::settings::Settings,
) -> Result<crate::utils::OAuthResponse, crate::utils::CustomAppError> {
    let redirect_url = format!("{}/api/auth/google", settings.application.base_url);
    let client_secret = settings.google.auth_client_secret.as_str();
    let client_id = settings.google.auth_client_id.as_str();
    let root_url = settings.google.auth_request_token_url.as_str();

    let client = reqwest::Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", &redirect_url),
        ("client_id", client_id),
        ("code", authorization_code),
        ("client_secret", client_secret),
    ];
    let response = client.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        let oauth_response = response.json::<crate::utils::OAuthResponse>().await?;
        Ok(oauth_response)
    } else {
        Err(crate::utils::CustomAppError::from((
            "An error occurred while trying to retrieve access token.".to_string(),
            crate::utils::ErrorContext::BadRequest,
        )))
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
    settings: &crate::settings::Settings,
) -> Result<crate::models::GoogleUserData, crate::utils::CustomAppError> {
    let client = reqwest::Client::new();
    let mut url = reqwest::Url::parse(&settings.google.auth_user_info_url).unwrap();

    url.query_pairs_mut()
        .append_pair("alt", "json")
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await?;

    if !response.status().is_success() {
        return Err(crate::utils::CustomAppError::from((
            "An error occurred while trying to retrieve user information.".to_string(),
            crate::utils::ErrorContext::BadRequest,
        )));
    }

    let user_info = response.json::<crate::models::GoogleUserData>().await?;
    Ok(user_info)
}
