use axum::http::StatusCode;
use crate::model::constants::DEFAULT_PROCESS_ID;
use crate::ports::ApiResponse;

use crate::model::doc_type::DocumentType;
use crate::services::keyring_service::KeyringServiceError;

type DocApiResult<T> = super::ApiResult<T, KeyringServiceError>;

async fn create_doc_type(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Json(doc_type): axum::extract::Json<DocumentType>,
) -> DocApiResult<DocumentType> {
    match state.keyring_service.create_doc_type(doc_type).await {
        Ok(dt) => Ok((StatusCode::CREATED, Json(dt))),
        Err(e) => {
            error!("Error while adding doctype: {:?}", e);
            Err(e)
        }
    }
}

async fn update_doc_type(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(doc_type): axum::extract::Json<DocumentType>,
) -> DocApiResult<bool> {
    match state.keyring_service.update_doc_type(id, doc_type).await {
        Ok(id) => Ok((StatusCode::OK, Json(id))),
        Err(e) => {
            error!("Error while adding doctype: {:?}", e);
            Err(e)
        }
    }
}

async fn delete_default_doc_type(
    state: axum::extract::State<crate::AppState>,
    id: axum::extract::Path<String>,
) -> DocApiResult<String> {
    delete_doc_type(
        state,
        id,
        axum::extract::Path(DEFAULT_PROCESS_ID.to_string()),
    )
    .await
}

async fn delete_doc_type(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(pid): axum::extract::Path<String>,
) -> DocApiResult<String> {
    match state.keyring_service.delete_doc_type(id, pid).await {
        Ok(id) => Ok((StatusCode::OK, Json(id))),
        Err(e) => {
            error!("Error while deleting doctype: {:?}", e);
            Err(e)
        }
    }
}

async fn get_default_doc_type(
    state: axum::extract::State<crate::AppState>,
    id: axum::extract::Path<String>,
) -> DocApiResult<Option<DocumentType>> {
    get_doc_type(
        state,
        id,
        axum::extract::Path(DEFAULT_PROCESS_ID.to_string()),
    )
    .await
}

//#[rocket::get("/<pid>/<id>", format = "json")]
async fn get_doc_type(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(pid): axum::extract::Path<String>,
) -> DocApiResult<Option<DocumentType>> {
    match state.keyring_service.get_doc_type(id, pid).await {
        Ok(dt) => match dt {
            Some(dt) => Ok((StatusCode::OK, Json(Some(dt)))),
            None => Ok((StatusCode::OK, Json(None)))
        },
        Err(e) => {
            error!("Error while retrieving doctype: {:?}", e);
            Err(e)
        }
    }
}

//#[rocket::get("/", format = "json")]
async fn get_doc_types(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
) -> DocApiResult<Vec<DocumentType>> {
    match state.keyring_service.get_doc_types().await {
        Ok(dt) => Ok((StatusCode::OK, Json(dt))),
        Err(e) => {
            error!("Error while retrieving doc_types: {:?}", e);
            Err(e)
        }
    }
}

pub(crate) fn router() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(get_doc_types).post(create_doc_type))
        .route(
            "/:id",
            axum::routing::get(get_default_doc_type)
                .post(update_doc_type)
                .delete(delete_default_doc_type),
        )
        .route(
            "/:pid/:id",
            axum::routing::get(get_doc_type).delete(delete_doc_type),
        )
}
