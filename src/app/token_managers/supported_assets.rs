use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use error_stack::ResultExt;

use crate::error::{log_convert, ApiError, ConfigurationError};
use crate::state::AppState;

mod types;

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

async fn create_supported_asset(
    State(app_state): State<AppState>,
    Path(token_manager_id): Path<String>,
    Json(req): Json<types::CreateSARequest>,
) -> Result<impl IntoResponse, ApiError> {
    let output = app_state
        .storage
        .get_token_manager_interface()
        .await
        .change_context(ApiError::CreateSupportedAssetError)
        .map_err(log_convert)?
        .get_supported_asset_interface(&token_manager_id)
        .await
        .change_context(ApiError::CreateSupportedAssetError)
        .map_err(log_convert)?
        .create_supported_asset(crate::storage::types::SupportedAsset {
            asset_type: req.asset_type.clone(),
            smart_contract_refs: req.smart_contract_refs.as_bytes().to_vec(),
        })
        .await
        .change_context(ApiError::CreateSupportedAssetError)
        .map_err(log_convert)?;

    Ok(Json(types::CreateSAResponse {
        supported_asset_id: output,
        asset_type: req.asset_type,
    }))
}

async fn list_supported_assets(
    State(app_state): State<AppState>,
    Path(token_manager_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let output = app_state
        .storage
        .get_token_manager_interface()
        .await
        .change_context(ApiError::CreateSupportedAssetError)
        .map_err(log_convert)?
        .get_supported_asset_interface(&token_manager_id)
        .await
        .change_context(ApiError::CreateSupportedAssetError)
        .map_err(log_convert)?
        .list_supported_assets()
        .await
        .change_context(ApiError::CreateSupportedAssetError)
        .map_err(log_convert)?;

    Ok(Json(output))
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
