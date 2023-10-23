use crate::model::constants::DEFAULT_DOC_TYPE;
use crate::model::document::{Document, DocumentPart};
use crate::model::ids::{InfoModelDateTime, InfoModelId, MessageType, SecurityToken};
use std::collections::HashMap;

const MESSAGE_ID: &str = "message_id";
const MODEL_VERSION: &str = "model_version";
const CORRELATION_MESSAGE: &str = "correlation_message";
const TRANSFER_CONTRACT: &str = "transfer_contract";
const ISSUED: &str = "issued";
const ISSUER_CONNECTOR: &str = "issuer_connector";
const CONTENT_VERSION: &str = "content_version";
/// const RECIPIENT_CONNECTOR: &'static str = "recipient_connector"; // all messages should contain the CH connector, so we skip this information
const SENDER_AGENT: &str = "sender_agent";
///const RECIPIENT_AGENT: &'static str = "recipient_agent";  // all messages should contain the CH agent, so we skip this information
const PAYLOAD: &str = "payload";
const PAYLOAD_TYPE: &str = "payload_type";

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
            issued: InfoModelDateTime::new(),
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

impl IdsMessage {

    pub fn restore() -> IdsMessage {
        IdsMessage {
            type_message: MessageType::LogMessage,
            //TODO recipient_agent CH
            ..Default::default()
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
        let mut m = IdsMessage::restore();
        // pid
        m.pid = Some(doc.pid.clone());
        // message_id
        let p_map = doc.get_parts_map();
        if let Some(v) = p_map.get(MESSAGE_ID) {
            m.id = Some(v.clone());
        }
        // model_version
        if let Some(v) = p_map.get(MODEL_VERSION) {
            m.model_version = v.clone();
        }

        // correlation_message
        if let Some(v) = p_map.get(CORRELATION_MESSAGE) {
            m.correlation_message = Some(v.clone());
        }

        // transfer_contract
        if let Some(v) = p_map.get(TRANSFER_CONTRACT) {
            m.transfer_contract = Some(v.clone());
        }

        // issued
        if let Some(v) = p_map.get(ISSUED) {
            match serde_json::from_str(v) {
                Ok(date_time) => {
                    m.issued = date_time;
                }
                Err(e) => {
                    error!(
                        "Error while converting DateTimeStamp (field 'issued') from database: {}",
                        e
                    );
                }
            }
        }

        // issuer_connector
        if let Some(v) = p_map.get(ISSUER_CONNECTOR) {
            m.issuer_connector = InfoModelId::SimpleId(v.clone());
        }

        // content_version
        if let Some(v) = p_map.get(CONTENT_VERSION) {
            m.content_version = Some(v.clone());
        }

        // sender_agent
        if let Some(v) = p_map.get(SENDER_AGENT) {
            m.sender_agent = v.clone();
        }

        // payload
        if let Some(v) = p_map.get(PAYLOAD) {
            m.payload = Some(v.clone());
        }

        // payload_type
        if let Some(v) = p_map.get(PAYLOAD_TYPE) {
            m.payload_type = Some(v.clone());
        }

        //TODO: security_token
        //TODO: authorization_token

        m
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
impl TryFrom<IdsMessage> for Document {
    type Error = serde_json::Error;

    fn try_from(m: IdsMessage) -> Result<Self, Self::Error> {
        let mut doc_parts = vec![];

        // message_id
        let id = match m.id {
            Some(m_id) => m_id,
            None => autogen("Message"),
        };

        doc_parts.push(DocumentPart::new(MESSAGE_ID.to_string(), id));

        // model_version
        doc_parts.push(DocumentPart::new(
            MODEL_VERSION.to_string(),
            m.model_version,
        ));

        // correlation_message
        if let Some(s) = m.correlation_message {
            doc_parts.push(DocumentPart::new(
                CORRELATION_MESSAGE.to_string(),
                s,
            ));
        }

        // issued
        doc_parts.push(DocumentPart::new(
            ISSUED.to_string(),
            serde_json::to_string(&m.issued)?,
        ));

        // issuer_connector
        doc_parts.push(DocumentPart::new(
            ISSUER_CONNECTOR.to_string(),
            m.issuer_connector.to_string(),
        ));

        // sender_agent
        doc_parts.push(DocumentPart::new(
            SENDER_AGENT.to_string(),
            m.sender_agent.to_string(),
        ));

        // transfer_contract
        if let Some(s) = m.transfer_contract {
            doc_parts.push(DocumentPart::new(
                TRANSFER_CONTRACT.to_string(),
                s,
            ));
        }

        // content_version
        if let Some(s) = m.content_version {
            doc_parts.push(DocumentPart::new(
                CONTENT_VERSION.to_string(),
                s,
            ));
        }

        // security_token
        //TODO

        // authorization_token
        //TODO

        // payload
        if let Some(s) = m.payload {
            doc_parts.push(DocumentPart::new(PAYLOAD.to_string(), s));
        }

        // payload_type
        if let Some(s) = m.payload_type {
            doc_parts.push(DocumentPart::new(PAYLOAD_TYPE.to_string(), s));
        }

        // pid
        Ok(Document::new(m.pid.unwrap(), DEFAULT_DOC_TYPE.to_string(), -1, doc_parts))
    }
}

#[inline]
fn autogen(message: &str) -> String {
    format!(
        "https://w3id.org/idsa/autogen/{}/{}",
        message,
        Document::create_uuid()
    )
}
