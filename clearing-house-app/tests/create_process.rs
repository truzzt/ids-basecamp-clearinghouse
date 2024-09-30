#![cfg(test)]

use axum::http::{Request, StatusCode};
use biscuit::jwa::SignatureAlgorithm::PS512;
use biscuit::jwk::JWKSet;
use clearing_house_app::model::claims::{get_fingerprint, ChClaims};
use clearing_house_app::model::ids::message::IdsMessage;
use clearing_house_app::model::ids::request::ClearingHouseMessage;
use clearing_house_app::model::ids::{IdsQueryResult, InfoModelId, MessageType};
use clearing_house_app::model::process::{OwnerList, Receipt};
use clearing_house_app::model::{claims::create_token, constants::SERVICE_HEADER};
use clearing_house_app::util::new_uuid;
use testcontainers::runners::AsyncRunner;
use tower::ServiceExt;

#[tokio::test]
async fn log_message() {
    const CLIENT_ID: &str = "69:F5:9D:B0:DD:A6:9D:30:5F:58:AA:2D:20:4D:B2:39:F0:54:FC:3B:keyid:4F:66:7D:BD:08:EE:C6:4A:D1:96:D8:7C:6C:A2:32:8A:EC:A6:AD:49";

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
        std::env::set_var("SHARED_SECRET", "test");
        std::env::set_var("CH_APP_LOG_LEVEL", "TRACE");
        std::env::set_var("CH_APP_CLEAR_DB", "false");
        std::env::set_var("CH_APP_STATIC_PROCESS_OWNER", "MDS_EDC_CONNECTOR");
        std::env::set_var("CH_APP_DATABASE_URL", connection_string);
    }

    let app = clearing_house_app::app().await.unwrap();

    // Prerequisite JWKS for checking the signature
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/.well-known/jwks.json")
                .body(axum::body::Body::empty())
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

    // ---------------------------------------------------------------------------------------------

    // Create a process
    let pid = new_uuid();
    let id = new_uuid();

    let process_owners = OwnerList {
        owners: vec![CLIENT_ID.to_string()],
    };
    let process_owners_payload = serde_json::to_string(&process_owners).expect("Should serialize");

    let msg = ClearingHouseMessage {
        header: IdsMessage {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::RequestMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: "https://w3id.org/idsa/core/ClearingHouse".to_string(),
            ..Default::default()
        },
        payload: Some(process_owners_payload),
        payload_type: None,
    };

    let claims = ChClaims::new(CLIENT_ID);

    // Send create process message
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/process/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(SERVICE_HEADER, create_token("test", "test", &claims))
                .body(serde_json::to_string(&msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();

    // Check status code
    assert_eq!(response.status(), StatusCode::CREATED);

    // ---------------------------------------------------------------------------------------------

    // Send authorized log message

    let log_msg = ClearingHouseMessage {
        header: IdsMessage {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::LogMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: "https://w3id.org/idsa/core/ClearingHouse".to_string(),
            ..Default::default()
        },
        payload: Some("test".to_string()),
        payload_type: None,
    };

    // Send log message
    let log_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/log/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(SERVICE_HEADER, create_token("test", "test", &claims))
                .body(serde_json::to_string(&log_msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();

    // Check status code
    assert_eq!(log_response.status(), StatusCode::CREATED);
    // get body
    let body = axum::body::to_bytes(log_response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!body.is_empty());

    // Decode receipt
    let receipt = serde_json::from_slice::<Receipt>(&body).unwrap();
    println!("Receipt: {:?}", receipt);
    let decoded_receipt = receipt
        .data
        .decode_with_jwks(&jwks, Some(PS512))
        .expect("Decoding JWS successful");
    let decoded_receipt_header = decoded_receipt
        .header()
        .expect("Header is now already decoded");

    assert_eq!(
        decoded_receipt_header.registered.key_id,
        get_fingerprint("keys/private_key.der")
    );

    let decoded_receipt_payload = decoded_receipt
        .payload()
        .expect("Payload is now already decoded");
    println!("Decoded Receipt: {:?}", decoded_receipt);

    assert_eq!(decoded_receipt_payload.process_id, pid);
    assert_eq!(decoded_receipt_payload.payload, "test".to_string());

    // ---------------------------------------------------------------------------------------------

    // Query ID
    let query_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/query/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(SERVICE_HEADER, create_token("test", "test", &claims))
                .body(serde_json::to_string(&log_msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(query_resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(query_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!body.is_empty());

    let ids_message = serde_json::from_slice::<IdsQueryResult>(&body).unwrap();
    println!("IDS Query Result: {:?}", ids_message);
    let query_docs = ids_message.documents;

    // Check the only document in the result
    assert_eq!(query_docs.len(), 1);
    let doc = query_docs
        .first()
        .expect("Document is there, just checked")
        .to_owned();
    assert_eq!(doc.payload.expect("Payload is there"), "test".to_string());
    assert_eq!(doc.model_version, "test".to_string());

    // ---------------------------------------------------------------------------------------------

    // Send unauthorized message
    let unauthorized_claims = ChClaims::new("unauthorized");

    // Send log message
    let log_response_unauth = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/log/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(
                    SERVICE_HEADER,
                    create_token("test", "test", &unauthorized_claims),
                )
                .body(serde_json::to_string(&log_msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(log_response_unauth.status(), StatusCode::FORBIDDEN);

    // ---------------------------------------------------------------------------------------------

    // Send log message from static_process_owner
    let static_process_owner_claims = ChClaims::new("MDS_EDC_CONNECTOR");

    // Send log message
    let log_response_unauth = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/log/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(
                    SERVICE_HEADER,
                    create_token("test", "test", &static_process_owner_claims),
                )
                .body(serde_json::to_string(&log_msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(log_response_unauth.status(), StatusCode::CREATED);

    // ---------------------------------------------------------------------------------------------

    // Query as static_process_owner
    let query_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/query/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(
                    SERVICE_HEADER,
                    create_token("test", "test", &static_process_owner_claims),
                )
                .body(serde_json::to_string(&log_msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(query_resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(query_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!body.is_empty());

    let ids_message = serde_json::from_slice::<IdsQueryResult>(&body).unwrap();
    println!("IDS Query Result: {:?}", ids_message);
    let query_docs = ids_message.documents;

    // Check the only document in the result
    assert_eq!(query_docs.len(), 2);
    let doc = query_docs
        .first()
        .expect("Document is there, just checked")
        .to_owned();
    assert_eq!(doc.payload.expect("Payload is there"), "test".to_string());
    assert_eq!(doc.model_version, "test".to_string());
}
