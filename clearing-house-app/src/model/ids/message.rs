use crate::model::document::Document;
use crate::model::ids::{InfoModelDateTime, InfoModelId, MessageType, SecurityToken};
use std::collections::HashMap;

/// Metadata describing payload exchanged by interacting Connectors.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdsMessage {
    //IDS name
    #[serde(rename = "@context")]
    // random id without context
    pub context: Option<HashMap<String, String>>,
    //IDS name
    #[serde(rename = "@type")]
    // random id without context
    pub type_message: MessageType,
    //IDS id name
    #[serde(rename = "@id", alias = "id", skip_serializing_if = "Option::is_none")]
    // random id without context
    pub id: Option<String>,
    //skip for IDS
    #[serde(skip)]
    // process id
    pub pid: Option<String>,
    /// Version of the Information Model against which the Message should be interpreted
    #[serde(rename = "ids:modelVersion", alias = "modelVersion")]
    pub model_version: String,
    /// Correlated message, e.g., response to a previous message. Value: URI of the correlatedMessage
    #[serde(
        rename = "ids:correlationMessage",
        alias = "correlationMessage",
        skip_serializing_if = "Option::is_none"
    )]
    pub correlation_message: Option<String>,
    /// Date of issuing the Message
    #[serde(rename = "ids:issued", alias = "issued")]
    pub issued: InfoModelDateTime,
    #[serde(rename = "ids:issuerConnector", alias = "issuerConnector")]
    /// Origin Connector of the message. Value: URI of origin Connector
    pub issuer_connector: InfoModelId,
    /// Agent, which initiated the message. Value: URI of an instance of ids:Agent.
    #[serde(rename = "ids:senderAgent", alias = "senderAgent")]
    pub sender_agent: String,
    /// Target Connector. Value: URI of target Connector. Can have multiple values at the same time.
    #[serde(
        rename = "ids:recipientConnector",
        alias = "recipientConnector",
        skip_serializing_if = "Option::is_none"
    )]
    pub recipient_connector: Option<Vec<InfoModelId>>,
    /// Agent, for which the message is intended. Value: URI of an instance of ids:Agent. Can have multiple values at the same time
    #[serde(
        rename = "ids:recipientAgent",
        alias = "recipientAgent",
        skip_serializing_if = "Option::is_none"
    )]
    pub recipient_agent: Option<Vec<InfoModelId>>,
    /// Contract which is (or will be) the legal basis of the data transfer. Value: Instance of class ids:Contract.
    #[serde(
        rename = "ids:transferContract",
        alias = "transferContract",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_contract: Option<String>,
    /// Value describing the version of the content. Value: Version number of the content.
    #[serde(
        rename = "ids:contentVersion",
        alias = "contentVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_version: Option<String>,
    /// Token representing a claim, that the sender supports a certain security profile. Value: Instance of ids:DynamicAttributeToken.
    #[serde(
        rename = "ids:securityToken",
        alias = "securityToken",
        skip_serializing
    )]
    pub security_token: Option<SecurityToken>,
    /// An authorization token. The token can be issued from the Connector of the Data Provider (A) to the Connector of the
    /// Data Consumer (B). Can be used to avoid full authentication via DAPS, if Connector B wants to access the data of
    /// Connector A. Value: Instance of ids:Token
    #[serde(
        rename = "ids:authorizationToken",
        alias = "authorizationToken",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_token: Option<String>,
    //IDS name
    #[serde(skip_serializing_if = "Option::is_none")]
    // Authorization
    pub payload: Option<String>,
    //IDS name
    #[serde(skip_serializing_if = "Option::is_none")]
    // Authorization
    pub payload_type: Option<String>,
}

impl Default for IdsMessage {
    fn default() -> Self {
        IdsMessage {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::Message,
            id: Some(autogen("MessageProcessedNotification")),
            pid: None,
            model_version: "".to_string(),
            correlation_message: None,
            issued: InfoModelDateTime::default(),
            issuer_connector: InfoModelId::new("".to_string()),
            sender_agent: "https://w3id.org/idsa/core/ClearingHouse".to_string(),
            recipient_connector: None,
            recipient_agent: None,
            transfer_contract: None,
            content_version: None,
            security_token: None,
            authorization_token: None,
            payload: None,
            payload_type: None,
        }
    }
}

/// Conversion from Document to IdsMessage
///
/// note: Documents are converted into LogMessages. The LogMessage contains
/// the payload and payload type, which is the data that was stored previously.
/// All other fields of the LogMessage are meta data about the logging, e.g.
/// when the message was logged, etc.
///
/// meta data that we also need to store
/// - message_id
/// - pid
/// - model_version
/// - correlation_message
/// - issued
/// - issuer_connector
/// - sender_agent
/// - transfer_contract
/// - content_version
/// - security_token
/// - authorization_token
/// - payload
/// - payload_type
impl From<Document> for IdsMessage {
    fn from(doc: Document) -> Self {
        doc.content.clone()
    }
}

/// Conversion from IdsMessage to Document
///
/// most important part to store:
/// payload and payload type
///
/// meta data that we also need to store
/// - message_id
/// - pid
/// - model_version
/// - correlation_message
/// - issued
/// - issuer_connector
/// - sender_agent
/// - transfer_contract
/// - content_version
/// - security_token
/// - authorization_token
/// - payload
/// - payload_type
impl Into<Document> for IdsMessage {
    fn into(self) -> Document {
        let mut m = self.clone();

        m.id = Some(m.id.unwrap_or_else(|| autogen("Message")));

        // Remove security tokens to protect against impersonation of other owners of the same process
        m.security_token = None;
        m.authorization_token = None;

        Document::new(m.pid.clone().expect("Missing pid"), m)
    }
}

#[inline]
fn autogen(message: &str) -> String {
    format!(
        "https://w3id.org/idsa/autogen/{}/{}",
        message,
        uuid::Uuid::new_v4()
    )
}
