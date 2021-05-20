use core_lib::errors::*;
use core_lib::util;
use ch_lib::model::ids::message::{DOC_TYPE, IdsMessage};
use ch_lib::model::ids::request::ClearingHouseMessage;
use core_lib::constants::{CONFIG_FILE, DOCUMENT_API_URL};
use crate::ch_api_client::ClearingHouseApiClient;
use core_lib::api::{ApiClient, HashMessage};
use crate::{TOKEN, delete_test_doc_type_from_keyring, insert_test_doc_type_into_keyring, CH_API};
use core_lib::api::client::document_api::DocumentApiClient;
use ch_lib::model::ids::MessageType;
use core_lib::model::new_uuid;

///Testcase: Check correctness of IDS response when querying existing document
#[test]
fn check_ids_message_when_querying_existing_document() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("check_ids_message_when_querying_existing_document");

    let ch_message: ClearingHouseMessage = serde_json::from_str(&util::read_file("tests/integration/json/query_message.json")?)?;
    let query_message = ch_message.header;

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc: HashMessage = serde_json::from_str(existing_message.payload.as_ref().unwrap())?;

    // run the test
    let json_data = util::read_file("tests/integration/json/query_message.json")?;
    let result = ch_api.query_with_pid_and_id(&TOKEN.to_string(), &pid, &existing_doc.doc_id, json_data)?;

    // check the ids response
    let ids_response = result.header;
    // we expect a result message
    assert_eq!(ids_response.type_message, MessageType::ResultMessage);
    // we have one recipient agent
    //TODO: check
    //assert_eq!(ids_response.recipient_agent.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    //TODO: check
    //let expected_recipient_agent = query_message.recipient_agent.clone().unwrap().pop().unwrap();
    //assert_eq!(ids_response.sender_agent, expected_recipient_agent.to_string());
    // we have one recipient connector
    assert_eq!(ids_response.recipient_connector.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    assert_eq!(ids_response.recipient_connector.clone().unwrap().pop().unwrap(), query_message.issuer_connector);
    //TODO: check security token
    //TODO: check auth token
    //TODO: check correlation message!

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

/// Testcase: Query existing document
#[test]
fn test_query_existing_document() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("test_query_existing_document");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let expected_message: ClearingHouseMessage = serde_json::from_str(&util::read_file("tests/integration/json/log_message.json")?)?;
    println!("expected: {:#?}", &expected_message);
    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let message_in_ch = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc: HashMessage = serde_json::from_str(message_in_ch.payload.as_ref().unwrap())?;

    // run the test
    println!("########################### QUERY #####################################");
    let json_data = util::read_file("tests/integration/json/query_message.json")?;
    let result = ch_api.query_with_pid_and_id(&TOKEN.to_string(), &pid, &existing_doc.doc_id, json_data)?;
    let result_doc: IdsMessage = serde_json::from_str(result.payload.as_ref().unwrap())?;
    println!("########################### QUERY RESULT ##############################");
    println!("result: {:#?}", result);

    // check
    // check message_id
    assert_eq!(expected_message.header.clone().id, result_doc.id);
    // check pid
    assert_eq!(expected_message.header.clone().pid, result_doc.pid);
    // check model_version
    assert_eq!(expected_message.header.clone().model_version, result_doc.model_version);
    // check correlation message
    assert_eq!(expected_message.header.clone().correlation_message, result_doc.correlation_message);
    // check issued
    assert_eq!(expected_message.header.clone().issued, result_doc.issued);
    //TODO: check issuer connector
    //assert_eq!(expected_message.header.clone().issuer_connector, result_doc.issuer_connector);
    // check sender agent
    assert_eq!(expected_message.header.clone().sender_agent, result_doc.sender_agent);
    // check transfer contract
    assert_eq!(expected_message.header.clone().transfer_contract, result_doc.transfer_contract);
    // check content version
    assert_eq!(expected_message.header.clone().content_version, result_doc.content_version);
    //TODO: check security token
    //TODO: check authorization token
    //TODO: check payload
    //assert_eq!(expected_message.header.clone().payload, result_doc.payload);
    //TODO: check payload type
    //assert_eq!(expected_message.header.clone().payload_type, result_doc.payload_type);

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

//TODO: Testcase: Query non-existing document
#[test]
fn test_query_non_existing_document() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("test_query_non_existing_document");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let message_in_ch = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc: HashMessage = serde_json::from_str(message_in_ch.payload.as_ref().unwrap())?;
    // there's a very slim chance this fails because we request a random doc_id
    let non_existing_doc_id = new_uuid();

    // run the test
    println!("########################### QUERY #####################################");
    let json_data = util::read_file("tests/integration/json/query_message.json")?;

    // should result in error
    assert!(ch_api.query_with_pid_and_id(&TOKEN.to_string(), &pid, &non_existing_doc_id, json_data).is_err());

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

///Testcase: Check correctness of IDS response when querying for pid
#[test]
fn check_ids_message_when_querying_for_pid() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("check_ids_message_when_querying_for_pid");

    let ch_message: ClearingHouseMessage = serde_json::from_str(&util::read_file("tests/integration/json/query_message.json")?)?;
    let query_message = ch_message.header;

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message_1 = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc_1: HashMessage = serde_json::from_str(existing_message_1.payload.as_ref().unwrap())?;

    let json_data = util::read_file("tests/integration/json/log_message_2.json")?;
    let existing_message_2 = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc_2: HashMessage = serde_json::from_str(existing_message_2.payload.as_ref().unwrap())?;

    // run the test
    let json_data = util::read_file("tests/integration/json/query_message.json")?;
    let result = ch_api.query_with_pid(&TOKEN.to_string(), &pid, json_data)?;

    // check the ids response
    let ids_response = result.header;
    // we expect a result message
    assert_eq!(ids_response.type_message, MessageType::ResultMessage);
    // we have one recipient agent
    //TODO: check
    //assert_eq!(ids_response.recipient_agent.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    //TODO: check
    //let expected_recipient_agent = query_message.recipient_agent.clone().unwrap().pop().unwrap();
    //assert_eq!(ids_response.sender_agent, expected_recipient_agent.to_string());
    // we have one recipient connector
    assert_eq!(ids_response.recipient_connector.as_ref().unwrap().len(), 1);
    // which is the sender of the query message
    assert_eq!(ids_response.recipient_connector.clone().unwrap().pop().unwrap(), query_message.issuer_connector);
    //TODO: check security token
    //TODO: check auth token
    //TODO: check correlation message!

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc_1.doc_id)?;
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc_2.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

//TODO: Testcase: Query existing pid with multiple documents
#[test]
fn test_query_for_pid() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid = String::from("test_query_for_pid");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message_1 = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc_1: HashMessage = serde_json::from_str(existing_message_1.payload.as_ref().unwrap())?;

    let json_data = util::read_file("tests/integration/json/log_message_2.json")?;
    let existing_message_2 = ch_api.log_message(&TOKEN.to_string(), &pid, json_data)?;
    let existing_doc_2: HashMessage = serde_json::from_str(existing_message_2.payload.as_ref().unwrap())?;

    // run the test
    let json_data = util::read_file("tests/integration/json/query_message.json")?;
    let result = ch_api.query_with_pid(&TOKEN.to_string(), &pid, json_data)?;

    // check that we got two ids messages
    let payload_messages: Vec<IdsMessage> = serde_json::from_str(result.payload.as_ref().unwrap())?;
    assert_eq!(payload_messages.len(), 2);

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc_1.doc_id)?;
    doc_api.delete_document(&TOKEN.to_string(), &pid, &existing_doc_2.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid, &dt_id)?;

    Ok(())
}

//TODO: Testcase: Query existing pid with no documents
#[test]
fn test_query_for_pid_with_no_docs() -> Result<()> {
    // configure client_api
    let config = util::load_config(CONFIG_FILE);
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;

    // prepare test data
    let dt_id = DOC_TYPE.to_string();
    let pid_with_docs = String::from("test_pid_with_docs");
    let pid_without_docs = String::from("test_pid_with_no_docs");

    // clean up doc type (in case of previous test failure)
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid_with_docs, &dt_id)?;
    insert_test_doc_type_into_keyring(&TOKEN.to_string(), &pid_with_docs, &dt_id)?;

    let json_data = util::read_file("tests/integration/json/log_message.json")?;
    let existing_message_1 = ch_api.log_message(&TOKEN.to_string(), &pid_with_docs, json_data)?;
    let existing_doc_1: HashMessage = serde_json::from_str(existing_message_1.payload.as_ref().unwrap())?;

    let json_data = util::read_file("tests/integration/json/log_message_2.json")?;
    let existing_message_2 = ch_api.log_message(&TOKEN.to_string(), &pid_with_docs, json_data)?;
    let existing_doc_2: HashMessage = serde_json::from_str(existing_message_2.payload.as_ref().unwrap())?;

    // run the test
    let json_data = util::read_file("tests/integration/json/query_message.json")?;
    let result = ch_api.query_with_pid(&TOKEN.to_string(), &pid_without_docs, json_data)?;

    // check that we got two ids messages
    let payload_messages: Vec<IdsMessage> = serde_json::from_str(result.payload.as_ref().unwrap())?;
    assert_eq!(payload_messages.len(), 0);

    // clean up
    doc_api.delete_document(&TOKEN.to_string(), &pid_with_docs, &existing_doc_1.doc_id)?;
    doc_api.delete_document(&TOKEN.to_string(), &pid_with_docs, &existing_doc_2.doc_id)?;

    // tear down
    delete_test_doc_type_from_keyring(&TOKEN.to_string(), &pid_with_docs, &dt_id)?;

    Ok(())
}

//TODO: Testcase: Query non-existing pid
//TODO: Testcase: Query existing pid with multiple documents with different user