use chrono::prelude::*;
use std::fmt;
use std::fmt::{Display, Formatter, Result};
use crate::model::ids::message::IdsMessage;

pub mod message;
pub mod request;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfoModelComplexId {
    //IDS name
    #[serde(rename = "@id", alias="id", skip_serializing_if = "Option::is_none")]
    //  Correlated message, e.g. a response to a previous request
    pub id: Option<String>
}

impl Display for InfoModelComplexId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.id {
            Some(id) => write!(f, "{}", serde_json::to_string(id).unwrap()),
            None => write!(f, "")
        }
    }
}

impl InfoModelComplexId {
    pub fn new(id: String) -> InfoModelComplexId {
        InfoModelComplexId {
            id: Some(id)
        }
    }
}
impl From<String> for InfoModelComplexId {
    fn from(id: String) -> InfoModelComplexId {
        InfoModelComplexId::new(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InfoModelId {
    SimpleId(String),
    ComplexId(InfoModelComplexId)
}

impl InfoModelId {
    pub fn new(id: String) -> InfoModelId {
        InfoModelId::SimpleId(id)
    }
    pub fn complex(id: InfoModelComplexId) -> InfoModelId {
        InfoModelId::ComplexId(id)
    }
}

impl fmt::Display for InfoModelId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoModelId::SimpleId(id) => fmt.write_str(&id)?,
            InfoModelId::ComplexId(id) => fmt.write_str(&id.to_string())?
        }
        Ok(())
    }
}
impl From<String> for InfoModelId {
    fn from(id: String) -> InfoModelId {
        InfoModelId::SimpleId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InfoModelDateTime {
    ComplexTime(InfoModelTimeStamp),
    Time(DateTime<Local>)
}

impl InfoModelDateTime {
    pub fn new() -> InfoModelDateTime {
        InfoModelDateTime::Time(Local::now())
    }
    pub fn complex() -> InfoModelDateTime {
        InfoModelDateTime::ComplexTime(InfoModelTimeStamp::default())
    }
}

impl fmt::Display for InfoModelDateTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoModelDateTime::Time(value) => fmt.write_str(&value.to_string())?,
            InfoModelDateTime::ComplexTime(value) => fmt.write_str(&value.to_string())?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfoModelTimeStamp {
    //IDS name
    #[serde(rename = "@type", alias="type", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    //IDS name
    #[serde(rename = "@value", alias="value")]
    pub value: DateTime<Local>,
}

impl Default for InfoModelTimeStamp {
    fn default() -> Self {
        InfoModelTimeStamp {
            format: Some("http://www.w3.org/2001/XMLSchema#dateTimeStamp".to_string()),
            value: Local::now()
        }
    }
}
impl Display for InfoModelTimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match serde_json::to_string(&self) {
            Ok(result) => write!(f, "{}", result),
            Err(e) => {
                error!("could not convert DateTimeStamp to json: {}", e);
                write!(f, "")
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    #[serde(rename = "ids:Message")]
    Message,
    #[serde(rename = "ids:Query")]
    Query,
    #[serde(rename = "ids:LogMessage")]
    LogMessage,
    #[serde(rename = "ids:QueryMessage")]
    QueryMessage,
    #[serde(rename = "ids:RequestMessage")]
    RequestMessage,
    #[serde(rename = "ids:ResultMessage")]
    ResultMessage,
    #[serde(rename = "ids:RejectionMessage")]
    RejectionMessage,
    #[serde(rename = "ids:MessageProcessedNotificationMessage")]
    MessageProcessedNotification,
    #[serde(rename = "ids:DynamicAttributeToken")]
    DAPSToken,
    //otherwise
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityToken {
    //IDS name
    #[serde(rename = "@type")]
    // random id without context
    pub type_message: MessageType,
    //IDS name
    #[serde(rename = "@id", alias = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    //IDS name
    #[serde(rename = "ids:tokenFormat", alias = "tokenFormat")]
    pub token_format: Option<InfoModelId>,
    //IDS name
    #[serde(rename = "ids:tokenValue", alias = "tokenValue")]
    pub token_value: String,
}

impl SecurityToken {
    pub fn new() -> SecurityToken {
        SecurityToken {
            type_message: MessageType::DAPSToken,
            id: Some(String::new()),
            token_format: None,
            token_value: String::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IdsQueryResult{
    pub date_from: String,
    pub date_to: String,
    pub page: i32,
    pub size: i32,
    pub order: String,
    pub documents: Vec<IdsMessage>
}

impl IdsQueryResult{
    pub fn new(date_from: i64, date_to: i64, page: Option<i32>, size: Option<i32>, order: String, documents: Vec<IdsMessage>) -> IdsQueryResult{

        IdsQueryResult{
            date_from: NaiveDateTime::from_timestamp(date_from, 0).format("%Y-%m-%d %H:%M:%S").to_string(),
            date_to: NaiveDateTime::from_timestamp(date_to, 0).format("%Y-%m-%d %H:%M:%S").to_string(),
            page: page.unwrap_or(-1),
            size: size.unwrap_or(-1),
            order,
            documents
        }
    }
}