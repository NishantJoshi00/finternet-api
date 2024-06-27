use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use error_stack::ResultExt;

use crate::error::{log_convert, ApiError, ConfigurationError};
use crate::state::AppState;

mod supported_assets;
mod types;

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

// async fn create_token_manager(State(app_state): State<AppState>, Json(req): Json<types::CreateTokenManagerRequest>) {}

async fn create_token_manager(
    State(app_state): State<AppState>,
    Json(req): Json<types::CreateTokenManagerRequest>,
) -> Result<Json<types::CreateTokenManagerResponse>, ApiError> {
    let token_manager_name = req.token_manager_name.clone();

    let token_manager_id = app_state
        .storage
        .get_token_manager_interface()
        .await
        .change_context(ApiError::CreateTokenManagerError)
        .map_err(log_convert)?
        .create_token_manager(req.into())
        .await
        .change_context(ApiError::CreateTokenManagerError)
        .map_err(log_convert)?;

    Ok(Json(types::CreateTokenManagerResponse {
        token_manager_id,
        token_manager_name,
    }))
}

async fn list_token_managers(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let token_managers = app_state
        .storage
        .get_token_manager_interface()
        .await
        .change_context(ApiError::ListTokenManagersError)
        .map_err(log_convert)?
        .list_token_manager()
        .await
        .change_context(ApiError::ListTokenManagersError)
        .map_err(log_convert)?;

    Ok(Json(
        token_managers
            .into_iter()
            .map(types::CreateTokenManagerResponse::from)
            .collect::<Vec<_>>(),
    )
    .into_response())
}

async fn get_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    ApiError::NotImplemented
}

async fn update_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}

async fn delete_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}
