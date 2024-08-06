use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;

use crate::error::{ApiError, ConfigurationError};
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

async fn create_asset(
    State(app_state): State<AppState>,
    Path(user_id): Path<String>,
    axum::Json(asset): axum::Json<types::MintAssetRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Err::<(), _>(ApiError::NotImplemented)
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
    Path((_user_id, _asset_id, _verb)): Path<(String, String, types::Verb)>,
    Json(action): Json<types::VerbRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Err::<(), _>(ApiError::NotImplemented)
}
