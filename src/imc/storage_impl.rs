use error_stack::{ensure, report};

use nanoid::nanoid;

use crate::error::{SResult, StorageError};
use crate::imc::User;
use crate::storage::types::{AssetInfo, TotalAssets};
use crate::storage::{
    AccountInterface, AssetInterface, StorageInterface, SupportedAssetInterface,
    TokenManagerInterface, UserInterface,
};

use super::{
    AccountStore, Asset, AssetStore, Storage, SupportedAssetStore, TokenManagerStore, UserStore,
};

#[async_trait::async_trait]
impl StorageInterface for Storage {
    async fn get_user_interface(
        &self,
    ) -> SResult<Box<dyn UserInterface + Send + Sync>, StorageError> {
        Ok(Box::new(self.users.clone()))
    }

    async fn get_token_manager_interface(
        &self,
    ) -> SResult<Box<dyn TokenManagerInterface + Send + Sync>, StorageError> {
        Ok(Box::new(self.token_managers.clone()))
    }
}

#[async_trait::async_trait]
impl UserInterface for UserStore {
    async fn create_user(
        &self,
        user: crate::storage::types::User,
    ) -> SResult<String, StorageError> {
        let user_id = nanoid!(5);

        let exist = self.is_valid_ua_addr(&user.ua_addr).await?;

        ensure!(!exist, StorageError::UaAddrExistsError);

        self.set.write().await.insert(user.ua_addr.clone());

        let new_user = User {
            accounts: AccountStore::new(),
            ua_addr: user.ua_addr,
            email: user.email,
            id: user_id.clone(),
            name: user.name,
            public_key: user.public_key,
        };

        self.map.write().await.insert(user_id.clone(), new_user);

        Ok(user_id)
    }

    async fn get_user(&self, user_id: &str) -> SResult<crate::storage::types::User, StorageError> {
        let store = self.map.read().await;

        let user = store
            .get(user_id)
            .ok_or(report!(StorageError::UserNotFoundError))?;

        Ok(crate::storage::types::User {
            ua_addr: user.ua_addr.clone(),
            email: user.email.clone(),
            name: user.name.clone(),
            public_key: user.public_key.clone(),
        })
    }

    async fn get_account_interface(
        &self,
        user_id: &str,
    ) -> SResult<Box<dyn AccountInterface + Send + Sync>, StorageError> {
        let store = self.map.read().await;

        let user = store
            .get(user_id)
            .ok_or(report!(StorageError::UserNotFoundError))?;

        Ok(Box::new(user.accounts.clone()))
    }

    async fn get_account_interface_by_ua(
        &self,
        ua_addr: &str,
    ) -> SResult<Box<dyn AccountInterface + Send + Sync>, StorageError> {
        let store = self.map.read().await;

        let user = store
            .values()
            .find(|user| user.ua_addr == ua_addr)
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
    async fn create_account(
        &self,
        acc: crate::storage::types::Account,
    ) -> SResult<String, StorageError> {
        let account_id = nanoid!(5);

        let new_account = super::Account {
            id: account_id.clone(),
            assets: AssetStore::new(),
            account_name: acc.account_name,
            token_manager_id: acc.token_manager_id,
            token_manager_ref: acc.token_manager_ref,
            asset_type: acc.asset_type,
        };

        self.map
            .write()
            .await
            .insert(account_id.clone(), new_account);

        Ok(account_id)
    }

    async fn get_account(
        &self,
        account_id: &str,
    ) -> SResult<
        (
            crate::storage::types::Account,
            crate::storage::types::TotalAssets,
        ),
        StorageError,
    > {
        let store = self.map.read().await;

        let account = store
            .get(account_id)
            .ok_or(report!(StorageError::AccountNotFoundError))?;

        let all_assets: Vec<AssetInfo> = self
            .get_asset_interface(account_id)
            .await?
            .list_assets()
            .await?;

        let output = all_assets
            .into_iter()
            .fold(TotalAssets::new(), |mut acc, cur| match cur {
                AssetInfo::Cash { amount, currency } => {
                    acc.money.amount += amount;
                    acc.money.currency = currency;
                    acc
                }
            });

        Ok((
            crate::storage::types::Account {
                account_name: account.account_name.clone(),
                token_manager_id: account.token_manager_id.clone(),
                token_manager_ref: account.token_manager_ref.clone(),
                asset_type: account.asset_type.clone(),
            },
            output,
        ))
    }

    async fn get_asset_interface(
        &self,
        account_id: &str,
    ) -> SResult<Box<dyn AssetInterface + Send + Sync>, StorageError> {
        let store = self.map.read().await;

        let account = store
            .get(account_id)
            .ok_or(report!(StorageError::AccountNotFoundError))?;

        Ok(Box::new(account.assets.clone()))
    }
}

#[async_trait::async_trait]
impl AssetInterface for AssetStore {
    async fn list_assets(&self) -> SResult<Vec<AssetInfo>, StorageError> {
        let output = self.map.read().await;

        Ok(output
            .values()
            .map(|value| &value.asset_info)
            .cloned()
            .collect())
    }

    async fn create_asset(&self, asset: AssetInfo) -> SResult<String, StorageError> {
        let asset_id = nanoid!(5);

        let new_asset = Asset {
            id: asset_id.clone(),
            asset_info: asset,
        };

        self.map.write().await.insert(asset_id.clone(), new_asset);

        Ok(asset_id)
    }

    async fn delete_asset(&self, asset_id: &str) -> SResult<AssetInfo, StorageError> {
        let mut store = self.map.write().await;

        let asset = store
            .remove(asset_id)
            .ok_or(report!(StorageError::AssetNotFoundError))?;

        Ok(asset.asset_info)
    }
}

#[async_trait::async_trait]
impl TokenManagerInterface for TokenManagerStore {
    async fn create_token_manager(
        &self,
        token_manager: crate::storage::types::TokenManager,
    ) -> SResult<String, StorageError> {
        let token_manager_id = nanoid!(5);

        let new_token_manager = super::TokenManager {
            id: token_manager_id.clone(),
            public_key: token_manager.public_key,
            supported_assets: SupportedAssetStore::new(),
            token_manager_name: token_manager.token_manager_name,
        };

        self.map
            .write()
            .await
            .insert(token_manager_id.clone(), new_token_manager);

        Ok(token_manager_id)
    }

    async fn get_token_manager(
        &self,
        token_manager_id: &str,
    ) -> SResult<crate::storage::types::TokenManager, StorageError> {
        let store = self.map.read().await;

        let token_manager = store
            .get(token_manager_id)
            .ok_or(report!(StorageError::TokenManagerNotFoundError))?;

        Ok(crate::storage::types::TokenManager {
            public_key: token_manager.public_key.clone(),
            token_manager_name: token_manager.token_manager_name.clone(),
        })
    }

    async fn list_token_manager(
        &self,
    ) -> SResult<Vec<crate::storage::types::TokenManagerInfo>, StorageError> {
        let store = self.map.read().await;

        Ok(store
            .values()
            .map(|token_manager| crate::storage::types::TokenManagerInfo {
                token_manager_id: token_manager.public_key.clone(),
                token_manager_name: token_manager.token_manager_name.clone(),
            })
            .collect())
    }

    async fn get_supported_asset_interface(
        &self,
        token_manager_id: &str,
    ) -> SResult<Box<dyn SupportedAssetInterface + Send + Sync>, StorageError> {
        let store = self.map.read().await;

        let token_manager = store
            .get(token_manager_id)
            .ok_or(report!(StorageError::TokenManagerNotFoundError))?;

        Ok(Box::new(token_manager.supported_assets.clone()))
    }
}

#[async_trait::async_trait]
impl SupportedAssetInterface for SupportedAssetStore {
    async fn create_supported_asset(
        &self,
        asset: crate::storage::types::SupportedAsset,
    ) -> SResult<String, StorageError> {
        let supported_asset_id = nanoid!(5);

        let new_supported_asset = super::SupportedAsset {
            asset_type: asset.asset_type,
            id: supported_asset_id.clone(),
            smart_contract_refs: asset.smart_contract_refs,
        };

        self.map
            .write()
            .await
            .insert(supported_asset_id.clone(), new_supported_asset);

        Ok(supported_asset_id)
    }

    async fn get_supported_asset(
        &self,
        supported_asset_id: &str,
    ) -> SResult<crate::storage::types::SupportedAsset, StorageError> {
        let store = self.map.read().await;

        let supported_asset = store
            .get(supported_asset_id)
            .ok_or(report!(StorageError::SupportedAssetNotFoundError))?;

        Ok(crate::storage::types::SupportedAsset {
            asset_type: supported_asset.asset_type.clone(),
            smart_contract_refs: supported_asset.smart_contract_refs.clone(),
        })
    }

    async fn list_supported_assets(
        &self,
    ) -> SResult<Vec<crate::storage::types::SupportedAsset>, StorageError> {
        let store = self.map.read().await;

        Ok(store
            .values()
            .map(|supported_asset| crate::storage::types::SupportedAsset {
                asset_type: supported_asset.asset_type.clone(),
                smart_contract_refs: supported_asset.smart_contract_refs.clone(),
            })
            .collect())
    }
}
