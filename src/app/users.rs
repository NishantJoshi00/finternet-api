use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use error_stack::ResultExt;

use crate::error::{log_convert, ApiError, ConfigurationError};
use crate::solana_connect;
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
    // Err(ApiError::NotImplemented)
    let mock_user = types::GetUserResponse {
        user: types::User {
            ua_addr: "arnab.d@unifiedledger1".to_string(),
            public_key: "7b3e7717a2a479e9c08372d7e20f6ae19cc071f2ad3b66f18d4c84243370153b"
                .to_string(),
            name: "Arnab".to_string(),
            email: "alice@example.com".to_string(),
        },
    };
    match crate::solana_connect(&mock_user, "/user/create").await {
        Ok(Some(signature)) => {
            // Use the signature as the user_id
            let output = types::CreateUserResponse {
                user_id: mock_user.user.ua_addr.clone(),
                ua_addr: mock_user.user.ua_addr.clone(),
                signature,
            };
            Ok(Json(output))
        }
        Ok(None) => {
            // Handle case where signature parsing failed
            Err(ApiError::SolanaProviderError)
        }
        Err(e) => Err(ApiError::CreateUserError),
    }
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
    // Err(ApiError::NotImplemented)

    let output = types::GetUserResponse {
        user: types::User {
            ua_addr: "arnab.d@unifiedledger1".to_string(),
            public_key: "7b3e7717a2a479e9c08372d7e20f6ae19cc071f2ad3b66f18d4c84243370153b"
                .to_string(),
            name: "Arnab".to_string(),
            email: "alice@example.com".to_string(),
        },
    };

    Ok(Json(output))
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
