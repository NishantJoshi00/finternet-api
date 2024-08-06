use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub ua_addr: String,
    pub public_key: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct CreateUserRequest {
    #[serde(flatten)]
    pub user: User,
}

#[derive(Serialize, Debug, Clone)]
pub(super) struct GetUserResponse {
    #[serde(flatten)]
    pub user: User,
}

#[derive(Serialize, Debug, Clone)]
pub(super) struct CreateUserResponse {
    pub user_id: String,
    pub ua_addr: String,
}
