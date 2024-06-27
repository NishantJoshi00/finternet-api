use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;

use crate::error::ConfigurationError;

mod accounts;
mod types;

pub fn router<S: Send + Sync + Clone + 'static>() -> Result<axum::Router<S>, ConfigurationError> {
    let router = axum::Router::new()
        .route("/", post(create_user))
        .route(
            "/:user_id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .nest("/:user_id/accounts", accounts::router::<S>()?);

    Ok(router)
}

async fn create_user(Json(user): Json<types::CreateUserRequest>) -> impl IntoResponse {
    // in: email, name, public_key, ua_addr 
    // out: 


}

async fn get_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn update_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    // ...
}

async fn delete_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    // ...
}
