use serde::Deserialize;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

pub mod prelude {
    pub use tracing::{debug, error, info, trace, warn};
}

pub fn setup(log_config: &LogConfig, crates_to_filter: impl AsRef<[&'static str]>) -> Guards {
    let subscriber = tracing_subscriber::registry();

    let console_filter = get_envfilter(
        log_config.filtering_directive.as_ref(),
        LogLevel::Warn,
        log_config.log_level,
        &crates_to_filter,
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    match log_config.log_format {
        LogFormat::Console => {
            let logging_layer = fmt::layer()
                .with_timer(fmt::time::time())
                .pretty()
                .with_writer(non_blocking)
                .with_filter(console_filter);

            subscriber.with(logging_layer).init();
        }
        LogFormat::Json => {
            let logging_layer = fmt::layer()
                .json()
                .with_timer(fmt::time::time())
                .with_writer(non_blocking)
                .with_filter(console_filter);

            subscriber.with(logging_layer).init();
        }
    }

    Guards { _log_guard: guard }
}

pub struct Guards {
    _log_guard: WorkerGuard,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct LogConfig {
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub filtering_directive: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Console,
    Json,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
            LogLevel::Off => Self::OFF,
        }
    }
}

macro_rules! cargo_workspace_members {
    () => {
        env!("CARGO_WORKSPACE_MEMBERS")
            .split(',')
            .collect::<std::collections::HashSet<&'static str>>()
    };
}

#[macro_export]
macro_rules! service_name {
    () => {
        env!("CARGO_BIN_NAME")
    };
}

fn get_envfilter(
    filtering_directive: Option<&String>,
    default_log_level: impl Into<LevelFilter> + Copy,
    filter_log_level: impl Into<LevelFilter> + Copy,
    crates_to_filter: impl AsRef<[&'static str]>,
) -> EnvFilter {
    filtering_directive
        .map(|filter| {
            // Try to create target filter from specified filtering directive, if set

            // Safety: If user is overriding the default filtering directive, then we need to panic
            // for invalid directives.
            #[allow(clippy::expect_used)]
            EnvFilter::builder()
                .with_default_directive(default_log_level.into().into())
                .parse(filter)
                .expect("Invalid EnvFilter filtering directive")
        })
        .unwrap_or_else(|| {
            // Construct a default target filter otherwise
            let mut workspace_members = cargo_workspace_members!();
            workspace_members.extend(crates_to_filter.as_ref());

            workspace_members
                .drain()
                .zip(std::iter::repeat(filter_log_level.into()))
                .fold(
                    EnvFilter::default().add_directive(default_log_level.into().into()),
                    |env_filter, (target, level)| {
                        // Safety: This is a hardcoded basic filtering directive. If even the basic
                        // filter is wrong, it's better to panic.
                        #[allow(clippy::expect_used)]
                        env_filter.add_directive(
                            format!("{target}={level}")
                                .parse()
                                .expect("Invalid EnvFilter directive format"),
                        )
                    },
                )
        })
}
