use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};

use crate::error::ConfigurationError;

pub fn router<S: Send + Sync + Clone + 'static>() -> Result<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_supported_asset).get(list_supported_assets))
        .route(
            "/:supported_asset_id",
            get(get_supported_asset)
                .put(update_supported_asset)
                .delete(delete_supported_asset),
        );

    Ok(router)
}

async fn create_supported_asset() -> impl IntoResponse {
    // ...
}

async fn list_supported_assets() -> impl IntoResponse {
    // ...
}

async fn get_supported_asset(Path(supported_asset_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn update_supported_asset(Path(supported_asset_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn delete_supported_asset(Path(supported_asset_id): Path<String>) -> impl IntoResponse {
    // ...
}
