use axum::body::Body;
use axum::http::{Request, StatusCode};
use biscuit::jwk::JWKSet;
use tower::ServiceExt;

#[tokio::test]
#[ignore]
async fn retrieve_public_key() {
    std::env::set_var("SERVICE_ID_LOG", "test");

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
