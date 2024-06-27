use serde::{Deserialize, Serialize};

use crate::storage::types::{AssetInfo, Currency};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MintAssetRequest {
    Cash { currency: Currency, amount: u64 },
    // Property { lat: f64, lon: f64 },
}

#[derive(Debug, Serialize)]
pub struct MintAssetResponse {
    pub asset_id: String,
    pub asset_info: AssetInfo,
}

#[derive(Debug, Deserialize)]
pub struct VerbRequest {
    pub peer_ua_addr: String,
    pub account_id: String,
}

//
// {
//    "type": "Money",
//    "currency": "USD",
//    "amount": 100
// },
// {
//   "type": "Property",
//   "lat": 37.7749,
//   "lon": -122.4194
// }
//
//
//
//
//
