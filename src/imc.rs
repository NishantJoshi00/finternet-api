use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::config::BackupConfig;
use crate::error::{SResult, StorageError};
use crate::storage::types::{AssetInfo, AssetType, TokenManagerRef};

mod storage_impl;

#[derive(Clone)]
pub struct Storage {
    users: UserStore,
    token_managers: TokenManagerStore,
}

#[derive(Clone)]
pub struct UserStore {
    map: Arc<RwLock<HashMap<String, User>>>,
    set: Arc<RwLock<HashSet<String>>>,
}

#[derive(Clone)]
pub struct TokenManagerStore {
    map: Arc<RwLock<HashMap<String, TokenManager>>>,
}

pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub ua_addr: String,
    accounts: AccountStore,
}

pub struct TokenManager {
    pub id: String,
    pub token_manager_name: String,
    pub public_key: String,
    supported_assets: SupportedAssetStore,
}

#[derive(Clone)]
pub struct AccountStore {
    map: Arc<RwLock<HashMap<String, Account>>>,
}

pub struct Account {
    pub id: String,
    pub token_manager_id: String,
    pub public_key: String,
    pub account_name: String,
    pub token_manager_ref: TokenManagerRef,
    pub asset_type: AssetType,
    assets: AssetStore,
}

#[derive(Clone)]
pub struct SupportedAssetStore {
    map: Arc<RwLock<HashMap<String, SupportedAsset>>>,
}

pub struct SupportedAsset {
    pub id: String,
    pub asset_type: AssetType,
    pub smart_contract_refs: Vec<u8>,
}

#[derive(Clone)]
pub struct AssetStore {
    map: Arc<RwLock<HashMap<String, Asset>>>,
}

impl AssetStore {
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AssetStore {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Asset {
    pub id: String,
    pub asset_info: AssetInfo,
}

impl AccountStore {
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl SupportedAssetStore {
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Storage {
    pub fn new() -> Self {
        Self {
            users: UserStore {
                map: Arc::new(RwLock::new(HashMap::new())),
                set: Arc::new(RwLock::new(HashSet::new())),
            },
            token_managers: TokenManagerStore {
                map: Arc::new(RwLock::new(HashMap::new())),
            },
        }
    }
    pub fn setup_disk_backup(&self, _config: &BackupConfig) -> SResult<(), StorageError> {
        // let path = config.path;
        // let user_path = path.join("users.bin");
        // let token_manager_path = path.join("token_managers.bin");
        //
        todo!()
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
            set: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}

impl Default for TokenManagerStore {
    fn default() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AccountStore {
    fn default() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for SupportedAssetStore {
    fn default() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
