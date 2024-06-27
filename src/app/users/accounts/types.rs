use serde::{Deserialize, Serialize};

use crate::storage::types::{Account, AssetType, TokenManagerRef, TotalAssets};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub token_manager_id: String,
    pub account_name: String,
    pub asset_type: AssetType,
    pub token_manager_ref: TokenManagerRef,
}

impl From<CreateAccountRequest> for crate::storage::types::Account {
    fn from(value: CreateAccountRequest) -> Self {
        Self {
            account_name: value.account_name,
            token_manager_id: value.token_manager_id,
            asset_type: crate::storage::types::AssetType::Cash,
            token_manager_ref: value.token_manager_ref,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub account_id: String,
    pub token_manager_id: String,
    pub account_name: String,
    pub asset_type: AssetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchAccountResponse {
    pub account: Account,
    pub total_assets: TotalAssets,
}
