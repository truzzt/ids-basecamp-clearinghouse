/// Represents the configuration for the application
#[derive(Debug, serde::Deserialize)]
pub(crate) struct CHConfig {
    pub(crate) database_url: String,
    pub(crate) clear_db: bool,
    #[serde(default)]
    pub(crate) log_level: Option<LogLevel>,
    #[serde(default)]
    pub(crate) signing_key: Option<String>,
    performance_tracing: Option<bool>,
}

/// Contains the log level for the application
#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Trace => String::from("TRACE"),
            LogLevel::Debug => String::from("DEBUG"),
            LogLevel::Info => String::from("INFO"),
            LogLevel::Warn => String::from("WARN"),
            LogLevel::Error => String::from("ERROR"),
        }
    }
}

/// Read configuration from `config.toml` and environment variables. `config_file_override` can be
/// used to override the default config file, mainly for testing purposes.
pub(crate) fn read_config(config_file_override: Option<&std::path::Path>) -> CHConfig {
    // Create config builder
    let mut conf_builder = config::Config::builder();

    // Override config file override path
    conf_builder = if let Some(config_file) = config_file_override {
        conf_builder.add_source(config::File::from(config_file))
    } else {
        conf_builder.add_source(config::File::with_name("config.toml"))
    };

    // Add environment variables and finish
    conf_builder =
        conf_builder.add_source(config::Environment::with_prefix("CH_APP").prefix_separator("_"));

    // Finalize and deserialize
    conf_builder
        .build()
        .expect("Failure to read configuration! Exiting...")
        .try_deserialize::<CHConfig>()
        .expect("Failure to parse configuration! Exiting...")
}

/// Configure logging based on environment variable `RUST_LOG`
pub(crate) fn configure_logging(config: &CHConfig) {
    if std::env::var("RUST_LOG").is_err() {
        if let Some(level) = &config.log_level {
            std::env::set_var("RUST_LOG", level.to_string());
        }
    }

    // setup logging
    let mut subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());

    // Add performance tracing
    if let Some(true) = config.performance_tracing {
        subscriber_builder =
            subscriber_builder.with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE);
    }

    subscriber_builder.init();
}

#[cfg(test)]
mod test {
    use serial_test::serial;

    /// Test reading config from environment variables
    #[test]
    #[serial]
    fn test_read_config_from_env() {
        std::env::set_var("CH_APP_DATABASE_URL", "mongodb://localhost:27117");
        std::env::set_var("CH_APP_CLEAR_DB", "true");
        std::env::set_var("CH_APP_LOG_LEVEL", "INFO");

        let conf = super::read_config(None);
        assert_eq!(conf.database_url, "mongodb://localhost:27117");
        assert!(conf.clear_db);
        assert_eq!(conf.log_level, Some(super::LogLevel::Info));

        // Cleanup
        std::env::remove_var("CH_APP_DATABASE_URL");
        std::env::remove_var("CH_APP_CLEAR_DB");
        std::env::remove_var("CH_APP_LOG_LEVEL");
    }

    /// Test reading config from toml file
    #[test]
    #[serial]
    fn test_read_config_from_toml() {
        // Create tempfile
        let file = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();

        // Write config to file
        let toml = r#"database_url = "mongodb://localhost:27019"
clear_db = true
log_level = "ERROR"
"#;

        // Write to file
        std::fs::write(file.path(), toml).expect("Failure to write config file!");

        // Read config
        let conf = super::read_config(Some(file.path()));

        // Test
        assert_eq!(conf.database_url, "mongodb://localhost:27019");
        assert!(conf.clear_db);
        assert_eq!(conf.log_level, Some(super::LogLevel::Error));
    }
}
