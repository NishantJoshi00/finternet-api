use axum::response::IntoResponse;

use crate::logging::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    #[error("Error while starting the server")]
    ServerStartError,

    #[error("Error while obtaining the local address")]
    LocalAddressError,

    #[error("Error while configuring the server")]
    ConfigError,

    #[error("Error while overriding the configuration")]
    OverrideError,

    #[error("Error while parsing the configuration")]
    ParseError,

    #[error("Error while binding the server")]
    ServerBindError,

    #[error("Failed while setting up presets")]
    PresetError,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiClientError {
    #[error("Error while sending the request")]
    RequestError,

    #[error("Non-success status code")]
    NonSuccessStatusError,

    #[error("Failed while deserializing the response")]
    DeserializationError,
}

pub type SResult<T, E> = error_stack::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Error while getting the account interface")]
    AccountInterfaceError,

    #[error("Error while getting the supported asset interface")]
    SupportedAssetInterfaceError,

    #[error("Error while getting the asset interface")]
    AssetInterfaceError,

    #[error("User not found")]
    UserNotFoundError,

    #[error("Account not found")]
    AccountNotFoundError,

    #[error("Token manager not found")]
    TokenManagerNotFoundError,

    #[error("Failed while deserializing the user data backup")]
    UserDeserializationError,

    #[error("Unique Address already exists")]
    UaAddrExistsError,

    #[error("Supported asset not found")]
    SupportedAssetNotFoundError,

    #[error("Asset not found")]
    AssetNotFoundError,
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum ApiError {
    #[error("Error while creating the user")]
    CreateUserError,
    #[error("Get user error")]
    GetUserError,
    #[error("Error while creating a token manager")]
    CreateTokenManagerError,

    #[error("Failed while creating supported asset")]
    CreateSupportedAssetError,

    #[error("Error while listing the token managers")]
    ListTokenManagersError,
    #[error("Account Creation Error")]
    AccountCreationError,
    #[error("Failed while fetching the account")]
    FetchAccountError,
    #[error("not implemented")]
    NotImplemented,
    #[error("Failed while creating assets")]
    CreateAssetError,
    #[error("Failed to act on the asset")]
    ActionAssetError,

    #[error("Failure while deleting the asset")]
    DeleteAssetError,

    #[error("Asset type not supported by the token manager")]
    AssetTypeNotSupportedError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::CreateUserError => {
                axum::response::Json("Error while creating the user").into_response()
            }
            ApiError::GetUserError => axum::response::Json("Get user error").into_response(),
            ApiError::CreateTokenManagerError => {
                axum::response::Json("Error while creating a token manager").into_response()
            }
            ApiError::ListTokenManagersError => {
                axum::response::Json("Error while listing the token managers").into_response()
            }
            ApiError::NotImplemented => (
                axum::http::StatusCode::NOT_IMPLEMENTED,
                axum::response::Json("not implemented"),
            )
                .into_response(),
            ApiError::AccountCreationError => {
                axum::response::Json("Account Creation Error").into_response()
            }
            ApiError::AssetTypeNotSupportedError => {
                axum::response::Json("Asset type not supported by the token manager")
                    .into_response()
            }
            ApiError::CreateAssetError => {
                axum::response::Json("Failed while creating assets").into_response()
            }
            ApiError::ActionAssetError => {
                axum::response::Json("Failed to act on the asset").into_response()
            }
            ApiError::CreateSupportedAssetError => {
                axum::response::Json("Failed while creating supported asset").into_response()
            }
            ApiError::FetchAccountError => {
                axum::response::Json("Failed while fetching the account").into_response()
            }
            ApiError::DeleteAssetError => {
                axum::response::Json("Failure while deleting the asset").into_response()
            }
        }
    }
}

#[track_caller]
pub fn log_convert(e: error_stack::Report<ApiError>) -> ApiError {
    error!(?e);

    *e.current_context()
}
