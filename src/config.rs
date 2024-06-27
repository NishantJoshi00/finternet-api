use crate::error::{ConfigurationError, SResult};
use crate::logging::LogConfig;
use config::{Environment, File};
use error_stack::ResultExt;
use serde::Deserialize;
use std::path::PathBuf;

use crate::logging::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    #[serde(rename(deserialize = "server"))]
    pub server_config: ServerSettings,
    // #[cfg(feature = "aws-kms")]
    // pub aws_kms: kms::AwsKmsConfig,
    pub imc_backup: BackupConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BackupConfig {
    pub path: PathBuf,
}

#[derive(Debug, strum::Display, strum::EnumString, Clone, Copy)]
pub enum Env {
    Development,
    Sandbox,
    Production,
}

impl Env {
    fn config_path(self) -> PathBuf {
        match self {
            Self::Development => PathBuf::from("config/development.toml"),
            Self::Sandbox => PathBuf::from("config/sandbox.toml"),
            Self::Production => PathBuf::from("config/production.toml"),
        }
    }
}

mod vars {
    pub const RUN_ENV: &str = "RUN_ENV";
}

impl Config {
    pub fn new() -> SResult<Self, ConfigurationError> {
        let env = if cfg!(not(debug_assertions)) {
            Env::Production
        } else {
            Env::Development
        };

        let env = std::env::var(vars::RUN_ENV)
            .map(|val| val.parse().unwrap_or(env))
            .unwrap_or(env);

        let config_builder = config::Config::builder()
            .set_override("env", env.to_string())
            .change_context(ConfigurationError::OverrideError)?;

        let config = config_builder
            .add_source(File::from(env.config_path()).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()
            .change_context(ConfigurationError::ConfigError)?;

        Ok(serde_path_to_error::deserialize(config)
            .map_err(|error| {
                error!("Unable to deserialize application configuration: {error}");
                error.into_inner()
            })
            .change_context(ConfigurationError::ParseError)?)
    }
}
