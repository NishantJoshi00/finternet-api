use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
pub struct MockTransferRequest {
    pub sender: String,
    pub recipient: String,
    pub asset: MockAsset,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VerbRequest {
    Transfer(MockTransferRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetListing {
    pub token_manager: String,
    pub default: bool,
    pub account: String,
    pub balance: u64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockAsset {
    pub currency: String,
    pub unit: u64,
    pub token_manager: String,
}

#[derive(Debug, Serialize)]
pub struct HistoryElement {
    pub verb: Verb,
    pub asset: MockAsset,
    pub recipient: String,
    pub sender: String,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub transaction_id: String,
    pub status: String,
}

// Response type from solana api
#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
    // data: serde_json::Value,
    pub signature: String,
}
