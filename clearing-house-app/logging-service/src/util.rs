use crate::model::errors::errors;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceConfig{
    pub service_id: String
}

pub fn add_service_config(service_id: String) -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::try_on_ignite("Adding Service Config", move |rocket| async move {
        match std::env::var(&service_id){
            Ok(id) => {
                Ok(rocket.manage(ServiceConfig {service_id: id}))
            },
            Err(_e) => {
                error!("Service ID not configured. Please configure environment variable {}", &service_id);
                return Err(rocket)
            }
        }
    })
}

/// Reads a file into a string
pub fn read_file(file: &str) -> errors::Result<String> {
    std::fs::read_to_string(file)
        .map_err(|e| errors::Error::from(e))
}