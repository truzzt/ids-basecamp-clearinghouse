use crate::util::new_uuid;
use chrono::Local;

use crate::model::ids::message::IdsMessage;
use uuid::Uuid;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Document {
    /// Document id
    #[serde(default = "new_uuid")]
    pub id: String,
    /// Process ID
    pub pid: String,
    /// timestamp: unix timestamp
    pub ts: chrono::DateTime<Local>,
    /// Content of the document
    pub content: IdsMessage,
}

/// Documents should have a globally unique id, setting the id manually is discouraged.
impl Document {
    pub fn create_uuid() -> String {
        Uuid::new_v4().hyphenated().to_string()
    }

    pub fn new(pid: String, content: IdsMessage) -> Self {
        Self {
            id: Document::create_uuid(),
            pid,
            ts: Local::now(),
            content,
        }
    }
}
