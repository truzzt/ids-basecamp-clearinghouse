use rocket::response::{self, Response, Responder};
use rocket::request::Request;
use rocket::http::Header;
use crate::model::constants::IDS_HEADER;
use core_lib::api::ApiResponse;
use crate::model::ids::message::IdsMessage;

#[derive(Debug)]
pub struct IdsResponse{
    pub api_response: ApiResponse,
    pub api_header: IdsMessage,
}

impl IdsResponse {
    pub fn new(api_response: ApiResponse, api_header: IdsMessage) -> IdsResponse {
        IdsResponse {
            api_response,
            api_header,
        }
    }

    pub fn respond(api_response: ApiResponse, ids_message: IdsMessage) -> IdsResponse{
        match api_response {
            ApiResponse::BadRequest(_) => {
                IdsResponse::new(api_response,IdsMessage::error(ids_message))
            },
            ApiResponse::SuccessOk(_) => {
                IdsResponse::new(api_response, IdsMessage::return_result(ids_message))
            },
            ApiResponse::SuccessCreate(_) => {
                IdsResponse::new(api_response, IdsMessage::processed(ids_message))
            },
            ApiResponse::NotFound(_) => {
                IdsResponse::new(api_response, IdsMessage::error(ids_message))
            },
            ApiResponse::InternalError(_) => {
                IdsResponse::new(api_response,IdsMessage::error(ids_message))
            },
            ApiResponse::Unauthorized(_) => {
                IdsResponse::new(api_response, IdsMessage::error(ids_message))
            }
            _ => {
                error!("Unanticipated api response: {:?}", api_response);
                IdsResponse::new(ApiResponse::InternalError(String::from("Internal error while logging message")),IdsMessage::error(ids_message))
            }
        }
    }
}




impl<'r> Responder<'r> for IdsResponse {
     fn respond_to(self, req: &Request) -> response::Result<'r> {
        let head_str = serde_json::to_string(&self.api_header).unwrap();
        debug!("ids-reponse: {}", &head_str);
         Response::build()
             .header(Header::new(IDS_HEADER, head_str))
             .merge(self.api_response.respond_to(req)?)
             .ok()
     }
}