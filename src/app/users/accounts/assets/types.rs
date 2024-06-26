use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Asset {
    Money { currency: String, amount: usize },
    Property { lat: f64, lon: f64 },
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
