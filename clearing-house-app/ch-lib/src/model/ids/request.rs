use rocket::{
    {Request, Data, data},
    data::FromDataSimple,
    outcome::Outcome::{Success, Failure},
    http::Status
};
use std::io::Read;
use crate::model::ids::message::IdsMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearingHouseMessage {
    pub header: IdsMessage,
    pub payload: Option<String>,
    #[serde(rename = "payloadType")]
    pub payload_type: Option<String>,
}

// Always use a limit to prevent DoS attacks. (32k)
const LIMIT: u64 = 32768;


//TODO: do we need this?
impl FromDataSimple for ClearingHouseMessage {
    type Error = String;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Ensure the content type is correct before opening the data.
        //let person_ct = ContentType::new("application", "json");
        //if req.content_type() != Some(&person_ct) {
        //    return Outcome::Forward(data);
        //}

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("error: {:?} input: {}", e, string)));
        }
        println!("CH request: {}", string);
        match serde_json::from_str::<ClearingHouseMessage>(&string) {
            Ok(msg) => Success(msg),
            Err(e) => return Failure((Status::InternalServerError, format!("{:?}", e)))
        }
    }
}

impl ClearingHouseMessage {
    pub fn new(header: IdsMessage, payload: Option<String>, payload_type: Option<String>) -> ClearingHouseMessage{
        ClearingHouseMessage{
            header,
            payload,
            payload_type
        }
    }
}