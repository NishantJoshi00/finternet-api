use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum Verb {
    #[serde(rename = ":transfer")]
    Transfer,
    // #[serde(rename = ":nominate")]
    // Nominate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub asset_class_id: String,
    pub chain_id: String,
    pub asset_state: AssetState,
    pub unit: u64,
    pub token_manager: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub holder: Vec<String>,
    pub asset_type: AssetType,
}

#[derive(Debug, Deserialize, Serialize)]
enum AssetState {
    Locked,
    Unlocked,
}

#[derive(Debug, Deserialize, Serialize)]
enum AssetType {
    Fungible,
    NonFungible,
}

// -- --

#[derive(Debug, Deserialize)]
pub struct MintAssetRequest {
    pub asset_class_id: String,
    pub chain_id: String,
    pub unit: u64,
    pub token_manager: String,
    pub holder: Vec<String>,
    pub asset_type: AssetType,
}

#[derive(Debug, Deserialize)]
pub struct TransferAssetRequest {
    pub asset_class_id: String,
    pub chain_id: String,
    pub unit: u64,
    pub token_manager: String,
    pub holder: Vec<String>,
    pub asset_type: AssetType,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VerbRequest {
    Transfer(TransferAssetRequest),
}
