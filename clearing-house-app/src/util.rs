/// Signal handler to catch a Ctrl+C and initiate a graceful shutdown
///
/// # Panics
///
/// May panic if the signal handler cannot be installed
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
        () = ctrl_c => {},
        () = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}

/// Returns a new UUID as a string with hyphens.
#[must_use]
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
