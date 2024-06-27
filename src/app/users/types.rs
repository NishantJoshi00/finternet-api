use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub(super) struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub public_key: String,
    pub ua_addr: String,
}
