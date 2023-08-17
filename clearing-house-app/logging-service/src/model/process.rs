#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Process {
    pub id: String,
    pub owners: Vec<String>,
}

impl Process {
    pub fn new(id: String, owners: Vec<String>) -> Process {
        Process {
            id,
            owners
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionCounter{
    pub tc: i64
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnerList{
    pub owners: Vec<String>
}

impl OwnerList{
    pub fn new(owners: Vec<String>) -> OwnerList{
        OwnerList{
            owners,
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    pub data: biscuit::jws::Compact<DataTransaction, biscuit::Empty>
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataTransaction {
    pub transaction_id: String,
    pub timestamp: i64,
    pub process_id: String,
    pub document_id: String,
    pub payload: String,
    pub chain_hash: String,
    pub client_id: String,
    pub clearing_house_version: String,
}

impl biscuit::CompactJson for DataTransaction{}

impl DataTransaction{
    pub fn sign(&self, key_path: &str) -> Receipt{
        use crate::model::claims::get_fingerprint;

        let jws = biscuit::jws::Compact::new_decoded(biscuit::jws::Header::from_registered_header(biscuit::jws::RegisteredHeader{
            algorithm: biscuit::jwa::SignatureAlgorithm::PS512,
            media_type: None,
            key_id: get_fingerprint(key_path),
            ..Default::default()}), self.clone());

        let keypair = biscuit::jws::Secret::rsa_keypair_from_file(key_path).unwrap();
        println!("decoded JWS:{:#?}", &jws);
        Receipt{
            data: jws.into_encoded(&keypair).unwrap()
        }
    }
}

// convenience method for testing
impl From<Receipt> for DataTransaction{
    fn from(r: Receipt) -> Self {
        match r.data.unverified_payload(){
            Ok(d) => d.clone(),
            Err(e) => {
                println!("Error occured: {:#?}", e);
                DataTransaction{

                    transaction_id: "error".to_string(),
                    timestamp: 0,
                    process_id: "error".to_string(),
                    document_id: "error".to_string(),
                    payload: "error".to_string(),
                    chain_hash: "error".to_string(),
                    client_id: "error".to_string(),
                    clearing_house_version: "error".to_string(),
                }
            }
        }
    }
}