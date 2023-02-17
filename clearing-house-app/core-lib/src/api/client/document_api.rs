use std::env;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json;
use crate::api::{ApiClient, DocumentReceipt, QueryResult};
use crate::api::crypto::create_service_token;
use crate::constants::{ROCKET_DOC_API, DOCUMENT_API_URL, SERVICE_HEADER, ENV_DOCUMENT_SERVICE_ID};
use crate::errors::*;
use crate::model::document::Document;
use crate::model::SortingOrder;
use crate::util::url_encode;

#[derive(Clone)]
pub struct DocumentApiClient {
    uri: String,
    api_service_id: String,
    caller_service_id: String
}

impl ApiClient for DocumentApiClient {
    fn new(uri: &str, service_id: &str) -> DocumentApiClient {
        let uri = String::from(uri);
        let api_id = match env::var(ENV_DOCUMENT_SERVICE_ID){
            Ok(id) => id,
            Err(_e) => {
                panic!("Service ID not configured. Please configure {}", ENV_DOCUMENT_SERVICE_ID);
            }
        };
        DocumentApiClient {
            uri,
            api_service_id: api_id.to_string(),
            caller_service_id: service_id.to_string()
        }
    }

    fn get_conf_param() -> String {
        String::from(DOCUMENT_API_URL)
    }
}

impl DocumentApiClient{

    pub async fn get_document(&self, client_id: &str, pid: &String, id: &String) -> Result<Option<Document>>{
        let document_url = format!("{}{}/{}/{}", self.uri, ROCKET_DOC_API, url_encode(pid), url_encode(id));
        let client = Client::new();

        let token = create_service_token(self.caller_service_id.as_str(), self.api_service_id.as_str(), client_id);

        debug!("calling {}", &document_url);
        let response = client
            .get(document_url.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(SERVICE_HEADER, &token)
            .send().await?;

        debug!("Status Code: {}", &response.status());
        match response.status(){
            StatusCode::OK => {
                let doc: Document = response.json().await?;
                Ok(Some(doc))
            }
            _ => Ok(None)
        }
    }

    pub async fn get_document_with_integrity_check(&self, client_id: &str, pid: &String, id: &String, hash: &String) -> Result<Document>{
        let document_url = format!("{}{}/{}/{}", self.uri, ROCKET_DOC_API, url_encode(pid), url_encode(id));
        let client = Client::new();

        let token = create_service_token(self.caller_service_id.as_str(), self.api_service_id.as_str(), client_id);

        debug!("calling {}", &document_url);
        let response = client
            .get(document_url.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(SERVICE_HEADER, &token)
            .query(&[("hash", hash.as_str())])
            .send().await?;

        debug!("Status Code: {}", &response.status());
        let doc: Document = response.json().await?;
        Ok(doc)
    }

    pub async fn get_documents(&self, client_id: &str, pid: &String, page: i32, size: i32, sort: SortingOrder, date_from: Option<String>, date_to: Option<String>) -> Result<QueryResult>{
        let document_url = format!("{}{}/{}", self.uri, ROCKET_DOC_API, url_encode(pid));
        let client = Client::new();
        debug!("calling {}", &document_url);

        let token = create_service_token(self.caller_service_id.as_str(), self.api_service_id.as_str(), client_id);

        let mut request = client
            .get(document_url.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(SERVICE_HEADER, &token)

            .query(&[("page", page)])
            .query(&[("size", size)])
            .query(&[("sort", sort)]);

        if date_from.is_some(){
            request = request.query(&[("date_from", date_from.unwrap())]);
        }

        if date_to.is_some(){
            request = request.query(&[("date_to", date_to.unwrap())]);
        }

    let response = request.send().await?;

        debug!("Status Code: {}", &response.status());
        let result: QueryResult = response.json().await?;
        Ok(result)
    }

    pub async fn create_document(&self, client_id: &str, doc: &Document) -> Result<DocumentReceipt> {
        let document_url = format!("{}{}", self.uri, ROCKET_DOC_API);
        let client = Client::new();

        let json_data = serde_json::to_string(doc)?;
        let token = create_service_token(self.caller_service_id.as_str(), self.api_service_id.as_str(), client_id);

        debug!("created jwt: {}", &token);
        debug!("calling {}", &document_url);
        let response = client
            .post(document_url.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(SERVICE_HEADER, &token)
            .body(json_data).send().await?;

        debug!("Status Code: {}", &response.status());
        match &response.status(){
            &StatusCode::CREATED => {
                let receipt = response.json().await?;
                println!("Payload: {:?}", receipt);
                Ok(receipt)
            },
            _ => bail!("Error while calling create_document(): status {} content {:?}", response.status(), response.text().await?)
        }

    }
 }