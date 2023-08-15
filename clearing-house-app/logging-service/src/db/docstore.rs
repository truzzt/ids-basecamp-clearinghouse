use mongodb::bson;
use mongodb::bson::doc;
use mongodb::options::{AggregateOptions, UpdateOptions};
use rocket::futures::StreamExt;
use core_lib::constants::{
    MAX_NUM_RESPONSE_ENTRIES,
    MONGO_COLL_DOCUMENT_BUCKET,
    MONGO_ID,
    MONGO_PID,
    MONGO_COUNTER,
    MONGO_DOC_ARRAY,
    MONGO_DT_ID,
    MONGO_FROM_TS,
    MONGO_TO_TS,
    MONGO_TC,
    MONGO_TS,
};
use core_lib::model::document::EncryptedDocument;
use core_lib::errors::*;
use core_lib::model::SortingOrder;
use crate::db::docstore::bucket::{DocumentBucketSize, DocumentBucketUpdate, restore_from_bucket};

#[derive(Clone)]
pub struct DataStore {
    client: mongodb::Client,
    database: mongodb::Database,
}

impl DataStore {
    pub async fn add_document(&self, doc: EncryptedDocument) -> Result<bool> {
        debug!("add_document to bucket");
        let coll = self.database.collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);
        let bucket_update = DocumentBucketUpdate::from(&doc);
        let mut update_options = UpdateOptions::default();
        update_options.upsert = Some(true);
        let id = format!("^{}_", doc.pid.clone());
        let re = mongodb::bson::Regex {
            pattern: id,
            options: String::new(),
        };

        let query = doc! {"_id": re, MONGO_PID: doc.pid.clone(), MONGO_COUNTER: mongodb::bson::bson!({"$lt": MAX_NUM_RESPONSE_ENTRIES as i64})};

        match coll.update_one(query,
                              doc! {
                            "$push": {
                                MONGO_DOC_ARRAY: mongodb::bson::to_bson(&bucket_update).unwrap(),
                            },
                            "$inc": {"counter": 1},
                            "$setOnInsert": { "_id": format!("{}_{}", doc.pid.clone(), doc.ts), MONGO_DT_ID: doc.dt_id.clone(), MONGO_FROM_TS: doc.ts},
                            "$set": {MONGO_TO_TS: doc.ts},
                        }, update_options).await {
            Ok(_r) => {
                debug!("added new document: {:#?}", &_r.upserted_id);
                Ok(true)
            }
            Err(e) => {
                error!("failed to store document: {:#?}", &e);
                Err(Error::from(e))
            }
        }
    }

    /// checks if the document exists
    /// document ids are globally unique
    pub async fn exists_document(&self, id: &String) -> Result<bool> {
        debug!("Check if document with id '{}' exists...", id);
        let query = doc! {format!("{}.{}", MONGO_DOC_ARRAY, MONGO_ID): id.clone()};

        let coll = self.database.collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);
        match coll.count_documents(Some(query), None).await? {
            0 => {
                debug!("Document with id '{}' does not exist!", &id);
                Ok(false)
            }
            _ => {
                debug!("... found.");
                Ok(true)
            }
        }
    }

    /// gets the model from the db
    pub async fn get_document(&self, id: &String, pid: &String) -> Result<Option<EncryptedDocument>> {
        debug!("Trying to get doc with id {}...", id);
        let coll = self.database.collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);

        let pipeline = vec![doc! {"$match":{
                                            MONGO_PID: pid.clone(),
                                            format!("{}.{}", MONGO_DOC_ARRAY, MONGO_ID): id.clone()
                                        }},
                            doc! {"$unwind": format!("${}", MONGO_DOC_ARRAY)},
                            doc! {"$addFields": {format!("{}.{}", MONGO_DOC_ARRAY, MONGO_PID): format!("${}", MONGO_PID), format!("{}.{}", MONGO_DOC_ARRAY, MONGO_DT_ID): format!("${}", MONGO_DT_ID)}},
                            doc! {"$replaceRoot": { "newRoot": format!("${}", MONGO_DOC_ARRAY)}},
                            doc! {"$match":{ MONGO_ID: id.clone()}}];

        let mut results = coll.aggregate(pipeline, None).await?;

        if let Some(result) = results.next().await {
            let doc: EncryptedDocument = bson::from_document(result?)?;
            return Ok(Some(doc));
        }

        return Ok(None);
    }

    /// gets documents for a single process from the db
    pub async fn get_document_with_previous_tc(&self, tc: i64) -> Result<Option<EncryptedDocument>> {
        let previous_tc = tc - 1;
        debug!("Trying to get document for tc {} ...", previous_tc);
        if previous_tc < 0 {
            info!("... not entry exists.");
            Ok(None)
        } else {
            let coll = self.database.collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);

            let pipeline = vec![doc! {"$match":{
                                                format!("{}.{}", MONGO_DOC_ARRAY, MONGO_TC): previous_tc
                                            }},
                                doc! {"$unwind": format!("${}", MONGO_DOC_ARRAY)},
                                doc! {"$addFields": {format!("{}.{}", MONGO_DOC_ARRAY, MONGO_PID): format!("${}", MONGO_PID), format!("{}.{}", MONGO_DOC_ARRAY, MONGO_DT_ID): format!("${}", MONGO_DT_ID)}},
                                doc! {"$replaceRoot": { "newRoot": format!("${}", MONGO_DOC_ARRAY)}},
                                doc! {"$match":{ MONGO_TC: previous_tc}}];

            let mut results = coll.aggregate(pipeline, None).await?;

            return if let Some(result) = results.next().await {
                debug!("Found {:#?}", &result);
                let doc: EncryptedDocument = bson::from_document(result?)?;
                Ok(Some(doc))
            } else {
                warn!("Document with tc {} not found!", previous_tc);
                Ok(None)
            };
        }
    }

    /// gets a page of documents of a specific document type for a single process from the db defined by parameters page, size and sort
    pub async fn get_documents_for_pid(&self, dt_id: &String, pid: &String, page: u64, size: u64, sort: &SortingOrder, date_from: &chrono::NaiveDateTime, date_to: &chrono::NaiveDateTime) -> Result<Vec<EncryptedDocument>> {
        debug!("...trying to get page {} of size {} of documents for pid {} of dt {}...", pid, dt_id, page, size);

        match self.get_start_bucket_size(dt_id, pid, page, size, sort, date_from, date_to).await {
            Ok(bucket_size) => {
                let offset = DataStore::get_offset(&bucket_size);
                let start_bucket = DataStore::get_start_bucket(page, size, &bucket_size, offset);
                trace!("...working with start_bucket {} and offset {} ...", start_bucket, offset);
                let start_entry = DataStore::get_start_entry(page, size, start_bucket, &bucket_size, offset);

                trace!("...working with start_entry {} in start_bucket {} and offset {} ...", start_entry, start_bucket, offset);

                let skip_buckets = (start_bucket - 1) as i32;
                let sort_order = match sort {
                    SortingOrder::Ascending => 1,
                    SortingOrder::Descending => -1,
                };

                let pipeline = vec![doc! {"$match":{
                                            MONGO_PID: pid.clone(),
                                            MONGO_DT_ID: dt_id.clone(),
                                            MONGO_FROM_TS: {"$lte": date_to.timestamp()},
                                            MONGO_TO_TS: {"$gte": date_from.timestamp()}
                                        }},
                                    doc! {"$sort" : {MONGO_FROM_TS: sort_order}},
                                    doc! {"$skip" : skip_buckets},
                                    // worst case: overlap between two buckets.
                                    doc! {"$limit" : 2},
                                    doc! {"$unwind": format!("${}", MONGO_DOC_ARRAY)},
                                    doc! {"$replaceRoot": { "newRoot": "$documents"}},
                                    doc! {"$match":{
                                            MONGO_TS: {"$gte": date_from.timestamp(), "$lte": date_to.timestamp()}
                                        }},
                                    doc! {"$sort" : {MONGO_TS: sort_order}},
                                    doc! {"$skip" : start_entry as i32},
                                    doc! { "$limit": size as i32}];

                let coll = self.database.collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);

                let mut options = AggregateOptions::default();
                options.allow_disk_use = Some(true);
                let mut results = coll.aggregate(pipeline, options).await?;

                let mut docs = vec!();
                while let Some(result) = results.next().await {
                    let doc: DocumentBucketUpdate = bson::from_document(result?)?;
                    docs.push(restore_from_bucket(pid, dt_id, doc));
                }

                return Ok(docs);
            }
            Err(e) => {
                error!("Error while getting bucket offset!");
                Err(Error::from(e))
            }
        }
    }

    /// offset is necessary for duration queries. There, start_entries of bucket depend on timestamps which usually creates an offset in the bucket
    async fn get_start_bucket_size(&self, dt_id: &String, pid: &String, page: u64, size: u64, sort: &SortingOrder, date_from: &chrono::NaiveDateTime, date_to: &chrono::NaiveDateTime) -> Result<DocumentBucketSize> {
        debug!("...trying to get the offset for page {} of size {} of documents for pid {} of dt {}...", pid, dt_id, page, size);
        let sort_order = match sort {
            SortingOrder::Ascending => {
                1
            }
            SortingOrder::Descending => {
                -1
            }
        };
        let coll = self.database.collection::<DocumentBucketSize>(MONGO_COLL_DOCUMENT_BUCKET);

        debug!("... match with pid: {}, dt_it: {}, to_ts <= {}, from_ts >= {} ...", pid, dt_id, date_from.timestamp(), date_to.timestamp());
        let pipeline = vec![doc! {"$match":{
                                            MONGO_PID: pid.clone(),
                                            MONGO_DT_ID: dt_id.clone(),
                                            MONGO_FROM_TS: {"$lte": date_to.timestamp()},
                                            MONGO_TO_TS: {"$gte": date_from.timestamp()}
                                        }},
                            // sorting according to sorting order, so we get either the start or end
                            doc! {"$sort" : {MONGO_FROM_TS: sort_order}},
                            doc! {"$limit" : 1},
                            // count all relevant documents in the target bucket
                            doc! {"$unwind": format!("${}", MONGO_DOC_ARRAY)},
                            doc! {"$match":{
                                            format!("{}.{}", MONGO_DOC_ARRAY, MONGO_TS): {"$lte": date_to.timestamp(), "$gte": date_from.timestamp()}
                                        }},
                            // modify result to return total number of docs in bucket and number of relevant docs in bucket
                            doc! { "$group": { "_id": {"total": "$counter"}, "size": { "$sum": 1 } } },
                            doc! { "$project": {"_id":0, "capacity": "$_id.total", "size":true}}];

        let mut options = AggregateOptions::default();
        options.allow_disk_use = Some(true);
        let mut results = coll.aggregate(pipeline, options).await?;
        let mut bucket_size = DocumentBucketSize {
            capacity: MAX_NUM_RESPONSE_ENTRIES as i32,
            size: 0,
        };
        while let Some(result) = results.next().await {
            debug!("... retrieved: {:#?}", &result);
            let result_bucket: DocumentBucketSize = bson::from_document(result?)?;
            bucket_size = result_bucket;
        }
        debug!("... sending offset: {:?}", bucket_size);
        Ok(bucket_size)
    }

    fn get_offset(bucket_size: &DocumentBucketSize) -> u64 {
        return (bucket_size.capacity - bucket_size.size) as u64 % MAX_NUM_RESPONSE_ENTRIES;
    }

    fn get_start_bucket(page: u64, size: u64, bucket_size: &DocumentBucketSize, offset: u64) -> u64 {
        let docs_to_skip = (page - 1) * size + offset + MAX_NUM_RESPONSE_ENTRIES - bucket_size.capacity as u64;
        return (docs_to_skip / MAX_NUM_RESPONSE_ENTRIES) + 1;
    }

    fn get_start_entry(page: u64, size: u64, start_bucket: u64, bucket_size: &DocumentBucketSize, offset: u64) -> u64 {
        // docs to skip calculated by page * size
        let docs_to_skip = (page - 1) * size + offset;
        let mut start_entry = 0;
        if start_bucket > 1 {
            start_entry = docs_to_skip - bucket_size.capacity as u64;
            if start_entry > 2 {
                start_entry = start_entry - (start_bucket - 2) * MAX_NUM_RESPONSE_ENTRIES
            }
        }
        return start_entry;
    }
}

mod bucket {
    use core_lib::model::document::EncryptedDocument;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct DocumentBucket {
        pub counter: u64,
        pub pid: String,
        pub dt_id: String,
        pub from_ts: i64,
        pub to_ts: i64,
        pub documents: Vec<EncryptedDocument>,
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
        pub cts: Vec<String>,
    }

    impl From<&EncryptedDocument> for DocumentBucketUpdate {
        fn from(doc: &EncryptedDocument) -> Self {
            DocumentBucketUpdate {
                id: doc.id.clone(),
                ts: doc.ts,
                tc: doc.tc,
                hash: doc.hash.clone(),
                keys_ct: doc.keys_ct.clone(),
                cts: doc.cts.to_vec(),
            }
        }
    }

    pub fn restore_from_bucket(pid: &String, dt_id: &String, bucket_update: DocumentBucketUpdate) -> EncryptedDocument {
        EncryptedDocument {
            id: bucket_update.id.clone(),
            dt_id: dt_id.clone(),
            pid: pid.clone(),
            ts: bucket_update.ts,
            tc: bucket_update.tc,
            hash: bucket_update.hash.clone(),
            keys_ct: bucket_update.keys_ct.clone(),
            cts: bucket_update.cts.to_vec(),
        }
    }
}