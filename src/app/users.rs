use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;

use crate::error::{ApiError, ConfigurationError};
use crate::state::AppState;

mod assets;
mod types;

/// A router for the users API.
///
/// This router handles requests to the `/users` endpoint.
///
///
pub fn router() -> Result<axum::Router<AppState>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_user))
        .route(
            "/:user_id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .nest("/:user_id/assets", assets::router()?);

    Ok(router)
}

/// Create a new user.
///
/// ## Arguments
/// - `app_state`: The application state.
/// - `user`: [`CreateUserRequest`] The request type for the API
///
/// ## Returns
///
/// The response type [`CreateUserResponse`] for the API.
async fn create_user(
    State(_app_state): State<AppState>,
    Json(_user): Json<types::CreateUserRequest>,
) -> Result<Json<types::CreateUserResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Get a user by ID.
///
///
/// ## Arguments
/// - `app_state`: The application state.
/// - `user_id`: The ID of the user to get.
///
/// ## Returns
/// The response type [`GetUserResponse`] for the API.
///
async fn get_user(
    State(_app_state): State<AppState>,
    Path(_user_id): Path<String>,
) -> Result<Json<types::GetUserResponse>, ApiError> {
    Err(ApiError::NotImplemented)
}

/// Update a user by ID.
///
async fn update_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}

/// Delete a user by ID.
///
async fn delete_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}
