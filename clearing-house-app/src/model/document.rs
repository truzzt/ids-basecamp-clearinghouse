use chrono::Local;

use crate::model::ids::message::IdsMessage;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Document {
    /// Document id
    pub id: uuid::Uuid,
    /// Process ID
    pub pid: String,
    /// timestamp: unix timestamp
    pub ts: chrono::DateTime<Local>,
    /// Content of the document
    pub content: IdsMessage,
}

/// Documents should have a globally unique id, setting the id manually is discouraged.
impl Document {
    #[must_use]
    pub fn new(pid: String, content: IdsMessage) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            pid,
            ts: Local::now(),
            content,
        }
    }
}
