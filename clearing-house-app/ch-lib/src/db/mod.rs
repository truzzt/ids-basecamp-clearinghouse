use mongodb::{
    doc,
    bson,
    coll::options::{
        IndexOptions,
        FindOptions,
    },
    Bson,
    Client,
    ThreadedClient,
    CommandType,
    db::{
        Database,
        ThreadedDatabase
    }
};

use core_lib::constants::{MONGO_DB, MONGO_ID, MONGO_OWNER, MONGO_COLL_PROCESSES};
use core_lib::db::{DataStoreApi, drop_table, create_table};
use core_lib::errors::*;
use serde_json::json;
use core_lib::model::process::Process;

#[derive(Clone)]
pub struct ProcessStore {
    client: Client,
    database: Database
}

impl DataStoreApi for ProcessStore {

    fn clean_db(&self) -> Result<bool> {
        info!("cleaning mongoDBs:");
        // create the collections
        let mut success = true;
        success = success & drop_table(&self.database, MONGO_COLL_PROCESSES)?;
        return Ok(success);
    }

    //TODO: check the indexes
    fn create_indexes(&self) -> Result<bool> {
        info!("creating indexes");
        let mut success = true;
        let name_index = doc! {
            "name": 1
        };
        let mut index_unique = IndexOptions::new();
        index_unique.unique = Some(true);
        success = success & create_table(&self.database, MONGO_COLL_PROCESSES, name_index.clone(), None)?;
        return Ok(success);
    }

    fn db_exists(&self) -> Result<bool> {
        Ok(self.database.list_collections(None)?.count() > 0)
    }

    fn metrics(&self) -> Result<serde_json::Value> {
        let cmd = doc! { "serverStatus": 1, "repl": 0, "metrics": 1, "locks": 0 };
        let result = self.database.command(cmd, CommandType::Suppressed, None)?;
        Ok(json!(Bson::Document(result)))
    }

    fn new(host: &str, port: u16) -> ProcessStore{
        let client = Client::connect(host, port)
            .expect("Failed to initialize mongodb client.");
        ProcessStore {
            client: client.clone(),
            database: client.db(MONGO_DB)
        }
    }

    fn statistics(&self) -> Result<serde_json::Value> {
        let cmd = doc! { "dbStats": 1, "scale": 1024 };
        let result = self.database.command(cmd, CommandType::Suppressed, None)?;
        Ok(json!(Bson::Document(result)))
    }
}

impl ProcessStore {
    pub fn get_processes(&self) -> Result<Vec<Process>> {
        // The model type collection
        let mut ret = vec![];
        let coll = self.database.collection(MONGO_COLL_PROCESSES);
        let mut options = FindOptions::new();
        options.sort = Some(doc!{ MONGO_ID: 1});
        return match coll.find(None, Some(options)) {
            Ok(mut cursor) => {
                loop{
                    if cursor.has_next()?{
                        // we checked has_next() so unwrap() is safe to get to the Result
                        let dt = cursor.next().unwrap()?;
                        let process = mongodb::from_bson::<Process>(Bson::Document(dt))?;
                        ret.push(process);
                    }
                    else{
                        break;
                    }
                }
                Ok(ret)
            }
            Err(e) => Err(format!("no processes found: {}", e.to_string()).into())
        }
    }

    pub fn delete_process(&self, pid: &String) -> Result<bool> {
        // The model type collection
        let coll = self.database.collection(MONGO_COLL_PROCESSES);
        let result = coll.delete_many(doc! { MONGO_ID: pid }, None)?;
        if result.deleted_count >= 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// checks if the id exits
    pub fn exists_process(&self, pid: &String) -> Result<bool> {
        // The model type collection
        let coll = self.database.collection(MONGO_COLL_PROCESSES);
        let result = coll.find_one(Some(doc!{ MONGO_ID: pid }), None)?;
        return match result {
            Some(r) => {
                // We check if we found a valid model type. This is not entirely necessary at this point, because there
                // should only be valid model types in this collection...
                let _doc_type = mongodb::from_bson::<Process>(Bson::Document(r))?;
                Ok(true)
            },
            None => {
                debug!("process {} does not exist!", &pid);
                Ok(false)
            }
        };
    }

    pub fn get_process(&self, pid: &String) -> Result<Process> {
        let coll = self.database.collection(MONGO_COLL_PROCESSES);
        return match coll.find_one(Some(doc!{ MONGO_ID: pid }), None)? {
            Some(r) => {
                Ok(mongodb::from_bson::<Process>(Bson::Document(r))?)
            },
            None => {
                Err(format!("process {} was not found", pid).into())
            }
        }
    }

    pub fn is_authorized(&self, user: &String, pid: &String) -> Result<bool>{
        debug!("checking if user '{}' is authorized to access '{}'", user, pid);
        return match self.get_process(&pid){
            Ok(process) => {
                let authorized = process.owners.iter().any(|o| {
                    debug!("found owner {}", o);
                    user.eq(o)
                });
                Ok(authorized)
            }
            _ => {
                Err(format!("User '{}' could not be authorized", &user).into())
            }
        }
    }

    // store process in db
    pub fn store_process(&self, process: Process) -> Result<String> {
        let coll = self.database.collection(MONGO_COLL_PROCESSES);
        let serialized_doc = mongodb::to_bson(&process)?;
        match serialized_doc.as_document() {
            Some(dt) => {
                let _v = coll.insert_one(dt.clone(), None)?;
                debug!("inserted process {:?}: {:?}", &process.id, _v);
                Ok(process.id)
            },
            _ => bail!("conversion to process failed!"),
        }
    }
}