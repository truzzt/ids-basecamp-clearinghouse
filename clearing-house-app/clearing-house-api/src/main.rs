#[macro_use] extern crate rocket;

use core_lib::api::client::{ApiClientConfigurator, ApiClientEnum};
use core_lib::model::JwksCache;
use core_lib::util::setup_logger;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;

use ch_lib::db::ProcessStoreConfigurator;
use ch_lib::model::constants::{SERVER_AGENT, SERVER_CONNECTOR_NAME, SERVER_MODEL_VERSION, SIGNING_KEY};
use ch_lib::model::ServerInfo;

pub mod clearing_house_api;

pub fn add_server_info() -> AdHoc {
    AdHoc::on_ignite("Adding Server Info", |rocket| async {
        let server_agent = rocket.figment().extract_inner(SERVER_AGENT).unwrap_or(String::new());
        let connector_name = rocket.figment().extract_inner(SERVER_CONNECTOR_NAME).unwrap_or(String::new());
        let model_version = rocket.figment().extract_inner(SERVER_MODEL_VERSION).unwrap_or(String::new());
        let info = ServerInfo::new(model_version, connector_name, server_agent);
        rocket.manage(info)
    })
}

pub fn add_signing_key() -> AdHoc {
    AdHoc::on_ignite("Adding Signing Key", |rocket| async {
        let private_key_path = rocket.figment().extract_inner(SIGNING_KEY).unwrap_or(String::from("keys/private_key.der"));
        rocket.manage(private_key_path)
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    // setup logging
    setup_logger().expect("Failure to set up the logger! Exiting...");

    rocket::build()
        .attach(ProcessStoreConfigurator)
        .attach(add_server_info())
        .attach(add_signing_key())
        .attach(ApiClientConfigurator::new(ApiClientEnum::Daps))
        .attach(ApiClientConfigurator::new(ApiClientEnum::Document))
        .attach(ApiClientConfigurator::new(ApiClientEnum::Keyring))
        .attach(clearing_house_api::mount_api())
        .manage(JwksCache::new())
}