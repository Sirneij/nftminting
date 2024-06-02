/// QueryCode is a struct that represents the query parameters that are sent
/// from the frontend to the backend when the user is redirected back to the
/// backend after logging in with a third-party service.
#[derive(Debug, serde::Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}
