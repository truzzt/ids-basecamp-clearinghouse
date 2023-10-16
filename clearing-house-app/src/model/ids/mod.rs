use crate::model::ids::message::IdsMessage;

pub mod message;
pub mod request;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct InfoModelComplexId {
    //IDS name
    #[serde(rename = "@id", alias = "id", skip_serializing_if = "Option::is_none")]
    //  Correlated message, e.g. a response to a previous request
    pub id: Option<String>,
}

impl std::fmt::Display for InfoModelComplexId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.id {
            Some(id) => write!(f, "{}", serde_json::to_string(id).unwrap()),
            None => write!(f, ""),
        }
    }
}

impl InfoModelComplexId {
    pub fn new(id: String) -> InfoModelComplexId {
        InfoModelComplexId { id: Some(id) }
    }
}
impl From<String> for InfoModelComplexId {
    fn from(id: String) -> InfoModelComplexId {
        InfoModelComplexId::new(id)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InfoModelId {
    SimpleId(String),
    ComplexId(InfoModelComplexId),
}

impl InfoModelId {
    pub fn new(id: String) -> InfoModelId {
        InfoModelId::SimpleId(id)
    }
}

impl std::fmt::Display for InfoModelId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InfoModelId::SimpleId(id) => fmt.write_str(id)?,
            InfoModelId::ComplexId(id) => fmt.write_str(&id.to_string())?,
        }
        Ok(())
    }
}
impl From<String> for InfoModelId {
    fn from(id: String) -> InfoModelId {
        InfoModelId::SimpleId(id)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InfoModelDateTime {
    ComplexTime(InfoModelTimeStamp),
    Time(chrono::DateTime<chrono::Local>),
}

impl InfoModelDateTime {
    pub fn new() -> InfoModelDateTime {
        InfoModelDateTime::Time(chrono::Local::now())
    }
}

impl std::fmt::Display for InfoModelDateTime {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InfoModelDateTime::Time(value) => fmt.write_str(&value.to_string())?,
            InfoModelDateTime::ComplexTime(value) => fmt.write_str(&value.to_string())?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct InfoModelTimeStamp {
    //IDS name
    #[serde(
        rename = "@type",
        alias = "type",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<String>,
    //IDS name
    #[serde(rename = "@value", alias = "value")]
    pub value: chrono::DateTime<chrono::Local>,
}

impl Default for InfoModelTimeStamp {
    fn default() -> Self {
        InfoModelTimeStamp {
            format: Some("http://www.w3.org/2001/XMLSchema#dateTimeStamp".to_string()),
            value: chrono::Local::now(),
        }
    }
}
impl std::fmt::Display for InfoModelTimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(result) => write!(f, "{}", result),
            Err(e) => {
                error!("could not convert DateTimeStamp to json: {}", e);
                write!(f, "")
            }
        }
    }
}

/**
There are three Subclasses of the abstract ids:Message class. Namely the ids:RequestMessage, ids:ResponseMessage
and ids:NotificationMessage. Each subclass itself has subclasses that fulfill a specific purpose in the communication process.

For communication in the IDS, usually the more specific subclasses of the three mentioned ones are used.
The message classes relevant for the Connector to Connector communication are listed below. The entire Collection of Messages
available in the Information Model can be found here.

Based on [v4.2.0](https://github.com/International-Data-Spaces-Association/InformationModel/blob/v4.2.0/taxonomies/Message.ttl)
*/
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum MessageType {
    #[serde(rename = "ids:Message")]
    Message,

    /// ## Basic Message Types: Request, Response, Notification
    /// Client-generated message initiating a communication, motivated by a certain reason and with an answer expected.
    #[serde(rename = "ids:RequestMessage")]
    RequestMessage,
    /// Response messages hold information about the reaction of a recipient to a formerly sent command or event. They must be correlated to this message.
    #[serde(rename = "ids:ResponseMessage")]
    ResponseMessage,
    /// Event messages are informative and no response is expected by the sender.
    #[serde(rename = "ids:NotificationMessage")]
    NotificationMessage,

    /// ## Core IDS Messages
    /// Command messages are usually sent when a response is expected by the sender. Changes state on the recipient side. Therefore, commands are not 'safe' in the sense of REST.
    #[serde(rename = "ids:CommandMessage")]
    CommandMessage,
    /// Result messages are intended to annotate the results of a query command.
    #[serde(rename = "ids:ResultMessage")]
    ResultMessage,
    /// Rejection messages are specialized response messages that notify the sender of a message that processing of this message has failed.
    #[serde(rename = "ids:RejectionMessage")]
    RejectionMessage,

    /// ## Self-description
    /// Message requesting metadata. If no URI is supplied via the ids:requestedElement field, this messages is treated like a self-description request and the recipient should return its self-description via an ids:DescriptionResponseMessage. However, if a URI is supplied, the Connector should either return metadata about the requested element via an ids:DescriptionResponseMessage, or send an ids:RejectionMessage, e.g., because the element was not found.
    #[serde(rename = "ids:DescriptionRequestMessage")]
    DescriptionRequestMessage,
    /// Message containing the metadata, which a Connector previously requested via the ids:DescriptionRequestMessage, in its payload.
    #[serde(rename = "ids:DescriptionResponseMessage")]
    DescriptionResponseMessage,

    /// ## Connector-related Messages
    /// Superclass of all messages, indicating a change of a connector's conditions.
    #[serde(rename = "ids:ConnectorNotificationMessage")]
    ConnectorNotificationMessage,
    /// Event notifying the recipient(s) about the availability and current configuration of a connector. The payload of the message must contain the updated connector's self-description.
    #[serde(rename = "ids:ConnectorUpdateMessage")]
    ConnectorUpdateMessage,
    /// Event notifying the recipient(s) that a connector will be unavailable. The same connector may be available again in the future.
    #[serde(rename = "ids:ConnectorUnavailableMessage")]
    ConnectorUnavailableMessage,
    /// Whenever a Connector has been successfully certified by the Certification Body, the Identity Provider can use this message to notify Infrastructure Components.
    #[serde(rename = "ids:ConnectorCertificateGrantedMessage")]
    ConnectorCertificateGrantedMessage,
    /// Indicates that a (previously certified) Connector is no more certified. This could happen, for instance, if the Certification Body revokes a granted certificate or if the certificate just expires.
    #[serde(rename = "ids:ConnectorCertificateRevokedMessage")]
    ConnectorCertificateRevokedMessage,

    /// ## Participant-related Messages
    /// Superclass of all messages, indicating a change of a particpants's conditions.
    #[serde(rename = "ids:ParticipantNotificationMessage")]
    ParticipantNotificationMessage,
    /// Event notifying the recipient(s) about the availability and current description of a participant. The payload of the message must contain the participant's self-description.
    #[serde(rename = "ids:ParticipantUpdateMessage")]
    ParticipantUpdateMessage,
    /// Event notifying the recipient(s) that a participant will be unavailable. The same participant may be available again in the future.
    #[serde(rename = "ids:ParticipantUnavailableMessage")]
    ParticipantUnavailableMessage,
    /// Whenever a Participant has been successfully certified by the Certification Body, the Identity Provider can use this message to notify Infrastructure Components.
    #[serde(rename = "ids:ParticipantCertificateGrantedMessage")]
    ParticipantCertificateGrantedMessage,
    /// Indicates that a (previously certified) Participant is no more certified. This could happen, for instance, if the Certification Body revokes a granted certificate or if the certificate just expires.
    #[serde(rename = "ids:ParticipantCertificateRevokedMessage")]
    ParticipantCertificateRevokedMessage,

    /// ## Query related Messages
    /// Query message intended to be consumed by a component.
    #[serde(rename = "ids:QueryMessage")]
    QueryMessage,
    /// Class of query languages in which query strings may be formalized.
    #[serde(rename = "ids:QueryLanguage")]
    QueryLanguage,
    /// Class of recipients of a query message, e.g., BROKER, APPSTORE, ANY.
    #[serde(rename = "ids:QueryTarget")]
    QueryTarget,

    /// ## Contract Negotiation related Messages
    /// Message containing a suggested content contract (as offered by the data consumer to the data provider) in the associated payload (which is an instance of ids:ContractRequest).
    #[serde(rename = "ids:ContractRequestMessage")]
    ContractRequestMessage,
    /// Message containing a response to a contract request (of a data consumer) in form of a counter-proposal of a contract in the associated payload (which is an instance of ids:ContractOffer).
    #[serde(rename = "ids:ContractResponseMessage")]
    ContractResponseMessage,
    /// Message containing a offered content contract (as offered by a data provider to the data consumer) in the associated payload (which is an instance of ids:ContractOffer). In contrast to the ids:ContractResponseMessage, the ids:ContractOfferMessage is not related to a previous contract
    #[serde(rename = "ids:ContractOfferMessage")]
    ContractOfferMessage,
    /// Message containing a contract, as an instance of ids:ContractAgreement, with resource access modalities on which two parties have agreed in the payload.
    #[serde(rename = "ids:ContractAgreementMessage")]
    ContractAgreementMessage,
    /// Message indicating rejection of a contract.
    #[serde(rename = "ids:ContractRejectionMessage")]
    ContractRejectionMessage,
    /// Message containing supplemental information to access resources of a contract (e.g., resource access tokens).
    #[serde(rename = "ids:ContractSupplementMessage")]
    ContractSupplementMessage,

    /// ## Security-related Messages
    /// Message requesting an access token. This is intended for point-to-point communication with, e.g., Brokers.
    #[serde(rename = "ids:AccessTokenRequestMessage")]
    AccessTokenRequestMessage,
    /// Response to an access token request, intended for point-to-point communication.
    #[serde(rename = "ids:AccessTokenResponseMessage")]
    AccessTokenResponseMessage,

    /// ## Resource related messages
    /// Superclass of all messages, indicating a change of a resource.
    #[serde(rename = "ids:ResourceNotificationMessage")]
    ResourceNotificationMessage,
    /// Message indicating the availability and current description of a specific resource. The resource must be present in the payload of this message.
    #[serde(rename = "ids:ResourceUpdateMessage")]
    ResourceUpdateMessage,
    /// Message indicating that a specific resource is unavailable. The same resource may be available again in the future.
    #[serde(rename = "ids:ResourceUnavailableMessage")]
    ResourceUnavailableMessage,
    /// Message requesting the recipient to invoke a specific operation.
    #[serde(rename = "ids:OperationInvokeMessage")]
    OperationInvokeMessage,
    /// Notification that a request has been accepted and is being processed.
    #[serde(rename = "ids:RequestInProcessMessage")]
    RequestInProcessMessage,
    /// Notification that a message has been successfully processed (i.e. not ignored or rejected).
    #[serde(rename = "ids:MessageProcessedNotificationMessage")]
    MessageProcessedNotificationMessage,
    /// Message indicating that the result of a former InvokeOperation message is available. May transfer the result data in its associated payload section.
    #[serde(rename = "ids:OperationResultMessage")]
    OperationResultMessage,

    /// ## Artifact-related Messages
    /// Message asking for retrieving the specified Artifact as the payload of an ArtifactResponse message.
    #[serde(rename = "ids:ArtifactRequestMessage")]
    ArtifactRequestMessage,
    /// Message that follows up a RetrieveArtifact Message and contains the Artifact's data in the payload section.
    #[serde(rename = "ids:ArtifactResponseMessage")]
    ArtifactResponseMessage,

    /// ## Upload Messages
    /// Message used to upload a data to a recipient. Payload contains data.
    #[serde(rename = "ids:UploadMessage")]
    UploadMessage,
    /// Message that follows up a UploadMessage and contains the upload confirmation.
    #[serde(rename = "ids:UploadResponseMessage")]
    UploadResponseMessage,

    /// ## ParIS Messages
    /// This class is deprecated. Use ids:DescriptionRequestMessage instead. Message asking for retrieving the specified Participants information as the payload of an ids:ParticipantResponse message.
    #[serde(rename = "ids:ParticipantRequestMessage")]
    ParticipantRequestMessage,
    /// This class is deprecated. Use ids:DescriptionResponseMessage instead. ParticipantResponseMessage follows up a ParticipantRequestMessage and contains the Participant's information in the payload section.
    #[serde(rename = "ids:ParticipantResponseMessage")]
    ParticipantResponseMessage,

    /// ## Log messaging
    /// Log Message which can be used to transfer logs e.g., to the clearing house.
    #[serde(rename = "ids:LogMessage")]
    LogMessage,

    /// ## App-related Messages
    /// Message that asks for registration or update of a data app to the App Store. Payload contains app-related metadata (instance of class ids:AppResource). Message header may contain an app identifier parameter of a prior registered data app. If the app identifier is supplied, the message should be interpreted as a registration for an app update. Otherwise this message is used to register a new app.
    #[serde(rename = "ids:AppRegistrationRequestMessage")]
    AppRegistrationRequestMessage,
    /// Message that follows up an AppRegistrationRequestMessage and contains the app registration confirmation.
    #[serde(rename = "ids:AppRegistrationResponseMessage")]
    AppRegistrationResponseMessage,
    /// Message that usually follows a AppRegistrationResponseMessage and is used to upload a data app to the app store. Payload contains data app. Note that the message must refer to the prior sent, corresponding AppResource instance. The IRI of the ids:appArtifactReference must must match the IRI of the artifact which is the value for the ids:instance property. The ids:instance is specific for each representation. Therefore, if someone wants to upload multiple representations for an app, he has to state them using multiple ids:instance properties inside the AppRepresentation (and therefore inside the AppResource). Otherwise no mapping between payload and app metadata can be achieved.
    #[serde(rename = "ids:AppUploadMessage")]
    AppUploadMessage,
    /// Message that follows up an AppUploadMessage and contains the app upload confimation.
    #[serde(rename = "ids:AppUploadResponseMessage")]
    AppUploadResponseMessage,
    /// Superclass of all messages, indicating a change of a DataApp. Unlike Resource-related Messages, AppNotificationMessages should lead to a state change for an app at the recipient, the AppStore.
    #[serde(rename = "ids:AppNotificationMessage")]
    AppNotificationMessage,
    /// Message indicating that a specific App should be available (again) in the AppStore.
    #[serde(rename = "ids:AppAvailableMessage")]
    AppAvailableMessage,
    /// Message indicating that a specific App should be unavailable in the AppStore.
    #[serde(rename = "ids:AppUnavailableMessage")]
    AppUnavailableMessage,
    /// Message indicating that an App should be deleted from the AppStore.
    #[serde(rename = "ids:AppDeleteMessage")]
    AppDeleteMessage,

    /// TODO: Not existent in the IDS Information Model
    #[serde(rename = "ids:DynamicAttributeToken")]
    DAPSToken,
    /*
    #[serde(rename = "ids:Query")]
    Query,
    //otherwise
    Other,
    */
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct IdsQueryResult {
    pub date_from: String,
    pub date_to: String,
    pub page: i32,
    pub size: i32,
    pub order: String,
    pub documents: Vec<IdsMessage>,
}

impl IdsQueryResult {
    pub fn new(
        date_from: i64,
        date_to: i64,
        page: Option<i32>,
        size: Option<i32>,
        order: String,
        documents: Vec<IdsMessage>,
    ) -> IdsQueryResult {
        let date_from = chrono::NaiveDateTime::from_timestamp_opt(date_from, 0)
            .expect("Invalid date_from seconds")
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let date_to = chrono::NaiveDateTime::from_timestamp_opt(date_to, 0)
            .expect("Invalid date_to seconds")
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        IdsQueryResult {
            date_from,
            date_to,
            page: page.unwrap_or(-1),
            size: size.unwrap_or(-1),
            order,
            documents,
        }
    }
}
