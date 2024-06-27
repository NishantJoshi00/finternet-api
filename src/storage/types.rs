use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenManagerRef {
    pub id: String,
    pub token_manager_name: String,
    pub internal_addr: String, // better name required
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AssetInfo {
    Cash {  currency: Currency,  amount: u64 },
    // Property {}
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
// TODO: Use macros to generate the following enums
pub enum AssetType {
    Cash,
    // Property
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    USD,
}

pub struct User {
    pub email: String,
    pub name: String,
    pub public_key: String,
    pub ua_addr: String,
}

pub struct TokenManager {
    pub token_manager_name: String,
    pub public_key: String,
}

pub struct TokenManagerInfo {
    pub token_manager_id: String,
    pub token_manager_name: String,
}

pub struct Account {
    pub account_name: String,
    pub token_manager_id: String,
    pub asset_type: AssetType,
    pub token_manager_ref: TokenManagerRef,
}

pub struct TotalAssets {
    pub money: Money,
}

pub struct Money {
    pub currency: Currency,
    pub amount: u64,
}

impl TotalAssets {
    pub fn new() -> Self {
        Self {
            money: Money {
                currency: Currency::USD,
                amount: 0,
            },
        }
    }
}

pub struct SupportedAsset {
    pub asset_type: AssetType,
    pub smart_contract_refs: Vec<u8>,
}
