use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};

use crate::error::ConfigurationError;

pub fn router<S: Send + Sync + Clone + 'static>() -> Result<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_asset))
        .route(
            "/:asset_id",
            get(get_asset).put(update_asset).delete(delete_asset),
        )
        .route("/:asset_id/asset::verb", post(action_asset));

    Ok(router)
}

async fn create_asset() -> impl IntoResponse {
    // ...
}

async fn get_asset(Path(asset_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn update_asset(Path(asset_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn delete_asset(Path(asset_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn action_asset(Path((asset_id, verb)): Path<(String, String)>) -> impl IntoResponse {
    // ...
}
