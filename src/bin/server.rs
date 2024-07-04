use error_stack::{FutureExt, ResultExt};
use finternet_app_api::error::{ConfigurationError, SResult};
use finternet_app_api::state::AppState;
use finternet_app_api::{logging, service_name};
use tokio::net::TcpListener;

use finternet_app_api::logging::prelude::*;

#[tokio::main]
async fn main() -> SResult<(), ConfigurationError> {
    let config = finternet_app_api::config::Config::new()?;

    let _guard = logging::setup(&config.log, [service_name!(), "tower_http"]);

    info!("Config: {:#?}", config);

    let router = finternet_app_api::app::router()?;

    let state = AppState::imc_backed(config.clone());

    let state = hardcode_state(state)
        .change_context(ConfigurationError::PresetError)
        .await?;

    let router = router.with_state(state);

    finternet_app_api::app::start_server(
        router,
        TcpListener::bind((config.server_config.host, config.server_config.port))
            .await
            .change_context(ConfigurationError::ServerBindError)?,
    )
    .await?;

    Ok(())
}

pub async fn hardcode_state(
    state: AppState,
) -> SResult<AppState, finternet_app_api::error::StorageError> {
    let user1 = state
        .storage
        .get_user_interface()
        .await?
        .create_user(finternet_app_api::storage::types::User {
            email: "nishant.joshi@juspay.in".into(),
            name: "Nishant Joshi".into(),
            ua_addr: "abc".into(),
        })
        .await?;

    let user2 = state
        .storage
        .get_user_interface()
        .await?
        .create_user(finternet_app_api::storage::types::User {
            email: "natarajan@juspay.in".into(),
            name: "Natarajan Kannan".into(),
            ua_addr: "xyz".into(),
        })
        .await?;

    let account1 = state
        .storage
        .get_user_interface()
        .await?
        .get_account_interface(&user1)
        .await?
        .create_account(finternet_app_api::storage::types::Account {
            account_name: "NJ's ICICI Account".into(),
            token_manager_id: "icici".into(),
            asset_type: finternet_app_api::storage::types::AssetType::Cash,
            public_key: "3zQGRCA9tSYP6ewrV6UV4B39DreT3w4kehsug8zXjttmD9Rg".into(),
            token_manager_ref: finternet_app_api::storage::types::TokenManagerRef {
                id: "xyz".into(),
                token_manager_name: "icici".into(),
                internal_addr: "3uPA63hYmrfN4K6nVbhrNLGQZQbFX1z3XmYvwgFXgkg2yYFz".into(),
            },
        })
        .await?;

    let account2 = state
        .storage
        .get_user_interface()
        .await?
        .get_account_interface(&user2)
        .await?
        .create_account(finternet_app_api::storage::types::Account {
            account_name: "NK's ICICI Account".into(),
            token_manager_id: "icici".into(),
            asset_type: finternet_app_api::storage::types::AssetType::Cash,
            public_key: "3ujvrN7Uw7Y9Zek9WwpvxLAyX8eqNbXfXGWjSJKSXY9TFLTw".into(),
            token_manager_ref: finternet_app_api::storage::types::TokenManagerRef {
                id: "xyz".into(),
                token_manager_name: "icici".into(),
                internal_addr: "3uPA63hYmrfN4K6nVbhrNLGQZQbFX1z3XmYvwgFXgkg2yYFz".into(),
            },
        })
        .await?;

    info!("user1: {}\taccount1: {}", user1, account1);
    info!("user2: {}\taccount2: {}", user2, account2);

    Ok(state)
}
