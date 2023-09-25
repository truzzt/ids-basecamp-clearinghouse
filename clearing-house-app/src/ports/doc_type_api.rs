use crate::model::constants::{DEFAULT_PROCESS_ID, ROCKET_DOC_TYPE_API};
use crate::ports::ApiResponse;
use crate::AppState;

use crate::model::doc_type::DocumentType;

//#[rocket::post("/", format = "json", data = "<doc_type>")]
async fn create_doc_type(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Json(doc_type): axum::extract::Json<DocumentType>,
) -> ApiResponse<DocumentType> {
    match state.keyring_service.create_doc_type(doc_type).await {
        Ok(dt) => ApiResponse::SuccessCreate(dt),
        Err(e) => {
            error!("Error while adding doctype: {:?}", e);
            ApiResponse::InternalError(e.to_string())
        }
    }
}

//#[rocket::post("/<id>", format = "json", data = "<doc_type>")]
async fn update_doc_type(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(doc_type): axum::extract::Json<DocumentType>,
) -> ApiResponse<bool> {
    match state.keyring_service.update_doc_type(id, doc_type).await {
        Ok(id) => ApiResponse::SuccessOk(id),
        Err(e) => {
            error!("Error while adding doctype: {:?}", e);
            ApiResponse::InternalError(e.to_string())
        }
    }
}

//#[rocket::delete("/<id>", format = "json")]
async fn delete_default_doc_type(
    state: axum::extract::State<crate::AppState>,
    id: axum::extract::Path<String>,
) -> ApiResponse<String> {
    delete_doc_type(state, id, axum::extract::Path(DEFAULT_PROCESS_ID.to_string())).await
}

//#[rocket::delete("/<pid>/<id>", format = "json")]
async fn delete_doc_type(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(pid): axum::extract::Path<String>,
) -> ApiResponse<String> {
    match state.keyring_service.delete_doc_type(id, pid).await {
        Ok(id) => ApiResponse::SuccessOk(id),
        Err(e) => {
            error!("Error while deleting doctype: {:?}", e);
            ApiResponse::InternalError(e.to_string())
        }
    }
}

//#[rocket::get("/<id>", format = "json")]
async fn get_default_doc_type(state: axum::extract::State<crate::AppState>,
                              id: axum::extract::Path<String>) -> ApiResponse<Option<DocumentType>>{
    get_doc_type(state, id, axum::extract::Path(DEFAULT_PROCESS_ID.to_string())).await
}

//#[rocket::get("/<pid>/<id>", format = "json")]
async fn get_doc_type(axum::extract::State(state): axum::extract::State<crate::AppState>,
                      axum::extract::Path(id): axum::extract::Path<String>,
                      axum::extract::Path(pid): axum::extract::Path<String>) -> ApiResponse<Option<DocumentType>> {
    match state.keyring_service.get_doc_type(id, pid).await {
        Ok(dt) => match dt {
            Some(dt) => ApiResponse::SuccessOk(Some(dt)),
            None => ApiResponse::SuccessOk(None),
        },
        Err(e) => {
            error!("Error while retrieving doctype: {:?}", e);
            ApiResponse::InternalError(e.to_string())
        }
    }
}

//#[rocket::get("/", format = "json")]
async fn get_doc_types(axum::extract::State(state): axum::extract::State<crate::AppState>) -> ApiResponse<Vec<DocumentType>> {
    match state.keyring_service.get_doc_types().await {
        Ok(dt) => ApiResponse::SuccessOk(dt),
        Err(e) => {
            error!("Error while retrieving doctypes: {:?}", e);
            ApiResponse::InternalError(e.to_string())
        }
    }
}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/",
               axum::routing::get(get_doc_types)
            .post(create_doc_type))
        .route("/:id",
               axum::routing::get(get_default_doc_type)
            .post(update_doc_type)
            .delete(delete_default_doc_type))
        .route("/:pid/:id",
               axum::routing::get(get_doc_type)
                   .delete(delete_doc_type))
}
