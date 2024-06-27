use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use error_stack::ResultExt;
use serde::Deserialize;

use crate::error::{log_convert, ApiError, ConfigurationError};
use crate::state::AppState;

mod types;

pub fn router() -> Result<axum::Router<AppState>, ConfigurationError> {
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
    // #[serde(rename = ":nominate")]
    // Nominate,
}

async fn create_asset(
    State(app_state): State<AppState>,
    Path((user_id, account_id)): Path<(String, String)>,
    axum::Json(asset): axum::Json<types::MintAssetRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let asset_store = app_state
        .storage
        .get_user_interface()
        .await
        .change_context(ApiError::CreateAssetError)
        .map_err(log_convert)?
        .get_account_interface(&user_id)
        .await
        .change_context(ApiError::CreateAssetError)
        .map_err(log_convert)?
        .get_asset_interface(&account_id)
        .await
        .change_context(ApiError::CreateAssetError)
        .map_err(log_convert)?;

    let asset = match asset {
        types::MintAssetRequest::Money { currency, amount } => {
            crate::storage::types::AssetInfo::Cash { currency, amount }
        }
    };

    let asset_id = asset_store
        .create_asset(asset.clone())
        .await
        .change_context(ApiError::CreateAssetError)
        .map_err(log_convert)?;

    Ok(axum::response::Json(types::MintAssetResponse {
        asset_id,
        asset_info: asset,
    }))

    // Err::<(), _>(ApiError::NotImplemented)
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
    State(app_state): State<AppState>,
    Path((user_id, account_id, asset_id, verb)): Path<(String, String, String, Verb)>,
    Json(action): Json<types::VerbRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match verb {
        Verb::Transfer => {
            let my_asset_store = app_state
                .storage
                .get_user_interface()
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?
                .get_account_interface(&user_id)
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?
                .get_asset_interface(&account_id)
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?;

            let peer_asset_store = app_state
                .storage
                .get_user_interface()
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?
                .get_account_interface_by_ua(&action.peer_ua_addr)
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?
                .get_asset_interface(&action.account_id)
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?;

            let output = my_asset_store
                .delete_asset(&asset_id)
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?;

            let output = peer_asset_store
                .create_asset(output)
                .await
                .change_context(ApiError::ActionAssetError)
                .map_err(log_convert)?;

            Ok(axum::response::Json(output))
        }
    }
}
