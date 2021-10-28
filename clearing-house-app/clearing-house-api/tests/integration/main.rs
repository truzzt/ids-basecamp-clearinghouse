use core_lib::constants::ROCKET_DOC_TYPE_API;
use core_lib::errors::*;
use reqwest::{Client, StatusCode};
use reqwest::header::{CONTENT_TYPE, HeaderValue};

mod ch_api_client;
mod test_create_process;
mod test_log_message;
mod test_query_message;
mod test_get_pk;

/// Update this token to run tests successfully that require authentication
// token of "standard" test user
pub const TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJraWQiOiJkZWZhdWx0IiwiYWxnIjoiUlMyNTYifQ.eyJzY29wZXMiOlsiaWRzYzpJRFNfQ09OTkVDVE9SX0FUVFJJQlVURVNfQUxMIl0sImF1ZCI6Imlkc2M6SURTX0NPTk5FQ1RPUlNfQUxMIiwiaXNzIjoiaHR0cHM6Ly9kYXBzLmFpc2VjLmZyYXVuaG9mZXIuZGUiLCJuYmYiOjE2MzM5NjYzNTIsImlhdCI6MTYzMzk2NjM1MiwianRpIjoiTXpBM05EVXdNemd5TkRNME5qWTROVFV5TXc9PSIsImV4cCI6MTYzMzk2OTk1Miwic2VjdXJpdHlQcm9maWxlIjoiaWRzYzpUUlVTVF9TRUNVUklUWV9QUk9GSUxFIiwicmVmZXJyaW5nQ29ubmVjdG9yIjoiaHR0cDovL2NvbnN1bWVyLWNvcmUuZGVtbyIsIkB0eXBlIjoiaWRzOkRhdFBheWxvYWQiLCJAY29udGV4dCI6Imh0dHBzOi8vdzNpZC5vcmcvaWRzYS9jb250ZXh0cy9jb250ZXh0Lmpzb25sZCIsInRyYW5zcG9ydENlcnRzU2hhMjU2IjoiYzE1ZTY1NTgwODhkYmZlZjIxNWE0M2QyNTA3YmJkMTI0ZjQ0ZmI4ZmFjZDU2MWMxNDU2MWEyYzFhNjY5ZDBlMCIsInN1YiI6IkE1OjBDOkE1OkYwOjg0OkQ5OjkwOkJCOkJDOkQ5OjU3OjNBOjA0OkM4OjdGOjkzOkVEOjk3OkEyOjUyOmtleWlkOkNCOjhDOkM3OkI2Ojg1Ojc5OkE4OjIzOkE2OkNCOjE1OkFCOjE3OjUwOjJGOkU2OjY1OjQzOjVEOkU4In0.AWDbnz6NDOWvQy1fHcrHvxRCmQZbKUyoxSBvqPkZFGBS-wYAxaMTT17jsMbU7K7VZLl4pAVFD-epzSospiIYXj4eKZjz-W8p9WiZT8rCtYhiJwD4WzVTlFFuhOclDKjpGKRVs1JZ_Lr_wOIRUkTCA2ckDZh5rIPWi2FjpTTn15dPYQOVjg1vMLP7GBhbCNdjon9YT0qgZfpXd86LcCwK_r791WxfzMxXrD3QqeqbJK6TZYQWulQO4DsAcT8ReBjLjU7bgnnJVU19WcbW6bW92jinaTgwJZnln33C_x1W1sQMHe_pRpUDqfYQFGrXgS1C39VeX28XU6O50Vvv9zwEUA";
// token of different user
pub const OTHER_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJraWQiOiJkZWZhdWx0IiwiYWxnIjoiUlMyNTYifQ.eyJzY29wZXMiOlsiaWRzYzpJRFNfQ09OTkVDVE9SX0FUVFJJQlVURVNfQUxMIl0sImF1ZCI6Imlkc2M6SURTX0NPTk5FQ1RPUlNfQUxMIiwiaXNzIjoiaHR0cHM6Ly9kYXBzLmFpc2VjLmZyYXVuaG9mZXIuZGUiLCJuYmYiOjE2MzM5NjYzNjQsImlhdCI6MTYzMzk2NjM2NCwianRpIjoiTkRRM01URXlNRFl5TmpNeE56QXlPRE01IiwiZXhwIjoxNjMzOTY5OTY0LCJzZWN1cml0eVByb2ZpbGUiOiJpZHNjOlRSVVNUX1NFQ1VSSVRZX1BST0ZJTEUiLCJyZWZlcnJpbmdDb25uZWN0b3IiOiJodHRwOi8vcHJvdmlkZXItY29yZS5kZW1vIiwiQHR5cGUiOiJpZHM6RGF0UGF5bG9hZCIsIkBjb250ZXh0IjoiaHR0cHM6Ly93M2lkLm9yZy9pZHNhL2NvbnRleHRzL2NvbnRleHQuanNvbmxkIiwidHJhbnNwb3J0Q2VydHNTaGEyNTYiOiIxYzE3MjAxZTk5NDg2Y2NiM2VjYWYxOWVhNThhNDljMjc4MTcxMTU0NmQxOWY3ZTJmZGJmZDBkZDk4NGY3NmQ2Iiwic3ViIjoiN0E6MkI6REQ6MkE6MTQ6MjI6QTM6NTA6M0Q6RUE6RkI6NjA6NzI6NkE6RkI6MkU6NTg6NDE6Q0I6QzA6a2V5aWQ6Q0I6OEM6Qzc6QjY6ODU6Nzk6QTg6MjM6QTY6Q0I6MTU6QUI6MTc6NTA6MkY6RTY6NjU6NDM6NUQ6RTgifQ.HXZJXbST8RmNefHMAOETyQ3Otg1mBycZIgL52lPwPipVsYysBpqJqLeE7nRhFD6al6jDK6pXm4ghLA8BtOtLCsa7Rgf3zlW8XEdGacc_IMFvzZjudUvvBLw8xEDPRt535H6bIO0Ix-1rSEKEfLm644HZ2KsuHCxYMwn6n-zkmkbAJQNNXcEtC-kY1cB28NI-pPcl17IOw9jOvgaMpi7cERopkWDnFaGzBxQFKeEGdLH82dzlgx-CCySg2eAZFMuTUK2Ix1zRT2A0TBCwOTaPl6pzvUGN4X1oNQCH_bUvHIFUi-ERCql2pkvtlAuVY411mXqRpHrBv6NIulyGeaHRoQ";
pub const CH_API: &'static str = "http://localhost:8000";
pub const KEYRING_API: &'static str = "http://localhost:8002";
pub const EXPECTED_SENDER_AGENT: &'static str = "https://clearinghouse.aisec.fraunhofer.de";
pub const EXPECTED_ISSUER_CONNECTOR: &'static str = "https://clearinghouse.aisec.fraunhofer.de/";
pub const TEST_CONFIG: &'static str = "config.yml";


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
