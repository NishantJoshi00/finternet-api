use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub(super) struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub public_key: String,
    pub ua_addr: String,
}

#[derive(Serialize, Debug, Clone)]
pub(super) struct GetUserResponse {
    ua_addr: String,
    pub email: String,
    pub name: String,
    // pub public_key: String,
}

#[derive(Serialize, Debug, Clone)]
pub(super) struct CreateUserResponse {
    pub user_id: String,
    pub ua_addr: String,
}

impl From<CreateUserRequest> for crate::storage::types::User {
    #[inline]
    fn from(value: CreateUserRequest) -> Self {
        crate::storage::types::User {
            email: value.email,
            name: value.name,
            public_key: value.public_key,
            ua_addr: value.ua_addr,
        }
    }
}

impl From<crate::storage::types::User> for GetUserResponse {
    fn from(value: crate::storage::types::User) -> Self {
        Self {
            ua_addr: value.ua_addr,
            email: value.email,
            name: value.name,
        }
    }
}
