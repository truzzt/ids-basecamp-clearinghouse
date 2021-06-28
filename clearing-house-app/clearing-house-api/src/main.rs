#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate fern;
#[macro_use]
extern crate log;

use std::io::prelude::*;
use ch_lib::model::constants::{SERVER_MODEL, SERVER_NAME, SERVER_AGENT};
use core_lib::api::client::{blockchain_api::BlockchainApiClient, document_api::DocumentApiClient, keyring_api::KeyringApiClient};
use core_lib::constants::{CONFIG_FILE, DOCUMENT_API_URL, BLOCKCHAIN_API_URL, INIT_DB, DAPS_API_URL, KEYRING_API_URL};
use core_lib::util;
use core_lib::errors::*;

use ch_lib::model::ServerInfo;
use core_lib::db::DataStoreApi;
use core_lib::api::client::daps_api::DapsApiClient;
use ch_lib::db::ProcessStore;

pub mod clearing_house_api;

fn main() {
    if let Err(ref e) = launch_rocket() {
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn launch_rocket() -> Result<()> {

    // setup logging
    util::setup_logger()?;

    // read yaml config file
    let config = util::load_config(CONFIG_FILE);

    let bc_api: BlockchainApiClient = util::configure_api(BLOCKCHAIN_API_URL, &config)?;
    let daps_api: DapsApiClient = util::configure_api(DAPS_API_URL, &config)?;
    let doc_api: DocumentApiClient = util::configure_api(DOCUMENT_API_URL, &config)?;
    let key_api: KeyringApiClient = util::configure_api(KEYRING_API_URL, &config)?;

    let server_info = ServerInfo::new(
        match config[0][SERVER_MODEL].as_str() {
            Some(server_model) => server_model.to_string(),
            None => "server_model".to_string()
        },
        match config[0][SERVER_NAME].as_str() {
            Some(server_name) => server_name.to_string(),
            None => "server_name".to_string()
        },
        match config[0][SERVER_AGENT].as_str() {
            Some(server_agent) => server_agent.to_string(),
            None => "server_agent".to_string()
        }
    );
    // init database using config.yml
    let db: ProcessStore = util::configure_db(&config)?;
    // default value = true
    if config[0][INIT_DB].as_bool().unwrap_or(true) {
        db.clean_db()?;
        db.create_indexes()?;
    }

    let mut rocket = rocket::ignite()
        // configure document_api_cliet and manage it with rocket
        .manage(bc_api)
        .manage(daps_api)
        .manage(doc_api)
        .manage(key_api)
        .manage(db);
    rocket = clearing_house_api::mount(rocket, server_info);
    rocket.launch();

    Ok(())
}
