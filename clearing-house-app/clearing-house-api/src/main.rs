#[macro_use] extern crate rocket;

use std::path::Path;
use core_lib::api::client::{ApiClientConfigurator, ApiClientEnum};
use core_lib::model::JwksCache;
use core_lib::util::setup_logger;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;

use ch_lib::db::ProcessStoreConfigurator;
use ch_lib::model::constants::SIGNING_KEY;

pub mod clearing_house_api;

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
        .attach(ApiClientConfigurator::new(ApiClientEnum::Daps))
        .attach(ApiClientConfigurator::new(ApiClientEnum::Document))
        .attach(ApiClientConfigurator::new(ApiClientEnum::Keyring))
        .attach(clearing_house_api::mount_api())
        .manage(JwksCache::new())
}