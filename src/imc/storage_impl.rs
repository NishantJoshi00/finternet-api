use error_stack::report;

use crate::error::{SResult, StorageError};
use crate::storage::{
    AccountInterface, AssetInterface, StorageInterface, SupportedAssetInterface,
    TokenManagerInterface, UserInterface,
};

use super::{AccountStore, AssetStore, Storage, SupportedAssetStore, TokenManagerStore, UserStore};

#[async_trait::async_trait]
impl StorageInterface for Storage {
    async fn get_user_interface(&self) -> SResult<Box<dyn UserInterface>, StorageError> {
        Ok(Box::new(self.users.clone()))
    }

    async fn get_token_manager_interface(
        &self,
    ) -> SResult<Box<dyn TokenManagerInterface>, StorageError> {
        Ok(Box::new(self.token_managers.clone()))
    }
}

#[async_trait::async_trait]
impl UserInterface for UserStore {
    async fn get_account_interface(
        &self,
        user_id: &str,
    ) -> SResult<Box<dyn AccountInterface>, StorageError> {
        let store = self.map.read().await;

        let user = store
            .get(user_id)
            .ok_or(report!(StorageError::UserNotFoundError))?;

        Ok(Box::new(user.accounts.clone()))
    }

    async fn is_valid_ua_addr(&self, ua_addr: &str) -> SResult<bool, StorageError> {
        let set = self.set.read().await;

        Ok(set.contains(ua_addr))
    }
}

#[async_trait::async_trait]
impl AccountInterface for AccountStore {
    async fn get_asset_interface(
        &self,
        account_id: &str,
    ) -> SResult<Box<dyn AssetInterface>, StorageError> {
        let store = self.map.read().await;

        let account = store
            .get(account_id)
            .ok_or(report!(StorageError::AccountNotFoundError))?;

        Ok(Box::new(account.assets.clone()))
    }
}

#[async_trait::async_trait]
impl AssetInterface for AssetStore {}

#[async_trait::async_trait]
impl TokenManagerInterface for TokenManagerStore {
    async fn get_supported_asset_interface(
        &self,
        token_manager_id: &str,
    ) -> SResult<Box<dyn SupportedAssetInterface>, StorageError> {
        let store = self.map.read().await;

        let token_manager = store
            .get(token_manager_id)
            .ok_or(report!(StorageError::TokenManagerNotFoundError))?;

        Ok(Box::new(token_manager.supported_assets.clone()))
    }
}

#[async_trait::async_trait]
impl SupportedAssetInterface for SupportedAssetStore {}
