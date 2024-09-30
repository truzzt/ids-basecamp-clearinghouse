use axum::body::Body;
use axum::http::{Request, StatusCode};
use biscuit::jwk::JWKSet;
use tower::ServiceExt;

#[tokio::test]
async fn retrieve_public_key() {
    // Start testcontainer: Postgres
    let docker = testcontainers::clients::Cli::default();
    let postgres_instance = docker.run(testcontainers_modules::postgres::Postgres::default());
    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        postgres_instance.get_host_port_ipv4(5432)
    );

    std::env::set_var("SERVICE_ID_LOG", "test");
    std::env::set_var("CH_APP_DATABASE_URL", connection_string);

    let app = clearing_house_app::app().await.unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/.well-known/jwks.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!body.is_empty());
    let jwks = serde_json::from_slice::<JWKSet<biscuit::Empty>>(&body).expect("Decoded the JWKSet");
    println!("JWKS: {:?}", jwks);
}
