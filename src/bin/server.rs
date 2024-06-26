use error_stack::ResultExt;
use finternet_app_api::error::{ConfigurationError, SResult};
use finternet_app_api::{logging, service_name};
use tokio::net::TcpListener;

use finternet_app_api::logging::prelude::*;

#[tokio::main]
async fn main() -> SResult<(), ConfigurationError> {
    let config = finternet_app_api::config::Config::new()?;

    let _guard = logging::setup(&config.log, [service_name!(), "tower_http"]);

    info!("Config: {:#?}", config);

    let router = finternet_app_api::app::router::<()>()?;

    finternet_app_api::app::start_server(
        router,
        TcpListener::bind((config.server_config.host, config.server_config.port))
            .await
            .change_context(ConfigurationError::ServerBindError)?,
    )
    .await?;

    Ok(())
}
