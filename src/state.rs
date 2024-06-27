use crate::storage::StorageInterface;

#[derive(Clone)]
pub struct AppState {
    pub config: crate::config::Config,
    pub storage: Box<dyn StorageInterface + Send + Sync>,
}

impl AppState {
    // pub fn new(config: crate::config::Config, storage: Box<dyn StorageInterface>) -> Self {
    //     Self { config, storage }
    // }

    pub fn imc_backed(config: crate::config::Config) -> Self {
        Self {
            config,
            storage: Box::new(crate::imc::Storage::new()),
        }
    }
}
