use axum::body::Body;
use axum::http::{Request, StatusCode};
use biscuit::jwk::JWKSet;
use testcontainers::runners::AsyncRunner;
use tower::ServiceExt;

#[tokio::test]
async fn retrieve_public_key() {
    let (_instance, connection_string) = {
        // Start testcontainer: Postgres
        let postgres_instance = testcontainers_modules::postgres::Postgres::default()
            .start()
            .await
            .expect("Failed to start Postgres container");
        let connection_string = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            postgres_instance
                .get_host()
                .await
                .expect("Failed to get host"),
            postgres_instance
                .get_host_port_ipv4(5432)
                .await
                .expect("Failed to get port")
        );

        (postgres_instance, connection_string)
    };

    #[allow(unsafe_code)] // Deprecated safe from rust edition 2024
    unsafe {
        std::env::set_var("SERVICE_ID_LOG", "test");
        std::env::set_var("CH_APP_DATABASE_URL", connection_string);
    }

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
