use core_lib::constants::ROCKET_DOC_TYPE_API;
use core_lib::errors::*;
use reqwest::{Client, StatusCode};
use reqwest::header::{CONTENT_TYPE, HeaderValue};

mod ch_api_client;
mod test_create_process;
mod test_log_message;
mod test_query_message;

/// Update this token to run tests successfully that require authentication
pub const TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJraWQiOiJkZWZhdWx0IiwiYWxnIjoiUlMyNTYifQ.eyJzY29wZXMiOlsiaWRzYzpJRFNfQ09OTkVDVE9SX0FUVFJJQlVURVNfQUxMIl0sImF1ZCI6Imlkc2M6SURTX0NPTk5FQ1RPUlNfQUxMIiwiaXNzIjoiaHR0cHM6Ly9kYXBzLmFpc2VjLmZyYXVuaG9mZXIuZGUiLCJuYmYiOjE2MjQ4OTA4NjAsImlhdCI6MTYyNDg5MDg2MCwianRpIjoiTmpNMk1UazRPRGN4TnpFME5UY3lNRGsxTlE9PSIsImV4cCI6MTYyNDg5NDQ2MCwic2VjdXJpdHlQcm9maWxlIjoiaWRzYzpUUlVTVEVEX0NPTk5FQ1RPUl9TRUNVUklUWV9QUk9GSUxFIiwicmVmZXJyaW5nQ29ubmVjdG9yIjoiaHR0cDovL2NsZWFyaW5naG91c2V0ZXN0Y29ubmVjdG9yMS5kZW1vIiwiQHR5cGUiOiJpZHM6RGF0UGF5bG9hZCIsIkBjb250ZXh0IjoiaHR0cHM6Ly93M2lkLm9yZy9pZHNhL2NvbnRleHRzL2NvbnRleHQuanNvbmxkIiwidHJhbnNwb3J0Q2VydHNTaGEyNTYiOiIxZDRlYWNkMTQ2ZTg0MmU3YjllNjdkY2EyMWVjZjk5ZTk4NDliNmY0ZWJlYzlhYmQ4ODE2NzRmOTg2M2U3Y2VkIiwic3ViIjoiQjA6MDI6NDk6MjE6NEQ6QTU6N0M6Nzc6QTg6N0Q6MjM6RDc6MzM6RkQ6NjE6NUQ6OEU6QTU6NTY6QTc6a2V5aWQ6Q0I6OEM6Qzc6QjY6ODU6Nzk6QTg6MjM6QTY6Q0I6MTU6QUI6MTc6NTA6MkY6RTY6NjU6NDM6NUQ6RTgifQ.V6UwyeERCM9mMq_7Rad1fokhHkMIZkfzQTmfqFNoA5lsd-9FL7TxHVujH82zzYdTzN6cQMN15s2f0cNsqen0bSPfscKreJQsyAdUcYm2f4KMoHGOOtvTCTCNcLkzpkx7cLOOhRKB1_fqe5zaBcjTPMxunY0OxvFK8s9JZMwvjRAuQ12FzM5OqYQrYkcjzEx38mHodP3g7preqe4Mm8B-si0tN6mS1_TXHhXLyJ8x13HroBSUT9vEm8CYPSHAc0_tte3uoWKvH_ixg2R-NRKrJA9-vEYU0fimYARmy52H3qlCKtRcVUbVelNL1V2d5ZFWFhamT8f24w2ejJBbN1ezvA";
pub const CH_API: &'static str = "http://localhost:8000";
pub const KEYRING_API: &'static str = "http://localhost:8002";
pub const EXPECTED_SENDER_AGENT: &'static str = "AISEC Clearing House";
pub const EXPECTED_ISSUER_CONNECTOR: &'static str = "https://clearinghouse.aisec.fraunhofer.de/";



fn create_dt_json(dt_id: &String, pid: &String) -> String{
    let begin_dt = r#"{"id":""#;
    let begin_pid = r#"","pid":""#;
    let rest = r#"","parts":[{"name":"model_version"},{"name":"correlation_message"},{"name":"transfer_contract"},{"name":"issued"},{"name":"issuer_connector"},{"name":"content_version"},{"name":"recipient_connector"},{"name":"sender_agent"},{"name":"recipient_agent"},{"name":"payload"},{"name":"payload_type"},{"name":"message_id"}]}"#;

    let mut json = String::from(begin_dt);
    json.push_str(dt_id);
    json.push_str(begin_pid);
    json.push_str(pid);
    json.push_str(rest);
    return json
}

fn insert_test_doc_type_into_keyring(token: &String, pid: &String, dt_id: &String) -> Result<bool>{
    let client = Client::new();
    let dt_url = format!("http://localhost:8002{}", ROCKET_DOC_TYPE_API);

    let json_data = create_dt_json(dt_id, pid);

    println!("json_data: {}", json_data);

    println!("calling {}", &dt_url);
    let mut response = client
        .post(dt_url.as_str())
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .bearer_auth(token)
        .body(json_data).send()?;

    println!("Status Code: {}", &response.status());
    match response.status(){
        StatusCode::CREATED => {
            println!("Response: {}", response.text()?);
            Ok(true)
        },
        _ => {
            panic!("Couldn't prepare doc type for test");
        }
    }
}

fn delete_test_doc_type_from_keyring(token: &String, pid: &String, dt_id: &String) -> Result<bool>{
    let client = Client::new();
    let dt_url = format!("http://localhost:8002{}/{}/{}", ROCKET_DOC_TYPE_API, pid, dt_id);

    println!("calling {}", &dt_url);
    let mut response = client
        .delete(dt_url.as_str())
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .bearer_auth(token)
        .send()?;

    println!("Status Code: {}", &response.status());
    match response.status(){
        StatusCode::NO_CONTENT => {
            println!("Response: {}", response.text()?);
            Ok(true)
        },
        _ => {
            println!("Couldn't delete document type");
            Ok(false)
        }
    }
}
