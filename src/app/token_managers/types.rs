use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenManager {
    pub id: Option<String>,
    pub name: String,
    pub public_key: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct CreateTokenManagerRequest {
    #[serde(flatten)]
    pub token_manager: TokenManager,
}

#[derive(Debug, serde::Serialize)]
pub(super) struct CreateTokenManagerResponse {
    pub token_manager_id: String,
    pub token_manager_name: String,
}
