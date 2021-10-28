use core_lib::errors::*;
use core_lib::util;
use ch_lib::model::ids::message::{DOC_TYPE, IdsMessage};
use ch_lib::model::ids::request::ClearingHouseMessage;
use crate::ch_api_client::ClearingHouseApiClient;
use core_lib::api::ApiClient;
use crate::{TOKEN, delete_test_doc_type_from_keyring, insert_test_doc_type_into_keyring, CH_API, EXPECTED_SENDER_AGENT, EXPECTED_ISSUER_CONNECTOR, OTHER_TOKEN};
use ch_lib::model::ids::MessageType;
use ch_lib::model::ids::InfoModelId::SimpleId;
use ch_lib::model::{OwnerList, Receipt, DataTransaction};

/// Testcase: Log a message to a pid that does exist
#[test]
fn test_log_message() -> Result<()> {
    // configure client_api
    let ch_api = ClearingHouseApiClient::new(CH_API);

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("test_log_message_pid");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let result = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let ch_answer: Receipt = serde_json::from_str(result.payload.as_ref().unwrap())?;
    let _ch_answer_doc_id = DataTransaction::from(ch_answer).document_id;

    // There's no real test here because the payload of this message is not defined.
    // The most important information we return is the doc_id and the hash of the logged information
    // The "test" here is that the format of the messages is correct

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

/// Testcase: Log a message to a pid that already exists
#[test]
fn test_log_message_to_existing_pid() -> Result<()> {
    // configure client_api
    let ch_api = ClearingHouseApiClient::new(CH_API);

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("test_log_message_to_existing_pid");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    println!("existing message: {:#?}", &existing_message);
    let existing_doc: Receipt = serde_json::from_str(existing_message.payload.as_ref().unwrap())?;
    let _existing_doc_id = DataTransaction::from(existing_doc).document_id;

    // run the test
    let json_data = util::read_file("tests/integration/json/log_message_2.json")?;
    let new_message = ch_api.log_message(&TOKEN.to_string(), &pid, json_data.clone())?;
    let new_doc: Receipt = serde_json::from_str(new_message.payload.as_ref().unwrap())?;
    let _new_doc_id = DataTransaction::from(new_doc).document_id;

    // check the result
    let json_data = util::read_file("tests/integration/json/query_message.json")?;
    let result = ch_api.query_with_pid(&TOKEN.to_string(), &pid, json_data.clone())?;
    let payload_messages: Vec<IdsMessage> = serde_json::from_str(result.payload.as_ref().unwrap())?;
    assert_eq!(payload_messages.len(), 2);

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

///Testcase: Check correctness of IDS response when logging a document
#[test]
fn check_ids_message_when_logging_document() -> Result<()> {
    // configure client_api
    let ch_api = ClearingHouseApiClient::new(CH_API);

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
    let result_doc: Receipt = serde_json::from_str(result.payload.as_ref().unwrap())?;
    let _result_doc_id = DataTransaction::from(result_doc).document_id;

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

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

///Testcase: Create a document for existing pid with unauthorized user results in unauthorized
#[test]
fn test_log_message_with_unauthorized_user() -> Result<()> {
    // configure client_api
    let ch_api = ClearingHouseApiClient::new(CH_API);

    // prepare test data and create pid
    let pid = String::from("test_log_message_with_unauthorized_user");
    let json_data = util::read_file("tests/integration/json/request_message.json")?;
    ch_api.create_process(&TOKEN.to_string(), &pid, json_data)?;

    // write message with authorized user
    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let _r: Receipt = serde_json::from_str(&existing_message.payload.as_ref().unwrap())?;

    // run the test with unauthorized user
    let json_data = util::read_file("tests/integration/json/log_message_2.json")?;
    let new_message = ch_api.log_message(&OTHER_TOKEN.to_string(), &pid, json_data.clone())?;

    println!("{:#?}", new_message);

    // check the result
    assert!(new_message.payload.unwrap().contains("User not authorized."));

    Ok(())
}

///Testcase: Create a document for existing pid with different authorized user works
#[test]
fn test_log_message_in_existing_pid_with_authorized_user() -> Result<()> {
    // configure client_api
    let ch_api = ClearingHouseApiClient::new(CH_API);

    // prepare test data and create pid
    let pid = String::from("test_log_message_in_existing_pid_with_authorized_user");
    let mut message: ClearingHouseMessage = serde_json::from_str(&util::read_file("tests/integration/json/request_message.json")?)?;
    let ownerlist = OwnerList::new(vec!(String::from("7A:2B:DD:2A:14:22:A3:50:3D:EA:FB:60:72:6A:FB:2E:58:41:CB:C0:keyid:CB:8C:C7:B6:85:79:A8:23:A6:CB:15:AB:17:50:2F:E6:65:43:5D:E8")));
    println!("old payload: {:#?}", &message.payload);
    message.payload = Some(serde_json::to_string(&ownerlist)?);
    println!("new payload: {:#?}", &message.payload);
    let json_data = serde_json::to_string(&message)?;
    ch_api.create_process(&TOKEN.to_string(), &pid, json_data)?;

    // write message with authorized user
    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let _r: Receipt = serde_json::from_str(&existing_message.payload.as_ref().unwrap())?;

    // run the test with another authorized user
    let json_data = util::read_file("tests/integration/json/log_message_2.json")?;
    let new_message = ch_api.log_message(&OTHER_TOKEN.to_string(), &pid, json_data.clone())?;

    // check that the result is a receipt
    let _r: Receipt = serde_json::from_str(&new_message.payload.as_ref().unwrap())?;

    Ok(())
}