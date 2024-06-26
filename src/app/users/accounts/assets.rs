use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::Deserialize;

use crate::error::ConfigurationError;

mod types;

pub fn router<S: Send + Sync + Clone + 'static>() -> Result<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_asset))
        .route(
            "/:asset_id",
            get(get_asset).put(update_asset).delete(delete_asset),
        )
        .route("/:asset_id/asset:verb", post(action_asset));

    Ok(router)
}

#[derive(Debug, Deserialize)]
enum Verb {
    #[serde(rename = ":transfer")]
    Transfer,
    #[serde(rename = ":nominate")]
    Nominate,
}

async fn create_asset(
    Path((_user_id, _account_id)): Path<(String, String)>,
    axum::Json(asset): axum::Json<types::Asset>,
) -> impl IntoResponse {
    // in: asset_data
    // ...
}

async fn get_asset(
    Path((_user_id, _account_id, _asset_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // ...
}

async fn update_asset(
    Path((_user_id, _account_id, _asset_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // ...
}

async fn delete_asset(
    Path((_user_id, _account_id, _asset_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // ...
}

async fn action_asset(
    Path((_user_id, _account_id, _asset_id, _verb)): Path<(String, String, String, Verb)>,
) -> impl IntoResponse {
    format!("verb: {:#?}", _verb)
}
