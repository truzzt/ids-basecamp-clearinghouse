use core_lib::errors::*;
use core_lib::util;
use ch_lib::model::ids::message::{DOC_TYPE, IdsMessage};
use ch_lib::model::ids::request::ClearingHouseMessage;
use core_lib::constants::{CONFIG_FILE, DOCUMENT_API_URL};
use crate::ch_api_client::ClearingHouseApiClient;
use core_lib::api::{ApiClient, HashMessage};
use crate::{TOKEN, delete_test_doc_type_from_keyring, insert_test_doc_type_into_keyring, CH_API, EXPECTED_SENDER_AGENT, EXPECTED_ISSUER_CONNECTOR};
use core_lib::api::client::document_api::DocumentApiClient;
use ch_lib::model::ids::MessageType;
use ch_lib::model::ids::InfoModelId::SimpleId;

/// Testcase: Log a message to a pid that does exist
#[test]
fn test_log_message() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("test_log_message_pid");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let result = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let ch_answer: HashMessage = serde_json::from_str(result.payload.as_ref().unwrap())?;

    // There's no real test here because the payload of this message is not defined.
    // The most important information we return is the doc_id and the hash of the logged information
    // The "test" here is that the format of the messages is correct

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &ch_answer.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

/// Testcase: Log a message to a pid that already exists
#[test]
fn test_log_message_to_existing_pid() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("test_log_message_to_existing_pid");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc: HashMessage = serde_json::from_str(existing_message.payload.as_ref().unwrap())?;

    // run the test
    let json_data = util::read_file("tests/integration/json/log_message_2.json")?;
    let new_message = ch_api.log_message(&TOKEN.to_string(), &pid, json_data.clone())?;
    let new_doc: HashMessage = serde_json::from_str(new_message.payload.as_ref().unwrap())?;

    // check the result
    let json_data = util::read_file("tests/integration/json/query_message.json")?;
    let result = ch_api.query_with_pid(&TOKEN.to_string(), &pid, json_data.clone())?;
    let payload_messages: Vec<IdsMessage> = serde_json::from_str(result.payload.as_ref().unwrap())?;
    assert_eq!(payload_messages.len(), 2);

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc.doc_id)?;
    doc_api.delete_document(&TOKEN.to_string(), &pid, &new_doc.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

///Testcase: Check correctness of IDS response when logging a document
#[test]
fn check_ids_message_when_logging_document() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("check_ids_message_when_logging_document");

    let ch_message: ClearingHouseMessage = serde_json::from_str(&util::read_file("tests/integration/json/log_message.json")?)?;
    let log_message = ch_message.header;

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    // run the test
    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let result = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let result_doc: HashMessage = serde_json::from_str(result.payload.as_ref().unwrap())?;

    // check the ids response
    let ids_response = result.header;
    // we expect a message processed notification
    assert_eq!(ids_response.type_message, MessageType::MessageProcessedNotification);
    // we have one recipient agent,
    assert_eq!(ids_response.recipient_agent.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    assert_eq!(ids_response.recipient_agent.as_ref().unwrap()[0], SimpleId(log_message.sender_agent));
    // we have one recipient connector
    assert_eq!(ids_response.recipient_connector.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    assert_eq!(ids_response.recipient_connector.clone().unwrap().pop().unwrap(), log_message.issuer_connector);
    // sender agent is the clearing house (check config.yml on failure!)
    assert_eq!(ids_response.sender_agent, EXPECTED_SENDER_AGENT.to_string());
    // issuer connector is the clearing house (check config.yml on failure!)
    assert_eq!(ids_response.issuer_connector, SimpleId(EXPECTED_ISSUER_CONNECTOR.to_string()));
    // our message is the answer to the log_message
    assert_eq!(ids_response.correlation_message, log_message.id);

    //TODO: check security token
    //TODO: check auth token

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &result_doc.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

//TODO: Testcase: Create a document for existing pid with unauthorized user results in unauthorized
//TODO: Testcase: Create a document for existing pid with different authorized user works