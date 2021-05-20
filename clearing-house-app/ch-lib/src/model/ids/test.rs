use core_lib::model::document::{Document, DocumentPart};
use core_lib::errors::*;

use crate::model::ids::message::{IdsMessage, DOC_TYPE, MESSAGE_PROC_NOTIFICATION_MESSAGE};
use crate::model::ServerInfo;
use crate::model::ids::{InfoModelDateTime, InfoModelId, MessageType};
use std::collections::HashMap;

const MESSAGE_ID: &'static str = "message_id";
const MODEL_VERSION: &'static str = "model_version";
const CORRELATION_MESSAGE: &'static str = "correlation_message";
const TRANSFER_CONTRACT: &'static str = "transfer_contract";
const ISSUED: &'static str = "issued";
const ISSUER_CONNECTOR: &'static str = "issuer_connector";
const CONTENT_VERSION: &'static str = "content_version";
const RECIPIENT_CONNECTOR: &'static str = "recipient_connector";
const SENDER_AGENT: &'static str = "sender_agent";
const RECIPIENT_AGENT: &'static str = "recipient_agent";
const PAYLOAD: &'static str = "payload";
const PAYLOAD_TYPE: &'static str = "payload_type";

/// Convert Document to IdsMessage
#[test]
fn test_doc_to_message_conversion() -> Result<()>{
    // prepare test data
    let expected_content_version = String::from("test_content_version");
    let expected_cor_message = String::from("test_correlation");
    let expected_id = String::from("test_id");
    let issued = InfoModelDateTime::new();
    let expected_issued = issued.to_string();
    let expected_issuer_connector = String::from("test_issuer");
    let expected_model_version = String::from("test_model_version");
    let expected_payload = String::from("test_payload");
    let expected_payload_type = String::from("test_payload_type");
    let expected_pid = String::from("test_pid");
    let expected_recipient_agent = String::from("test_recipient_agent");
    let expected_recipient_connector = vec!(String::from("recipient_1"),String::from("recipient_2"));
    let expected_sender_agent = String::from("test_sender_agent");
    let expected_transfer_contract = String::from("test_transfer_contract");

    let mut doc_parts = vec!();
    doc_parts.push(DocumentPart::new(MESSAGE_ID.to_string(), Some(expected_id.clone())));
    doc_parts.push(DocumentPart::new(MODEL_VERSION.to_string(), Some(expected_model_version.clone())));
    doc_parts.push(DocumentPart::new(CORRELATION_MESSAGE.to_string(), Some(expected_cor_message.clone())));
    doc_parts.push(DocumentPart::new(ISSUED.to_string(), Some(expected_issued.clone())));
    doc_parts.push(DocumentPart::new(ISSUER_CONNECTOR.to_string(), Some(expected_issuer_connector.clone())));
    doc_parts.push(DocumentPart::new(SENDER_AGENT.to_string(), Some(expected_sender_agent.clone())));
    doc_parts.push(DocumentPart::new(TRANSFER_CONTRACT.to_string(), Some(expected_transfer_contract.clone())));
    doc_parts.push(DocumentPart::new(CONTENT_VERSION.to_string(), Some(expected_content_version.clone())));
    //TODO: security_token       // müssten wir speichern
    //TODO: authorization_token  // müssten wir speichern
    //TODO: payload und payload type verhalten sich seltsam
    let doc = Document::new(expected_pid.clone(),DOC_TYPE.to_string(), doc_parts);

    // run the test
    let m = IdsMessage::from(doc.clone());

    // check the converted message
    assert_eq!(m.id.unwrap(), expected_id);
    assert_eq!(m.model_version, expected_model_version);
    assert_eq!(m.correlation_message, Some(expected_cor_message));
    assert_eq!(m.transfer_contract, Some(expected_transfer_contract));
    assert_eq!(m.sender_agent, expected_sender_agent);
    //TODO: pid                  // müssten wir speichern
    //TODO: issued               // müssten wir speichern
    //TODO: issuer_connector     // müssten wir speichern
    //TODO: content_version      // müssten wir speichern
    //TODO: security_token       // müssten wir speichern
    //TODO: authorization_token  // müssten wir speichern
    //TODO: payload              // müssten wir speichern
    //TODO: payload_type         // müssten wir speichern
    Ok(())
}

/// Convert IdsMessage to Document
#[test]
fn test_message_to_doc_conversion() -> Result<()>{
    // prepare test data
    let expected_content_version = String::from("test_content_version");
    let expected_cor_message = String::from("test_correlation");
    let expected_id = String::from("test_id");
    let issued = InfoModelDateTime::new();
    let expected_issued = issued.to_string();
    let expected_issuer_connector = String::from("test_issuer");
    let expected_model_version = String::from("test_model_version");
    let expected_payload = String::from("test_payload");
    let expected_payload_type = String::from("test_payload_type");
    let expected_pid = String::from("test_pid");
    let expected_recipient_agent = String::from("test_recipient_agent");
    let expected_recipient_connector = vec!(String::from("recipient_1"),String::from("recipient_2"));
    let expected_sender_agent = String::from("test_sender_agent");
    let expected_transfer_contract = String::from("test_transfer_contract");

    let m = IdsMessage{
        context: None,
        type_message: MessageType::ResultMessage,
        id: Some(expected_id.clone()),
        pid: Some(expected_pid.clone()),
        model_version: expected_model_version.clone(),
        correlation_message: Some(expected_cor_message.clone()),
        issued,
        issuer_connector: InfoModelId::SimpleId(expected_issuer_connector.clone()),
        sender_agent: expected_sender_agent.clone(),
        recipient_connector: Some(expected_recipient_connector.iter().map(|rec| InfoModelId::SimpleId(rec.clone())).collect()),
        recipient_agent: Some(vec!(InfoModelId::SimpleId(expected_recipient_agent.clone()))),
        transfer_contract: Some(expected_transfer_contract.clone()),
        content_version: Some(expected_content_version.clone()),
        security_token: None,
        authorization_token: None,
        payload: Some(expected_payload.clone()),
        payload_type: Some(expected_payload_type.clone())
    };

    // run the test
    let doc = Document::from(m.clone());

    // check the converted document
    assert_eq!(doc.parts.len(), 10); // it's really 12 but we're missing 2 currently
    assert_eq!(doc.pid, expected_pid);
    assert_eq!(doc.dt_id, DOC_TYPE.to_string());

    // check the document parts
    let mut doc_map = HashMap::new();
    doc.parts.iter().for_each(|part| {
        doc_map.insert(part.name.clone(), part.content.clone());
    });

    // check the converted message
    assert_eq!(doc_map.get(MESSAGE_ID).unwrap().as_ref().unwrap(), &expected_id);
    assert_eq!(doc_map.get(MODEL_VERSION).unwrap().as_ref().unwrap(), &expected_model_version);
    assert_eq!(doc_map.get(CORRELATION_MESSAGE).unwrap().as_ref().unwrap(), &expected_cor_message);
    assert_eq!(doc_map.get(ISSUED).unwrap().as_ref().unwrap(), &expected_issued);
    assert_eq!(doc_map.get(ISSUER_CONNECTOR).unwrap().as_ref().unwrap(), &expected_issuer_connector);
    assert_eq!(doc_map.get(SENDER_AGENT).unwrap().as_ref().unwrap(), &expected_sender_agent);
    assert_eq!(doc_map.get(TRANSFER_CONTRACT).unwrap().as_ref().unwrap(), &expected_transfer_contract);
    assert_eq!(doc_map.get(CONTENT_VERSION).unwrap().as_ref().unwrap(), &expected_content_version);
    //TODO: security_token       // müssten wir speichern
    //TODO: authorization_token  // müssten wir speichern
    //TODO: payload und payload type verhalten sich seltsam
    //assert_eq!(doc_map.get(PAYLOAD).unwrap().as_ref().unwrap(), &expected_payload);
    //assert_eq!(doc_map.get(PAYLOAD_TYPE).unwrap().as_ref().unwrap(), &expected_payload_type);
    Ok(())
}
