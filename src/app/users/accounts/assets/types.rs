use serde::Deserialize;

use crate::storage::types::Currency;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MintAssetRequest {
    Money { currency: Currency, amount: u64 },
    // Property { lat: f64, lon: f64 },
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
