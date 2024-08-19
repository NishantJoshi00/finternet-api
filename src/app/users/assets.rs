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
        .route("/", post(create_asset).get(list_assets))
        .route(
            "/:asset_id",
            get(get_asset).put(update_asset).delete(delete_asset),
        )
        .route("/:asset_id/asset:verb", post(action_asset))
        .route("/:asset_id/history", get(get_history));

    Ok(router)
}

async fn create_asset(
    State(_app_state): State<AppState>,
    Path(_user_id): Path<String>,
    axum::Json(_asset): axum::Json<types::MintAssetRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Err::<(), _>(ApiError::NotImplemented)
}

async fn get_history(
    Path((user_id, _asset_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    // Err::<(), _>(ApiError::NotImplemented);

    let output = vec![
        types::HistoryElement {
            verb: types::Verb::Transfer,
            asset: types::MockAsset {
                currency: "USD".to_string(),
                unit: 100,
                token_manager: "Bank of America".to_string(),
            },
            recipient: "alex.t@unifiedledger1".to_string(),
            sender: user_id.clone(),
        },
        types::HistoryElement {
            verb: types::Verb::Transfer,
            asset: types::MockAsset {
                currency: "USD".to_string(),
                unit: 200,
                token_manager: "Bank of America".to_string(),
            },
            sender: "Christina.t@unifiedledger6".to_string(),
            recipient: user_id.clone(),
        },
        types::HistoryElement {
            verb: types::Verb::Transfer,
            asset: types::MockAsset {
                currency: "USD".to_string(),
                unit: 200,
                token_manager: "Bank of America".to_string(),
            },
            sender: "Nicki.t@unifiedledger6".to_string(),
            recipient: user_id,
        },
    ];

    Ok(axum::Json(output))
}

async fn list_assets(State(_app_state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let output = vec![
        types::AssetListing {
            token_manager: "Bank of America".to_string(),
            default: true,
            account: "1234".to_string(),
            balance: 5000,
            currency: "USD".to_string(),
        },
        types::AssetListing {
            token_manager: "JPMorgan Chase & Co.".to_string(),
            default: false,
            account: "1217".to_string(),
            balance: 1000,
            currency: "USD".to_string(),
        },
        types::AssetListing {
            token_manager: "Citi Bank".to_string(),
            default: false,
            account: "1174".to_string(),
            balance: 2000,
            currency: "USD".to_string(),
        },
    ];

    Ok(axum::Json(output))
}

async fn get_asset(
    Path((_user_id, _account_id, _asset_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // ...
    Err::<(), _>(ApiError::NotImplemented)
}

async fn update_asset(Path((_user_id, _asset_id)): Path<(String, String)>) -> impl IntoResponse {
    // ...
    Err::<(), _>(ApiError::NotImplemented)
}

async fn delete_asset(Path((_user_id, _asset_id)): Path<(String, String)>) -> impl IntoResponse {
    // ...
    Err::<(), _>(ApiError::NotImplemented)
}

async fn action_asset(
    State(_app_state): State<AppState>,
    Path((_user_id, _asset_id, verb)): Path<(String, String, types::Verb)>,
    Json(_action): Json<types::VerbRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match verb {
        types::Verb::Transfer => {
            let transaction_id = nanoid::nanoid!();

            match crate::solana_connect(
                &serde_json::json!({ "transaction_id": transaction_id }),
                "/token/transfer",
            )
            .await
            {
                Ok(Some(signature)) => Ok(axum::Json(types::TransferResponse {
                    transaction_id: signature,
                    status: "success".to_string(),
                })),
                Ok(None) => Err(ApiError::SolanaProviderError),
                Err(e) => Err(ApiError::TransferError),
            }
        }
    }
}
