#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    #[error("Error while starting the server")]
    ServerStartError,

    #[error("Error while obtaining the local address")]
    LocalAddressError,

    #[error("Error while configuring the server")]
    ConfigError,

    #[error("Error while overriding the configuration")]
    OverrideError,

    #[error("Error while parsing the configuration")]
    ParseError,

    #[error("Error while binding the server")]
    ServerBindError,
}

pub type SResult<T, E> = error_stack::Result<T, E>;
