use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct CreateTokenManagerRequest {
    pub token_manager_name: String,
    pub public_key: String,
}

#[derive(Debug, serde::Serialize)]
pub(super) struct CreateTokenManagerResponse {
    pub token_manager_id: String,
    pub token_manager_name: String,
}

impl From<crate::storage::types::TokenManagerInfo> for CreateTokenManagerResponse {
    fn from(value: crate::storage::types::TokenManagerInfo) -> Self {
        Self {
            token_manager_id: value.token_manager_id,
            token_manager_name: value.token_manager_name,
        }
    }
}

impl From<CreateTokenManagerRequest> for crate::storage::types::TokenManager {
    fn from(value: CreateTokenManagerRequest) -> Self {
        Self {
            token_manager_name: value.token_manager_name,
            public_key: value.public_key,
        }
    }
}
