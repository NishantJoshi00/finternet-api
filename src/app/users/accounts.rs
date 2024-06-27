use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use error_stack::ResultExt;

use crate::error::{log_convert, ApiError, ConfigurationError};
use crate::state::AppState;

mod assets;
mod types;

pub fn router() -> Result<axum::Router<AppState>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_account).get(list_accounts))
        .route(
            "/:account_id",
            get(get_account).put(update_account).delete(delete_account),
        )
        .nest("/:account_id/assets", assets::router()?);

    Ok(router)
}

async fn create_account(
    State(app_state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<types::CreateAccountRequest>,
) -> Result<Json<types::CreateAccountResponse>, ApiError> {
    // check if the token manager has the asset type
    let tm_supported_assets = app_state
        .storage
        .get_token_manager_interface()
        .await
        .change_context(ApiError::AccountCreationError)
        .map_err(log_convert)?
        .get_supported_asset_interface(&req.token_manager_id)
        .await
        .change_context(ApiError::AccountCreationError)
        .map_err(log_convert)?
        .list_supported_assets()
        .await
        .change_context(ApiError::AccountCreationError)
        .map_err(log_convert)?;

    let token_manager_id = req.token_manager_id.clone();
    let account_name = req.account_name.clone();
    let asset_type = req.asset_type.clone();

    tm_supported_assets
        .iter()
        .any(|asset| asset.asset_type == req.asset_type)
        .then_some(())
        .ok_or(ApiError::AssetTypeNotSupportedError)?;

    let account_id = app_state
        .storage
        .get_user_interface()
        .await
        .change_context(ApiError::AccountCreationError)
        .map_err(log_convert)?
        .get_account_interface(&user_id)
        .await
        .change_context(ApiError::AccountCreationError)
        .map_err(log_convert)?
        .create_account(req.into())
        .await
        .change_context(ApiError::AccountCreationError)
        .map_err(log_convert)?;

    Ok(Json(types::CreateAccountResponse {
        account_id,
        token_manager_id,
        account_name,
        asset_type,
    }))
}

async fn list_accounts() -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}

async fn get_account(
    State(app_state): State<AppState>,
    Path((user_id, account_id)): Path<(String, String)>,
) -> Result<Json<types::FetchAccountResponse>, ApiError> {
    let account_interface = app_state
        .storage
        .get_user_interface()
        .await
        .change_context(ApiError::FetchAccountError)
        .map_err(log_convert)?
        .get_account_interface(&user_id)
        .await
        .change_context(ApiError::FetchAccountError)
        .map_err(log_convert)?;

    let accounts = account_interface
        .get_account(&account_id)
        .await
        .change_context(ApiError::FetchAccountError)
        .map_err(log_convert)?;

    Ok(Json(types::FetchAccountResponse {
        account: accounts.0,
        total_assets: accounts.1,
    }))
    // Err(ApiError::NotImplemented)
}

async fn update_account(
    Path((_user_id, _account_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}

async fn delete_account(
    Path((_user_id, _account_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}
