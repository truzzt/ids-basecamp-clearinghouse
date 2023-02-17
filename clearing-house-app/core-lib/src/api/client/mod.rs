use std::env;
use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::{Rocket, Build};
use crate::api::ApiClient;
use crate::api::client::keyring_api::KeyringApiClient;
use crate::api::client::document_api::DocumentApiClient;
use crate::constants::{ENV_DOCUMENT_SERVICE_ID, ENV_KEYRING_SERVICE_ID};

pub mod document_api;
pub mod keyring_api;

#[derive(Clone, Debug)]
pub enum ApiClientEnum{
    Document,
    Keyring
}

#[derive(Clone, Debug)]
pub struct ApiClientConfigurator{
    api: ApiClientEnum,
}

impl ApiClientConfigurator{
    pub fn new(api: ApiClientEnum) -> Self{
        ApiClientConfigurator{
            api
        }
    }
}

#[rocket::async_trait]
impl Fairing for ApiClientConfigurator {
    fn info(&self) -> Info {
        match self.api {
            ApiClientEnum::Document => {
                Info {
                    name: "Configuring Document Api Client",
                    kind: Kind::Ignite
                }
            },
            ApiClientEnum::Keyring => {
                Info {
                    name: "Configuring Keyring Api Client",
                    kind: Kind::Ignite
                }
            }
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let config_key = match self.api {
            ApiClientEnum::Document => {
                debug!("Configuring Document Api Client...");
                DocumentApiClient::get_conf_param()
            },
            ApiClientEnum::Keyring => {
                debug!("Configuring Keyring Api Client...");
                KeyringApiClient::get_conf_param()
            }
        };
        let api_url: String = rocket.figment().extract_inner(&config_key).unwrap_or(String::new());
        if api_url.len() > 0 {
            debug!("...found api url: {}", &api_url);
            match self.api {
                ApiClientEnum::Document => {
                    match env::var(ENV_DOCUMENT_SERVICE_ID){
                        Ok(id) => {
                            let client: DocumentApiClient = ApiClient::new(&api_url, &id);
                            Ok(rocket.manage(client))
                        },
                        Err(_e) => {
                            error!("Service ID not configured. Please configure environment variable {}", ENV_DOCUMENT_SERVICE_ID);
                            Err(rocket)
                        }
                    }
                },
                ApiClientEnum::Keyring => {
                    match env::var(ENV_KEYRING_SERVICE_ID){
                        Ok(id) => {
                            let client: KeyringApiClient = ApiClient::new(&api_url, &id);
                            Ok(rocket.manage(client))
                        },
                        Err(_e) => {
                            error!("Service ID not configured. Please configure environment variable {}", ENV_KEYRING_SERVICE_ID);
                            Err(rocket)
                        }
                    }
                }
            }
        }
        else{
            error!("...api url not found in config file.");
            Err(rocket)
        }
    }
}