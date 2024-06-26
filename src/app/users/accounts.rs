use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};

use crate::error::ConfigurationError;

mod assets;

pub fn router<S: Send + Sync + Clone + 'static>() -> Result<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_account).get(list_accounts))
        .route(
            "/:account_id",
            get(get_account).put(update_account).delete(delete_account),
        )
        .nest("/assets", assets::router()?);

    Ok(router)
}

async fn create_account() -> impl IntoResponse {
    // ...
}

async fn list_accounts() -> impl IntoResponse {
    // ...
}

async fn get_account(Path(account_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn update_account(Path(account_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn delete_account(Path(account_id): Path<String>) -> impl IntoResponse {
    // ...
}
