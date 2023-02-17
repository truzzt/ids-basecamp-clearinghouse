use core_lib::api::ApiResponse;
use core_lib::api::crypto::ChClaims;
use core_lib::constants::ROCKET_KEYRING_API;
use core_lib::model::crypto::{KeyCtList, KeyMapListItem};
use rocket::fairing::AdHoc;
use rocket::State;
use rocket::serde::json::{json, Json};

use crate::db::KeyStore;
use crate::crypto::{generate_key_map, restore_key_map};

#[get("/generate_keys/<_pid>?<dt_id>", format = "json")]
async fn generate_keys(ch_claims: ChClaims, db: &State<KeyStore>, _pid: String, dt_id: String) -> ApiResponse {
    trace!("generate_keys");
    trace!("...user '{:?}'", &ch_claims.client_id);
    match db.get_msk().await{
        Ok(key) => {
            // check that doc type exists for pid
            match db.get_document_type(&dt_id).await{
                Ok(Some(dt)) => {
                    // generate new random key map
                    match generate_key_map(key, dt) {
                        Ok(key_map) => {
                            trace!("response: {:?}", &key_map);
                            return ApiResponse::SuccessCreate(json!(key_map));
                        },
                        Err(e) => {
                            error!("Error while generating key map: {}", e);
                            return ApiResponse::InternalError(String::from("Error while generating keys"));
                        }
                    }
                }
                Ok(None) =>{
                    warn!("document type {} not found", &dt_id);
                    return ApiResponse::BadRequest(String::from("Document type not found!"));
                }
                Err(e) => {
                    warn!("Error while retrieving document type: {}", e);
                    return ApiResponse::InternalError(String::from("Error while retrieving document type"));
                }
            }
        }
        Err(e) => {
            error!("Error while retrieving master key: {}", e);
            return ApiResponse::InternalError(String::from("Error while generating keys"));
        }
    }
}

#[get("/decrypt_keys/<_pid>", format = "json", data = "<key_cts>")]
async fn decrypt_keys(ch_claims: ChClaims, db: &State<KeyStore>, _pid: Option<String>, key_cts: Json<KeyCtList>) -> ApiResponse {
    trace!("decrypt_keys");
    trace!("...user '{:?}'", &ch_claims.client_id);
    let cts = key_cts.into_inner();
    debug!("number of cts to decrypt: {}", &cts.cts.len());

    // get master key
    match db.get_msk().await{
        Ok(m_key) => {
            // check that doc type exists for pid
            match db.get_document_type(&cts.dt).await{
                Ok(Some(dt)) => {
                    let mut dec_error_count = 0;
                    let mut map_error_count = 0;
                    // validate keys_ct input
                    let key_maps : Vec<KeyMapListItem> = cts.cts.iter().filter_map(
                        |key_ct| {
                            match hex::decode(key_ct.ct.clone()){
                                Ok(key) => Some((key_ct.id.clone(), key)),
                                Err(e) => {
                                    error!("Error while decoding key ciphertext: {}", e);
                                    dec_error_count = dec_error_count + 1;
                                    None
                                }
                            }
                        }
                    ).filter_map(
                            |(id, key)| {
                                match restore_key_map(m_key.clone(), dt.clone(), key){
                                    Ok(key_map) => {
                                        Some(KeyMapListItem::new(id, key_map))
                                    },
                                    Err(e) => {
                                        error!("Error while generating key map: {}", e);
                                        map_error_count = map_error_count + 1;
                                        None
                                    }
                                }
                            }
                        )
                        .collect();

                    let error_count = map_error_count + dec_error_count;

                    // Currently, we don't tolerate errors while decrypting keys
                    if error_count > 0 {
                        return ApiResponse::InternalError(String::from("Error while decrypting keys"));
                    }
                    else{
                        return ApiResponse::SuccessOk(json!(key_maps));
                    }
                }
                Ok(None) =>{
                    warn!("document type {} not found", &cts.dt);
                    return ApiResponse::BadRequest(String::from("Document type not found!"));
                }
                Err(e) => {
                    warn!("Error while retrieving document type: {}", e);
                    return ApiResponse::NotFound(String::from("Document type not found!"));
                }
            }
        }
        Err(e) => {
            error!("Error while retrieving master key: {}", e);
            return ApiResponse::InternalError(String::from("Error while decrypting keys"));
        }
    }

}

#[get("/decrypt_keys/<_pid>/<keys_ct>?<dt_id>", format = "json")]
async fn decrypt_key_map(ch_claims: ChClaims, db: &State<KeyStore>, keys_ct: String, _pid: Option<String>, dt_id: String) -> ApiResponse {
    trace!("decrypt_key_map");
    trace!("...user '{:?}'", &ch_claims.client_id);
    trace!("ct: {}", &keys_ct);
    // get master key
    match db.get_msk().await{
        Ok(key) => {
            // check that doc type exists for pid
            match db.get_document_type(&dt_id).await{
                Ok(Some(dt)) => {
                    // validate keys_ct input
                    let keys_ct = match hex::decode(keys_ct){
                        Ok(key) => key,
                        Err(e) => {
                            error!("Error while decoding key ciphertext: {}", e);
                            return ApiResponse::InternalError(String::from("Error while decrypting keys"));
                        }
                    };

                    match restore_key_map(key, dt, keys_ct){
                        Ok(key_map) => {
                            return ApiResponse::SuccessOk(json!(key_map));
                        },
                        Err(e) => {
                            error!("Error while generating key map: {}", e);
                            return ApiResponse::InternalError(String::from("Error while restoring keys"));
                        }
                    }
                }
                Ok(None) =>{
                    warn!("document type {} not found", &dt_id);
                    return ApiResponse::BadRequest(String::from("Document type not found!"));
                }
                Err(e) => {
                    warn!("Error while retrieving document type: {}", e);
                    return ApiResponse::NotFound(String::from("Document type not found!"));
                }
            }
        }
        Err(e) => {
            error!("Error while retrieving master key: {}", e);
            return ApiResponse::InternalError(String::from("Error while decrypting keys"));
        }
    }
}

pub fn mount_api() -> AdHoc {
    AdHoc::on_ignite("Mounting Keyring API", |rocket| async {
        rocket
            .mount(ROCKET_KEYRING_API, routes![decrypt_key_map, decrypt_keys, generate_keys])
    })
}