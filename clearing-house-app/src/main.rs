#![forbid(unsafe_code)]
#![warn(clippy::unwrap_used)]

use std::net::SocketAddr;

/// Main function: Reading config, initializing application state, starting server
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    #[cfg(feature = "sentry")]
    let _guard = sentry::init(("https://347cc3aa30aa0c07d437da8c780838d3@o4506146399322112.ingest.sentry.io/4506155710480384", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    // Setup router
    let app = clearing_house_app::app().await?;

    // Bind port and start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Starting server: Listening on {}", addr);
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(clearing_house_app::util::shutdown_signal())
        .await?)
}
