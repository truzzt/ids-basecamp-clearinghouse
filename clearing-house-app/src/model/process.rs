#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Process {
    pub id: String,
    pub owners: Vec<String>,
}

impl Process {
    pub fn new(id: String, owners: Vec<String>) -> Self {
        Self { id, owners }
    }

    pub fn is_authorized(&self, owner: &str) -> bool {
        self.owners.contains(&owner.to_string())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionCounter {
    pub tc: i64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnerList {
    pub owners: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    pub data: biscuit::jws::Compact<DataTransaction, biscuit::Empty>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataTransaction {
    pub timestamp: i64,
    pub process_id: String,
    pub document_id: String,
    pub payload: String,
    pub client_id: String,
    pub clearing_house_version: String,
}

impl biscuit::CompactJson for DataTransaction {}

impl DataTransaction {
    pub fn sign(&self, key_path: &str) -> Receipt {
        let jws = biscuit::jws::Compact::new_decoded(
            biscuit::jws::Header::from_registered_header(biscuit::jws::RegisteredHeader {
                algorithm: biscuit::jwa::SignatureAlgorithm::PS512,
                media_type: None,
                key_id: crate::model::claims::get_fingerprint(key_path),
                ..Default::default()
            }),
            self.clone(),
        );

        let keypair = biscuit::jws::Secret::rsa_keypair_from_file(key_path)
            .unwrap_or_else(|_| panic!("File exists at '{key_path}' and is a valid RSA keypair"));
        debug!("decoded JWS:{:#?}", &jws);
        Receipt {
            data: jws
                .into_encoded(&keypair)
                .expect("Encoded JWS with keypair"),
        }
    }
}
