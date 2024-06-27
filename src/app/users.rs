use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use error_stack::ResultExt;

use crate::error::{log_convert, ApiError, ConfigurationError};
use crate::state::AppState;

mod accounts;
mod types;

pub fn router() -> Result<axum::Router<AppState>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_user))
        .route(
            "/:user_id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .nest("/:user_id/accounts", accounts::router()?);

    Ok(router)
}

async fn create_user(
    State(app_state): State<AppState>,
    Json(user): Json<types::CreateUserRequest>,
) -> Result<Json<types::CreateUserResponse>, ApiError> {
    let ua_addr = user.ua_addr.clone();
    let user_id = app_state
        .storage
        .get_user_interface()
        .await
        .change_context(ApiError::CreateUserError)
        .map_err(log_convert)?
        .create_user(user.clone().into())
        .await
        .change_context(ApiError::CreateUserError)
        .map_err(log_convert)?;

    let output = types::CreateUserResponse { user_id, ua_addr };

    Ok(Json(output))
}

async fn get_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = app_state
        .storage
        .get_user_interface()
        .await
        .change_context(ApiError::GetUserError)
        .map_err(log_convert)?
        .get_user(&user_id)
        .await
        .change_context(ApiError::GetUserError)
        .map_err(log_convert)?;

    let output: types::GetUserResponse = user.into();

    Ok(Json(output))
}

async fn update_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}

async fn delete_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    // ...
    ApiError::NotImplemented
}
