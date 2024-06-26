use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};

use crate::error::ConfigurationError;

mod accounts;

pub fn router<S: Send + Sync + Clone + 'static>() -> Result<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_user))
        .route(
            "/:user_id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .nest("/accounts", accounts::router::<S>()?);

    Ok(router)
}

async fn create_user() -> impl IntoResponse {
    // ...
}

async fn get_user(Path(user_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn update_user(Path(user_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn delete_user(Path(user_id): Path<String>) -> impl IntoResponse {
    // ...
}
