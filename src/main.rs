use error_stack::ResultExt;
use finternet_app_api::config::Config;
use finternet_app_api::error::{ConfigurationError, SResult};
use finternet_app_api::state::AppState;
use finternet_app_api::{logging, service_name};
use tokio::net::TcpListener;

use finternet_app_api::logging::prelude::*;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // let config = finternet_app_api::config::Config::new().unwrap();

    let config = Config {
        log: logging::LogConfig {
            log_level: logging::LogLevel::Info,
            log_format: logging::LogFormat::Json,
            filtering_directive: None,
        },
        server_config: finternet_app_api::config::ServerSettings {
            port: 8080,
            host: "0.0.0.0".to_string(),
        },
    };

    // let _guard = logging::setup(&config.log, [service_name!(), "tower_http"]);

    info!("Config: {:#?}", config);

    let router = finternet_app_api::app::router().unwrap();

    let router = router.with_state(AppState::imc_backed(config.clone()));

    let router = router.layer(
        tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::any())
            .allow_methods(tower_http::cors::any())
            .allow_headers(tower_http::cors::any()),
    );

    Ok(router.into())
}
