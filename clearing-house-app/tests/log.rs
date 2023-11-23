#![cfg(test)]

use axum::http::{Request, StatusCode};
use biscuit::jwa::SignatureAlgorithm::PS512;
use biscuit::jwk::JWKSet;
use hyper::Body;
use clearing_house_app::model::claims::{ChClaims, get_fingerprint};
use clearing_house_app::model::ids::message::IdsMessage;
use clearing_house_app::model::ids::request::ClearingHouseMessage;
use clearing_house_app::model::ids::{IdsQueryResult, InfoModelId, MessageType};
use clearing_house_app::model::{claims::create_token, constants::SERVICE_HEADER};
use clearing_house_app::util::new_uuid;
use tower::ServiceExt;
use clearing_house_app::model::process::Receipt;

#[tokio::test]
#[ignore]
async fn log_message() {
    std::env::set_var("SERVICE_ID_LOG", "test");
    std::env::set_var("SHARED_SECRET", "test");
    std::env::set_var("CH_APP_LOG_LEVEL", "TRACE");
    std::env::set_var("CH_APP_CLEAR_DB", "false");

    let app = clearing_house_app::app().await.unwrap();

    // Prerequisite JWKS for checking the signature
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/.well-known/jwks.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(!body.is_empty());
    let jwks = serde_json::from_slice::<JWKSet<biscuit::Empty>>(&body).expect("Decoded the JWKSet");

    // ---------------------------------------------------------------------------------------------
    // Create a message
    let pid = new_uuid();
    let id = new_uuid();

    let msg = ClearingHouseMessage {
        header: IdsMessage {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::Message,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: "https://w3id.org/idsa/core/ClearingHouse".to_string(),
            ..Default::default()
        },
        payload: Some("test".to_string()),
        payload_type: None,
    };

    let claims = ChClaims::new("test");

    // Send log message
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/log/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(SERVICE_HEADER, create_token("test", "test", &claims))
                .body(serde_json::to_string(&msg).unwrap().into())
                .unwrap(),
        )
        .await
        .unwrap();

    // Check status code
    assert_eq!(response.status(), StatusCode::CREATED);
    // get body
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(!body.is_empty());

    // Decode receipt
    let receipt = serde_json::from_slice::<Receipt>(&body).unwrap();
    println!("Receipt: {:?}", receipt);
    let decoded_receipt = receipt.data
        .decode_with_jwks(&jwks, Some(PS512))
        .expect("Decoding JWS successful");
    let decoded_receipt_header = decoded_receipt
        .header()
        .expect("Header is now already decoded");

    assert_eq!(decoded_receipt_header.registered.key_id, get_fingerprint("keys/private_key.der"));

    let decoded_receipt_payload = decoded_receipt
        .payload()
        .expect("Payload is now already decoded");
    println!("Decoded Receipt: {:?}", decoded_receipt);

    assert_eq!(decoded_receipt_payload.process_id, pid);
    assert_eq!(decoded_receipt_payload.payload, "test".to_string());

    // ---------------------------------------------------------------------------------------------

    // Query ID
    let query_resp = app
        .oneshot(
            Request::builder()
                .uri(format!("/messages/query/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(SERVICE_HEADER, create_token("test", "test", &claims))
                .body(serde_json::to_string(&msg).unwrap().into())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(query_resp.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(query_resp.into_body()).await.unwrap();
    assert!(!body.is_empty());

    let ids_message = serde_json::from_slice::<IdsQueryResult>(&body).unwrap();
    println!("IDS Query Result: {:?}", ids_message);
    let query_docs = ids_message.documents;

    // Check the only document in the result
    assert_eq!(query_docs.len(), 1);
    let doc = query_docs.first().expect("Document is there, just checked").to_owned();
    assert_eq!(doc.payload.expect("Payload is there"), "test".to_string());
    assert_eq!(doc.model_version, "test".to_string());
}
