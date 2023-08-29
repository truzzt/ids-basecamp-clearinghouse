pub const CONTENT_TYPE: &str = "Content-Type";
pub const APPLICATION_JSON: &str = "application/json";
pub const SIGNING_KEY: &str = "signing_key";

pub const CLEARING_HOUSE_URL: &str = "clearing_house_url";
pub const ROCKET_CLEARING_HOUSE_BASE_API: &str = "/messages";
pub const ROCKET_PK_API: &str = "/";
pub const ROCKET_QUERY_API: &str = "/query";
pub const ROCKET_LOG_API: &str = "/log";
pub const ROCKET_BLOCKCHAIN_BASE_API: &str = "/blockchain";

// From core_lib

// definition of daps constants
pub const DAPS_AUD: &str = "idsc:IDS_CONNECTORS_ALL";
pub const DAPS_JWKS: &str = ".well-known/jwks.json";
pub const DAPS_KID: &str = "default";
pub const DAPS_AUTHHEADER: &str = "Authorization";
pub const DAPS_AUTHBEARER: &str = "Bearer";
pub const DAPS_CERTIFICATES: &str = "certs";

// definition of custom headers
pub const SERVICE_HEADER: &str = "CH-SERVICE";

// definition of config parameters (in config files)
pub const DATABASE_URL: &str = "database_url";
pub const DOCUMENT_API_URL: &str = "document_api_url";
pub const KEYRING_API_URL: &str = "keyring_api_url";
pub const DAPS_API_URL: &str = "daps_api_url";
pub const CLEAR_DB: &str = "clear_db";

// define here the config options from environment variables
pub const ENV_API_LOG_LEVEL: &str = "API_LOG_LEVEL";
pub const ENV_SHARED_SECRET: &str = "SHARED_SECRET";
pub const ENV_DOCUMENT_SERVICE_ID: &str = "SERVICE_ID_DOC";
pub const ENV_KEYRING_SERVICE_ID: &str = "SERVICE_ID_KEY";
pub const ENV_LOGGING_SERVICE_ID: &str = "SERVICE_ID_LOG";

// definition of rocket mount points
pub const ROCKET_DOC_API: &str = "/doc";
pub const ROCKET_DOC_TYPE_API: &str = "/doctype";
pub const ROCKET_POLICY_API: &str = "/policy";
pub const ROCKET_STATISTICS: &str = "/statistics";
pub const ROCKET_PROCESS_API: &str = "/process";
pub const ROCKET_KEYRING_API: &str = "/keyring";
pub const ROCKET_USER_API: &str = "/users";

// definition of service names
pub const DOCUMENT_DB_CLIENT: &str = "document-api";
pub const KEYRING_DB_CLIENT: &str = "keyring-api";
pub const PROCESS_DB_CLIENT: &str = "logging-service";

// definition of table names
pub const MONGO_DB: &str = "ch_ids";
pub const DOCUMENT_DB: &str = "document";
pub const KEYRING_DB: &str = "keyring";
pub const PROCESS_DB: &str = "process";
pub const MONGO_COLL_DOCUMENTS: &str = "documents";
pub const MONGO_COLL_DOCUMENT_BUCKET: &str = "document_bucket";
pub const MONGO_COLL_DOC_TYPES: &str = "doc_types";
pub const MONGO_COLL_DOC_PARTS: &str = "parts";
pub const MONGO_COLL_PROCESSES: &str = "processes";
pub const MONGO_COLL_TRANSACTIONS: &str = "transactions";
pub const MONGO_COLL_MASTER_KEY: &str = "keys";

// definition of database fields
pub const MONGO_ID: &str = "id";
pub const MONGO_MKEY: &str = "msk";
pub const MONGO_PID: &str = "pid";
pub const MONGO_DT_ID: &str = "dt_id";
pub const MONGO_NAME: &str = "name";
pub const MONGO_OWNER: &str = "owner";
pub const MONGO_TS: &str = "ts";
pub const MONGO_TC: &str = "tc";

pub const MONGO_DOC_ARRAY: &str = "documents";
pub const MONGO_COUNTER: &str = "counter";
pub const MONGO_FROM_TS: &str = "from_ts";
pub const MONGO_TO_TS: &str = "to_ts";

// definition of default database values
pub const DEFAULT_PROCESS_ID: &str = "default";
pub const MAX_NUM_RESPONSE_ENTRIES: u64 = 1000;
pub const DEFAULT_NUM_RESPONSE_ENTRIES: u64 = 100;

pub const DEFAULT_DOC_TYPE: &str = "IDS_MESSAGE";

// split string symbols for vec_to_string and string_to_vec
pub const SPLIT_QUOTE: &str = "'";
pub const SPLIT_SIGN: &str = "~";
pub const SPLIT_CT: &str = "::";

// definition of file names and folders
pub const FOLDER_DB: &str = "db_init";
pub const FOLDER_DATA: &str = "data";
pub const FILE_DOC: &str = "document.json";
pub const FILE_DEFAULT_DOC_TYPE: &str = "init_db/default_doc_type.json";

// definition of special document parts
pub const PAYLOAD_PART: &str = "payload";
