use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use error_stack::ResultExt;

use crate::error::{log_convert, ApiError, ConfigurationError};
use crate::state::AppState;

mod supported_assets;
mod types;

/// A router for the token managers API.
///
/// This router handles requests to the `/token_managers` endpoint.
///
///
pub fn router() -> Result<axum::Router<AppState>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_token_manager).get(list_token_managers))
        .route(
            "/:token_manager_id",
            get(get_token_manager)
                .put(update_token_manager)
                .delete(delete_token_manager),
        )
        .nest(
            "/:token_manager_id/supported_assets",
            supported_assets::router()?,
        );

    Ok(router)
}

/// Create a new token manager.
///
async fn create_token_manager(
    State(_app_state): State<AppState>,
    Json(_req): Json<types::CreateTokenManagerRequest>,
) -> Result<Json<types::CreateTokenManagerResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// List all token managers.
///
async fn list_token_managers(
    State(_app_state): State<AppState>,
) -> Result<Json<Vec<types::CreateTokenManagerResponse>>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get a token manager by ID.
///
async fn get_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    ApiError::NotImplemented
}

/// Update a token manager by ID.
///
async fn update_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}

/// Delete a token manager by ID.
///
async fn delete_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}
