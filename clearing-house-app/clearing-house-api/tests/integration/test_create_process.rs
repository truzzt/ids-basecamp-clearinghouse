// These tests require a clean database for each test run. The tests are not able to clean
// the database after them.

use crate::{TOKEN, CH_API, EXPECTED_SENDER_AGENT, EXPECTED_ISSUER_CONNECTOR};
use core_lib::errors::*;
use core_lib::util;
use crate::ch_api_client::ClearingHouseApiClient;
use core_lib::api::ApiClient;
use ch_lib::model::ids::MessageType;
use ch_lib::model::ids::InfoModelId::SimpleId;
use ch_lib::model::ids::request::ClearingHouseMessage;

///Testcase: Check correctness of IDS response when creating a process
#[test]
fn check_ids_message_for_create_process() -> Result<()>{
    // configure client_api
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);

    // prepare test data
    let pid = String::from("check_ids_message_for_create_process");
    let ch_message: ClearingHouseMessage = serde_json::from_str(&util::read_file("tests/integration/json/request_message.json")?)?;
    let create_pid_message = ch_message.header;

    // load message json and run the test
    let json_data = util::read_file("tests/integration/json/request_message.json")?;
    let result = ch_api.create_process(&TOKEN.to_string(), &pid, json_data)?;

    // check the ids response
    let ids_response = result.header;
    // we expect a message processed notification
    assert_eq!(ids_response.type_message, MessageType::MessageProcessedNotification);
    // we have one recipient agent,
    assert_eq!(ids_response.recipient_agent.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    assert_eq!(ids_response.recipient_agent.as_ref().unwrap()[0], SimpleId(create_pid_message.sender_agent));
    // we have one recipient connector
    assert_eq!(ids_response.recipient_connector.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    assert_eq!(ids_response.recipient_connector.clone().unwrap().pop().unwrap(), create_pid_message.issuer_connector);
    // sender agent is the clearing house (check config.yml on failure!)
    assert_eq!(ids_response.sender_agent, EXPECTED_SENDER_AGENT.to_string());
    // issuer connector is the clearing house (check config.yml on failure!)
    assert_eq!(ids_response.issuer_connector, SimpleId(EXPECTED_ISSUER_CONNECTOR.to_string()));
    // our message is the answer to the log_message
    assert_eq!(ids_response.correlation_message, create_pid_message.id);

    //TODO: check security token
    //TODO: check auth token
    Ok(())
}


/// Testcase: Standard case: Create process
#[test]
fn test_create_process() -> Result<()>{
    // configure client_api
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);

    // prepare test data
    let pid = String::from("test_create_process_pid");

    // load message json and run the test
    let json_data = util::read_file("tests/integration/json/request_message.json")?;
    let result = ch_api.create_process(&TOKEN.to_string(), &pid, json_data);

    // The returned payload should contain the pid
    assert!(result.unwrap().payload.unwrap().contains(&pid));

    Ok(())
}

/// Testcase: Trying to create process with existing pid causes Error
#[test]
fn test_create_process_with_existing_pid() -> Result<()>{
    // configure client_api
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);

    // prepare test data
    let pid = String::from("test_create_process_with_existing_pid");
    let json_data = util::read_file("tests/integration/json/request_message.json")?;
    ch_api.create_process(&TOKEN.to_string(), &pid, json_data)?;

    // run the test
    let json_data = util::read_file("tests/integration/json/request_message.json")?;
    let result = ch_api.create_process(&TOKEN.to_string(), &pid, json_data);

    assert!(result.err().unwrap().description().contains("400"));

    Ok(())
}