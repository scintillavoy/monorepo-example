use serde_with::DurationSeconds;
use std::{env, time::Duration};

use config::{Config, ConfigError, File};
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};
use tracing_subscriber::filter::LevelFilter;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub vault_address: String,
    pub vault_path: String,
    #[serde_as(as = "DisplayFromStr")]
    pub log_level: LevelFilter,
    pub otel_service_name: String,
    pub otel_endpoint: String,
    pub otel_trace_sampling_ratio: f64,
    pub custom_attribute: String,
    pub sentry_dsn: String,
    pub mysql_host: String,
    pub mysql_port: u16,
    pub mysql_username: String,
    pub mysql_database: String,
    #[serde(rename = "request_timeout_seconds")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub request_timeout: Duration,
    pub swagger_enabled: bool,
}

impl ApiConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("CONFIG_PATH").unwrap();
        let config = Config::builder()
            .add_source(File::with_name(&config_path))
            .build()?;
        config.try_deserialize()
    }
}
