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
        .nest("/:account_id/assets", assets::router::<S>()?);

    Ok(router)
}

async fn create_account() -> impl IntoResponse {
    // in: token_manager_id, account_name, asset_type, token_manager_ref
    // out: account_id
    "create_account"
    // ...
}

async fn list_accounts() -> impl IntoResponse {
    // ...
}

async fn get_account(Path((_user_id, _account_id)): Path<(String, String)>) -> impl IntoResponse {
    // ...
}

async fn update_account(
    Path((_user_id, _account_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // ...
}

async fn delete_account(
    Path((_user_id, _account_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // ...
}
