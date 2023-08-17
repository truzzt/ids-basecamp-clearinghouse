use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::env;
use std::str::FromStr;

use crate::constants::ENV_API_LOG_LEVEL;
use crate::errors;
use crate::errors::*;
use figment::{Figment, providers::{Format, Yaml}};
use rocket::fairing::AdHoc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceConfig{
    pub service_id: String
}

impl ServiceConfig{
    pub fn new(service_id: String) -> ServiceConfig{
        ServiceConfig{
            service_id
        }
    }
}

pub fn load_from_test_config(key: &str, file: &str) -> String{
    Figment::new().merge(Yaml::file(file)).extract_inner(key).unwrap_or(String::new())
}

pub fn add_service_config(service_id: String) -> AdHoc{
    AdHoc::try_on_ignite("Adding Service Config", move |rocket| async move {
        match env::var(&service_id){
            Ok(id) => {
                Ok(rocket.manage(ServiceConfig::new(id)))
            },
            Err(_e) => {
                error!("Service ID not configured. Please configure environment variable {}", &service_id);
                return Err(rocket)
            }
        }
    })
}


/// setup the fern logger and set log level to environment variable `ENV_API_LOG_LEVEL`
/// allowed levels: `Off`, `Error`, `Warn`, `Info`, `Debug`, `Trace`
pub fn setup_logger() -> Result<()> {
    let log_level;
    match env::var(ENV_API_LOG_LEVEL){
        Ok(l) => log_level = l.clone(),
        Err(_e) => {
            println!("Log level not set correctly. Logging disabled");
            log_level = String::from("Off")
        }
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::from_str(&log_level.as_str())?)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

pub fn read_file(file: &str) -> Result<String> {
    std::fs::read_to_string(file)
        .map_err(|e| errors::Error::from(e))
}

pub fn url_encode(id: &str) -> String{
    utf8_percent_encode(id, NON_ALPHANUMERIC).to_string()
}