use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};

use crate::error::ConfigurationError;
use crate::state::AppState;

pub fn router() -> Result<axum::Router<AppState>, ConfigurationError> {
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

async fn create_supported_asset(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // in: asset_type, smart_contract!
    // out:
    // ...
}

async fn list_supported_assets() -> impl IntoResponse {
    // ...
}

async fn get_supported_asset(
    Path((_token_manager_id, _supported_asset_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // ...
}

async fn update_supported_asset(
    Path((_token_manager_id, _supported_asset_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // ...
}

async fn delete_supported_asset(
    Path((_token_manager_id, _supported_asset_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // ...
}
