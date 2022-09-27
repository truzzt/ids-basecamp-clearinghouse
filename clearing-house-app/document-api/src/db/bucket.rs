use core_lib::model::document::EncryptedDocument;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentBucket {
    pub counter: u64,
    pub pid: String,
    pub dt_id: String,
    pub from_ts: i64,
    pub to_ts: i64,
    pub documents: Vec<EncryptedDocument>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentBucketSize {
    pub capacity: i32,
    pub size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBucketUpdate {
    pub id: String,
    pub ts: i64,
    pub tc: i64,
    pub hash: String,
    pub keys_ct: String,
    pub cts: Vec<String>
}

impl From<&EncryptedDocument> for DocumentBucketUpdate{
    fn from(doc: &EncryptedDocument) -> Self {
        DocumentBucketUpdate{
            id: doc.id.clone(),
            ts: doc.ts,
            tc: doc.tc,
            hash: doc.hash.clone(),
            keys_ct: doc.keys_ct.clone(),
            cts: doc.cts.to_vec()
        }
    }
}

pub fn restore_from_bucket(pid: &String, dt_id: &String, bucket_update: DocumentBucketUpdate) -> EncryptedDocument{
    EncryptedDocument{
        id: bucket_update.id.clone(),
        dt_id: dt_id.clone(),
        pid: pid.clone(),
        ts: bucket_update.ts,
        tc: bucket_update.tc,
        hash: bucket_update.hash.clone(),
        keys_ct: bucket_update.keys_ct.clone(),
        cts: bucket_update.cts.to_vec()
    }
}