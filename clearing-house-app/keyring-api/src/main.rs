#![forbid(unsafe_code)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use core_lib::util::{add_service_config, setup_logger};
use rocket::{Build, Rocket};
use core_lib::constants::ENV_KEYRING_SERVICE_ID;
use crate::db::KeyringDbConfigurator;

mod api;
mod db;
mod crypto;
mod model;
#[cfg(test)] mod tests;

#[launch]
fn rocket() -> Rocket<Build> {
    // setup logging
    setup_logger().expect("Failure to set up the logger! Exiting...");

    rocket::build()
        .attach(add_service_config(ENV_KEYRING_SERVICE_ID.to_string()))
        .attach(api::key_api::mount_api())
        .attach(api::doc_type_api::mount_api())
        .attach(KeyringDbConfigurator)
}