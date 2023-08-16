use core_lib::api::ApiResponse;
use core_lib::constants::{ROCKET_DOC_TYPE_API, DEFAULT_PROCESS_ID};
use rocket::fairing::AdHoc;
use rocket::State;
use rocket::serde::json::{json,Json};

use crate::services::keyring_service::KeyringService;
use crate::model::doc_type::DocumentType;

#[rocket::post("/", format = "json", data = "<doc_type>")]
async fn create_doc_type(key_api: &State<KeyringService>, doc_type: Json<DocumentType>) -> ApiResponse {
    match key_api.inner().create_doc_type(doc_type.into_inner()).await{
        Ok(dt) => ApiResponse::SuccessCreate(json!(dt)),
        Err(e) => {
            error!("Error while adding doctype: {:?}", e);
            return ApiResponse::InternalError(e.to_string())
        }
    }
}

#[rocket::post("/<id>", format = "json", data = "<doc_type>")]
async fn update_doc_type(key_api: &State<KeyringService>, id: String, doc_type: Json<DocumentType>) -> ApiResponse {
    match key_api.inner().update_doc_type(id, doc_type.into_inner()).await{
        Ok(id) => ApiResponse::SuccessOk(json!(id)),
        Err(e) => {
            error!("Error while adding doctype: {:?}", e);
            return ApiResponse::InternalError(e.to_string())
        }
    }
}

#[rocket::delete("/<id>", format = "json")]
async fn delete_default_doc_type(key_api: &State<KeyringService>, id: String) -> ApiResponse{
   delete_doc_type(key_api, id, DEFAULT_PROCESS_ID.to_string()).await
}

#[rocket::delete("/<pid>/<id>", format = "json")]
async fn delete_doc_type(key_api: &State<KeyringService>, id: String, pid: String) -> ApiResponse{
    match key_api.inner().delete_doc_type(id, pid).await{
        Ok(id) => ApiResponse::SuccessOk(json!(id)),
        Err(e) => {
            error!("Error while deleting doctype: {:?}", e);
            return ApiResponse::InternalError(e.to_string())
        }
    }
}

#[rocket::get("/<id>", format = "json")]
async fn get_default_doc_type(key_api: &State<KeyringService>, id: String) -> ApiResponse {
    get_doc_type(key_api, id, DEFAULT_PROCESS_ID.to_string()).await
}

#[rocket::get("/<pid>/<id>", format = "json")]
async fn get_doc_type(key_api: &State<KeyringService>, id: String, pid: String) -> ApiResponse {
    match key_api.inner().get_doc_type(id, pid).await{
        Ok(dt) => {
            match dt{
                Some(dt) => ApiResponse::SuccessOk(json!(dt)),
                None => ApiResponse::SuccessOk(json!(null))
            }
        },
        Err(e) => {
            error!("Error while retrieving doctype: {:?}", e);
            return ApiResponse::InternalError(e.to_string())
        }
    }
}

#[rocket::get("/", format = "json")]
async fn get_doc_types(key_api: &State<KeyringService>) -> ApiResponse {
    match key_api.inner().get_doc_types().await{
        Ok(dt) => ApiResponse::SuccessOk(json!(dt)),
        Err(e) => {
            error!("Error while retrieving doctypes: {:?}", e);
            return ApiResponse::InternalError(e.to_string())
        }
    }
}

pub fn mount_api() -> AdHoc {
    AdHoc::on_ignite("Mounting Document Type API", |rocket| async {
        rocket
            .mount(ROCKET_DOC_TYPE_API, rocket::routes![create_doc_type,
                update_doc_type, delete_default_doc_type, delete_doc_type,
                get_default_doc_type, get_doc_type , get_doc_types])
    })
}