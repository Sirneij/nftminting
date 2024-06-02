/// User model
///
/// This model represents a user in the system.
#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub thumbnail: Option<String>,
    pub provider: String,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// UserRegistration is a struct that represents the user data that is saved in the
/// database when the user logs in with a third-party service.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub thumbnail: Option<String>,
    pub provider: String,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
}

/// GoogleUserData is a struct that represents the user data that is returned
/// from Google when the user logs in with Google.
#[derive(serde::Deserialize)]
pub struct GoogleUserData {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}
