use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};

use crate::error::ConfigurationError;

mod supported_assets;

pub fn router<S: Send + Sync + Clone + 'static>() -> Result<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_token_manager).get(list_token_managers))
        .route(
            "/:token_manager_id",
            get(get_token_manager)
                .put(update_token_manager)
                .delete(delete_token_manager),
        )
        .nest("/:token_manager_id/supported_assets", supported_assets::router()?);

    Ok(router)
}

async fn create_token_manager() -> impl IntoResponse {
    // in: token_manager_name, public_key
    // ...
}

async fn list_token_managers() -> impl IntoResponse {
    // ...
}

async fn get_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn update_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn delete_token_manager(Path(_token_manager_id): Path<String>) -> impl IntoResponse {
    // ...
}
