use dyn_clone::DynClone;

use crate::error::{SResult, StorageError};

use self::types::{Account, TokenManager, TokenManagerInfo, TotalAssets, User};

pub mod types;

#[async_trait::async_trait]
pub trait StorageInterface: DynClone {
    async fn get_user_interface(
        &self,
    ) -> SResult<Box<dyn UserInterface + Send + Sync>, StorageError>;
    async fn get_token_manager_interface(
        &self,
    ) -> SResult<Box<dyn TokenManagerInterface + Send + Sync>, StorageError>;
}

#[async_trait::async_trait]
pub trait UserInterface {
    // User -> user_id
    async fn create_user(&self, user: User) -> SResult<String, StorageError>;

    async fn get_user(&self, user_id: &str) -> SResult<User, StorageError>;

    async fn get_account_interface(
        &self,
        user_id: &str,
    ) -> SResult<Box<dyn AccountInterface + Send + Sync>, StorageError>;

    async fn is_valid_ua_addr(&self, ua_addr: &str) -> SResult<bool, StorageError>;
}

#[async_trait::async_trait]
pub trait TokenManagerInterface {
    // TokenManager -> token_manager_id
    async fn create_token_manager(
        &self,
        token_manager: TokenManager,
    ) -> SResult<String, StorageError>;

    async fn get_token_manager(
        &self,
        token_manager_id: &str,
    ) -> SResult<TokenManager, StorageError>;

    async fn list_token_manager(&self) -> SResult<Vec<TokenManagerInfo>, StorageError>;

    async fn get_supported_asset_interface(
        &self,
        token_manager_id: &str,
    ) -> SResult<Box<dyn SupportedAssetInterface + Send + Sync>, StorageError>;
}

#[async_trait::async_trait]
pub trait SupportedAssetInterface {
    async fn create_supported_asset(
        &self,
        token_manager_id: &str,
        asset: types::SupportedAsset,
    ) -> SResult<String, StorageError>;

    async fn get_supported_asset(
        &self,
        supported_asset_id: &str,
    ) -> SResult<types::SupportedAsset, StorageError>;

    async fn list_supported_assets(&self) -> SResult<Vec<types::SupportedAsset>, StorageError>;
}

#[async_trait::async_trait]
pub trait AccountInterface {
    async fn create_account(&self, acc: Account) -> SResult<String, StorageError>;

    async fn get_account(&self, account_id: &str) -> SResult<(Account, TotalAssets), StorageError>;

    async fn get_asset_interface(
        &self,
        account_id: &str,
    ) -> SResult<Box<dyn AssetInterface + Send + Sync>, StorageError>;
}

#[async_trait::async_trait]
pub trait AssetInterface {
    async fn list_assets(&self) -> SResult<Vec<types::AssetInfo>, StorageError>;
}

dyn_clone::clone_trait_object!(StorageInterface);
