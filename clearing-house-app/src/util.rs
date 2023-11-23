use anyhow::Context;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceConfig {
    pub service_id: String,
}

/// Reads a file into a string
pub(crate) fn read_file(file: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(file)
        .with_context(|| format!("Failed to read contents of file '{}'", file))
}

pub(super) fn init_service_config(service_id: String) -> anyhow::Result<ServiceConfig> {
    match std::env::var(&service_id) {
        Ok(id) => Ok(ServiceConfig { service_id: id }),
        Err(_e) => {
            anyhow::bail!(
                "Service ID not configured. Please configure environment variable {}",
                &service_id
            );
        }
    }
}

pub(super) fn init_signing_key(signing_key_path: Option<&str>) -> anyhow::Result<String> {
    let private_key_path = signing_key_path.unwrap_or("keys/private_key.der");
    if std::path::Path::new(&private_key_path).exists() {
        Ok(private_key_path.to_string())
    } else {
        anyhow::bail!("Signing key not found! Aborting startup! Please configure signing_key!");
    }
}

/// Signal handler to catch a Ctrl+C and initiate a graceful shutdown
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}

/// Returns a new UUID as a string with hyphens.
pub fn new_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().hyphenated().to_string()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_new_uuid() {
        let uuid = super::new_uuid();
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }
}
