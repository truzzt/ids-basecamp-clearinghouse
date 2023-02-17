use std::env;
use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use crate::api::ApiClient;
use crate::api::crypto::create_service_token;
use crate::errors::*;
use crate::constants::{ROCKET_KEYRING_API, KEYRING_API_URL, SERVICE_HEADER, ENV_KEYRING_SERVICE_ID};
use crate::model::crypto::{KeyMap, KeyMapListItem, KeyCtList};

#[derive(Clone)]
pub struct KeyringApiClient {
    uri: String,
    api_service_id: String,
    caller_service_id: String
}

impl ApiClient for KeyringApiClient {

    fn new(uri: &str, service_id: &str) -> KeyringApiClient {
        let uri = String::from(uri);
        let api_id = match env::var(ENV_KEYRING_SERVICE_ID){
            Ok(id) => id,
            Err(_e) => {
                panic!("Service ID not configured. Please configure {}", ENV_KEYRING_SERVICE_ID);
            }
        };
        KeyringApiClient {
            uri,
            api_service_id: api_id.to_string(),
            caller_service_id: service_id.to_string()
        }
    }

    fn get_conf_param() -> String {
        String::from(KEYRING_API_URL)
    }
}

impl KeyringApiClient {

    /// Calls the keyring api to generate new aes keys
    pub async fn generate_keys(&self, client_id: &str, pid: &str, dt_id: &str) -> Result<KeyMap> {
        let keys_url = format!("{}{}/generate_keys/{}", self.uri, ROCKET_KEYRING_API, pid);
        let client = Client::new();

        let token = create_service_token(self.caller_service_id.as_str(), self.api_service_id.as_str(), client_id);

        debug!("calling {}", &keys_url);
        let result = client.get(keys_url.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(SERVICE_HEADER, &token)
            .query(&[("dt_id", dt_id)])
            .send().await?;

        debug!("Status Code: {}", result.status());
        let key_map: KeyMap = result.json().await?;
        trace!("Payload: {:?}", key_map);
        Ok(key_map)
    }

    /// Calls the keyring api to decrypt aes keys
    pub async fn decrypt_keys(&self, client_id: &str, pid: &str, dt_id: &str, ct: &[u8]) -> Result<KeyMap>{
        let keys_url = format!("{}{}/decrypt_keys/{}/{}", self.uri, ROCKET_KEYRING_API, pid, hex::encode_upper(ct));
        let client = Client::new();

        let token = create_service_token(self.caller_service_id.as_str(), self.api_service_id.as_str(), client_id);

        debug!("calling {}", &keys_url);
        let result = client.get(keys_url.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(SERVICE_HEADER, &token)
            .query(&[("dt_id", dt_id)])
            .send().await?;

        debug!("Status Code: {}", &result.status());
        let key_map: KeyMap = result.json().await?;
        trace!("Payload: {:?}", key_map);
        Ok(key_map)
    }

    /// Calls the keyring api to decrypt aes keys
    pub async fn decrypt_multiple_keys(&self, client_id: &str, pid: &str, cts: &KeyCtList) -> Result<Vec<KeyMapListItem>>{
        let keys_url = format!("{}{}/decrypt_keys/{}", self.uri, ROCKET_KEYRING_API, pid);
        let client = Client::new();

        let json_data = serde_json::to_string(cts)?;
        let token = create_service_token(self.caller_service_id.as_str(), self.api_service_id.as_str(), client_id);

        debug!("calling {}", &keys_url);
        let result = client.get(keys_url.as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(SERVICE_HEADER, &token)
            .body(json_data)
            .send().await?;

        debug!("Status Code: {}", &result.status());
        let key_maps: Vec<KeyMapListItem> = result.json().await?;
        trace!("Payload: {:?}", key_maps);
        Ok(key_maps)
    }
}