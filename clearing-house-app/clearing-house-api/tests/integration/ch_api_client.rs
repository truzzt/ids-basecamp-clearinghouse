use reqwest::{Client, StatusCode};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json;
use core_lib::api::ApiClient;
use core_lib::errors::*;
use ch_lib::model::ids::message::IdsMessage;
use ch_lib::model::ids::request::ClearingHouseMessage;

#[derive(Clone)]
pub struct ClearingHouseApiClient {
    uri: String,
}

impl ApiClient for ClearingHouseApiClient {
    fn new(uri: &str) -> ClearingHouseApiClient {
        let uri = String::from(uri);
        ClearingHouseApiClient {
            uri,
        }
    }
}

impl ClearingHouseApiClient {
    pub fn log_message(&self, token: &String, pid: &String, msg: String) -> Result<ClearingHouseMessage>{
        let uri = format!("{}/messages/log/{}", self.uri, pid);
        let client = Client::new();

        println!("calling {}", &uri);
        let mut response = client.post(uri.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .bearer_auth(token)
            .body(msg)
            .send()?;

        println!("Status Code: {}", &response.status());
        println!("Headers: {:?}", response.headers());
        let ids_header = response.headers().get("ids-header").unwrap().to_str().unwrap();
        let ids_response: IdsMessage =  serde_json::from_str(ids_header)?;
        let message = ClearingHouseMessage::new(ids_response, response.text().ok(), Some("application/json".to_string()));
        Ok(message)
    }

    pub fn create_process(&self, token: &String, pid: &String, msg: String) -> Result<ClearingHouseMessage>{
        let uri = format!("{}/process/{}", self.uri, pid);
        let client = Client::new();

        println!("calling {}", &uri);
        let mut response = client.post(uri.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .bearer_auth(token)
            .body(msg)
            .send()?;

        println!("Status Code: {}", &response.status());
        println!("Headers: {:?}", response.headers());
        match response.status(){
            StatusCode::CREATED => {
                let ids_header = response.headers().get("ids-header").unwrap().to_str().unwrap();
                let ids_response: IdsMessage =  serde_json::from_str(ids_header)?;
                let message = ClearingHouseMessage::new(ids_response, response.text().ok(), Some("application/json".to_string()));
                Ok(message)
            },
            _ => Err(Error::from_kind(ErrorKind::from(format!("Status Code not ok, was {:#?}", response.status()))))
        }
    }

    pub fn query_with_pid(&self, token: &String, pid: &String, msg: String) -> Result<ClearingHouseMessage>{
        let uri = format!("{}/messages/query/{}", self.uri, pid);
        let client = Client::new();

        println!("calling {}", &uri);
        let mut response = client.post(uri.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .bearer_auth(token)
            .body(msg)
            .send()?;

        println!("Status Code: {}", &response.status());
        println!("Headers: {:?}", response.headers());
        match response.status(){
            StatusCode::OK => {
                let ids_header = response.headers().get("ids-header").unwrap().to_str().unwrap();
                let ids_response: IdsMessage =  serde_json::from_str(ids_header)?;
                let message = ClearingHouseMessage::new(ids_response, response.text().ok(), Some("application/json".to_string()));
                Ok(message)
            },
            _ => Err(Error::from_kind(ErrorKind::from(format!("Status Code not ok, was {:#?}", response.status()))))
        }
    }

    pub fn query_with_pid_and_id(&self, token: &String, pid: &String, id: &String, msg: String) -> Result<ClearingHouseMessage>{
        let uri = format!("{}/messages/query/{}/{}", self.uri, pid, id);
        let client = Client::new();

        println!("calling {}", &uri);
        let mut response = client.post(uri.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .bearer_auth(token)
            .body(msg)
            .send()?;

        println!("Status Code: {}", &response.status());
        println!("Headers: {:?}", response.headers());
        match response.status(){
            StatusCode::OK => {
                let ids_header = response.headers().get("ids-header").unwrap().to_str().unwrap();
                let ids_response: IdsMessage =  serde_json::from_str(ids_header)?;
                let message = ClearingHouseMessage::new(ids_response, response.text().ok(), Some("application/json".to_string()));
                Ok(message)
            },
            _ => Err(Error::from_kind(ErrorKind::from(format!("Status Code not ok, was {:#?}", response.status()))))
        }
    }
}