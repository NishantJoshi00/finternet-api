use dyn_clone::DynClone;

use crate::error::{SResult, StorageError};

pub mod types;

#[async_trait::async_trait]
pub trait StorageInterface: DynClone {
    async fn get_user_interface(&self) -> SResult<Box<dyn UserInterface>, StorageError>;
    async fn get_token_manager_interface(
        &self,
    ) -> SResult<Box<dyn TokenManagerInterface>, StorageError>;
}

#[async_trait::async_trait]
pub trait UserInterface {
    async fn get_account_interface(
        &self,
        user_id: &str,
    ) -> SResult<Box<dyn AccountInterface>, StorageError>;

    async fn is_valid_ua_addr(&self, ua_addr: &str) -> SResult<bool, StorageError>;
}

#[async_trait::async_trait]
pub trait TokenManagerInterface {
    async fn get_supported_asset_interface(
        &self,
        token_manager_id: &str,
    ) -> SResult<Box<dyn SupportedAssetInterface>, StorageError>;
}

#[async_trait::async_trait]
pub trait SupportedAssetInterface {}

#[async_trait::async_trait]
pub trait AccountInterface {
    async fn get_asset_interface(
        &self,
        account_id: &str,
    ) -> SResult<Box<dyn AssetInterface>, StorageError>;
}

#[async_trait::async_trait]
pub trait AssetInterface {}

dyn_clone::clone_trait_object!(StorageInterface);
