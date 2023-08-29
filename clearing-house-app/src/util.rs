use crate::model::constants::SIGNING_KEY;
use crate::model::errors::errors;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceConfig {
    pub service_id: String,
}

pub fn add_service_config(service_id: String) -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::try_on_ignite("Adding Service Config", move |rocket| async move {
        match std::env::var(&service_id) {
            Ok(id) => Ok(rocket.manage(ServiceConfig { service_id: id })),
            Err(_e) => {
                error!(
                    "Service ID not configured. Please configure environment variable {}",
                    &service_id
                );
                Err(rocket)
            }
        }
    })
}

pub fn add_signing_key() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::try_on_ignite("Adding Signing Key", |rocket| async {
        let private_key_path = rocket
            .figment()
            .extract_inner(SIGNING_KEY)
            .unwrap_or(String::from("keys/private_key.der"));
        if Path::new(&private_key_path).exists() {
            Ok(rocket.manage(private_key_path))
        } else {
            tracing::error!(
                "Signing key not found! Aborting startup! Please configure signing_key!"
            );
            Err(rocket)
        }
    })
}

/// Reads a file into a string
pub fn read_file(file: &str) -> errors::Result<String> {
    std::fs::read_to_string(file).map_err(errors::Error::from)
}
