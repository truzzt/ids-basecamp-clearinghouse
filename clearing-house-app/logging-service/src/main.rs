#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::env;
use std::path::Path;
use core_lib::api::client::{ApiClientConfigurator, ApiClientEnum};
use core_lib::util::{add_service_config, setup_logger};
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use core_lib::constants::ENV_LOGGING_SERVICE_ID;

use db::ProcessStoreConfigurator;
use model::constants::SIGNING_KEY;

pub mod logging_api;
pub mod db;
pub mod model;

pub fn add_signing_key() -> AdHoc {
    AdHoc::try_on_ignite("Adding Signing Key", |rocket| async {
        let private_key_path = rocket.figment().extract_inner(SIGNING_KEY).unwrap_or(String::from("keys/private_key.der"));
        if Path::new(&private_key_path).exists(){
            Ok(rocket.manage(private_key_path))
        }
        else{
            error!("Signing key not found! Aborting startup! Please configure signing_key!");
            return Err(rocket)
        }
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    // setup logging
    setup_logger().expect("Failure to set up the logger! Exiting...");

    rocket::build()
        .attach(ProcessStoreConfigurator)
        .attach(add_signing_key())
        .attach(add_service_config(ENV_LOGGING_SERVICE_ID.to_string()))
        .attach(ApiClientConfigurator::new(ApiClientEnum::Document))
        .attach(ApiClientConfigurator::new(ApiClientEnum::Keyring))
        .attach(logging_api::mount_api())
}
