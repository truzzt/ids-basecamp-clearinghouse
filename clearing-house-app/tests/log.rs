#![cfg(test)]

use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use clearing_house_app::model::ids::message::IdsMessage;
use clearing_house_app::model::ids::{InfoModelDateTime, InfoModelId, MessageType};
use clearing_house_app::model::ids::request::ClearingHouseMessage;
use clearing_house_app::util::new_uuid;
use clearing_house_app::model::{claims::create_token, constants::SERVICE_HEADER};
use clearing_house_app::model::claims::ChClaims;

#[tokio::test]
#[ignore]
async fn log_message() {
    std::env::set_var("SERVICE_ID_LOG", "test");
    std::env::set_var("SHARED_SECRET", "test");
    std::env::set_var("CH_APP_LOG_LEVEL", "TRACE");
    std::env::set_var("CH_APP_CLEAR_DB", "false");

    let app = clearing_house_app::app().await.unwrap();

    let pid = new_uuid();

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
            id: Some(new_uuid()),
            pid: None,
            model_version: "".to_string(),
            correlation_message: None,
            issued: InfoModelDateTime::default(),
            issuer_connector: InfoModelId::new("".to_string()),
            sender_agent: "https://w3id.org/idsa/core/ClearingHouse".to_string(),
            recipient_connector: None,
            recipient_agent: None,
            transfer_contract: None,
            content_version: None,
            security_token: None,
            authorization_token: None,
            payload: None,
            payload_type: None,
        },
        payload: Some("test".to_string()),
        payload_type: None,
    };

    let claims = ChClaims::new("test");

    // Log
    let response = app.clone()
        .oneshot(Request::builder()
            .uri(format!("/messages/log/{}", pid))
            .method("POST")
            .header("Content-Type", "application/json")
            .header(SERVICE_HEADER, create_token("test", "test", &claims))
            .body(serde_json::to_string(&msg).unwrap().into()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(!body.is_empty());
    println!("Receipt: {:?}", body);

    // Query
    let query_resp = app
        .oneshot(Request::builder()
            .uri(format!("/messages/query/{}", pid))
            .method("POST")
            .header("Content-Type", "application/json")
            .header(SERVICE_HEADER, create_token("test", "test", &claims))
            .body(serde_json::to_string(&msg).unwrap().into()).unwrap())
        .await
        .unwrap();

    let body = hyper::body::to_bytes(query_resp.into_body()).await.unwrap();
    println!("Query: {:?}", body);

}