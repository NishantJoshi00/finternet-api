use serde::{Deserialize, Serialize};

use crate::storage::types::AssetType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSARequest {
    pub asset_type: AssetType,
    pub smart_contract_refs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSAResponse {
    pub supported_asset_id: String,
    pub asset_type: AssetType,
}
