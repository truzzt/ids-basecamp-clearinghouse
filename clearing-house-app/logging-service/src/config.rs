#[derive(Debug, serde::Deserialize)]
pub(crate) struct CHConfig {
    pub(crate) process_database_url: String,
    pub(crate) keyring_database_url: String,
    pub(crate) document_database_url: String,
    pub(crate) clear_db: bool,
    #[serde(default)]
    pub(crate) log_level: Option<LogLevel>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Into<tracing::Level> for LogLevel {
    fn into(self) -> tracing::Level {
        match self {
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

/// Read configuration from `config.toml` and environment variables
pub(crate) fn read_config() -> CHConfig {
    let conf = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::with_prefix("CH_APP_"))
        .build()
        .expect("Failure to read configuration! Exiting...");

    conf.try_deserialize::<CHConfig>()
        .expect("Failure to parse configuration! Exiting...")
}

/// Configure logging based on environment variable `RUST_LOG`
pub(crate) fn configure_logging(log_level: Option<LogLevel>) {
    if std::env::var("RUST_LOG").is_err() {
        if let Some(level) = log_level {
            std::env::set_var("RUST_LOG", level.to_string());
        }
    }

    // setup logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}