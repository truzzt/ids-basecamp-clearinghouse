use crate::crypto::generate_random_seed;
use crate::util::new_uuid;
use hkdf::Hkdf;
use sha2::Sha256;
use std::collections::HashMap;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct MasterKey {
    pub id: String,
    pub key: String,
    pub salt: String,
}

impl MasterKey {
    pub fn new(id: String, key: String, salt: String) -> MasterKey {
        MasterKey { id, key, salt }
    }

    pub fn new_random() -> MasterKey {
        let key_salt = generate_random_seed();
        let ikm = generate_random_seed();
        let (master_key, _) = Hkdf::<Sha256>::extract(Some(&key_salt), &ikm);

        MasterKey::new(
            new_uuid(),
            hex::encode_upper(master_key),
            hex::encode_upper(generate_random_seed()),
        )
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct KeyEntry {
    pub id: String,
    pub key: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl KeyEntry {
    pub fn new(id: String, key: Vec<u8>, nonce: Vec<u8>) -> KeyEntry {
        KeyEntry { id, key, nonce }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct KeyMap {
    pub keys: HashMap<String, KeyEntry>,
    pub keys_enc: Option<Vec<u8>>,
}

impl KeyMap {
    pub fn new(keys: HashMap<String, KeyEntry>, keys_enc: Option<Vec<u8>>) -> KeyMap {
        KeyMap { keys, keys_enc }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyCt {
    pub id: String,
    pub ct: String,
}

impl KeyCt {
    pub fn new(id: String, ct: String) -> KeyCt {
        KeyCt { id, ct }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyCtList {
    pub dt: String,
    pub cts: Vec<KeyCt>,
}

impl KeyCtList {
    pub fn new(dt: String, cts: Vec<KeyCt>) -> KeyCtList {
        KeyCtList { dt, cts }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyMapListItem {
    pub id: String,
    pub map: KeyMap,
}

impl KeyMapListItem {
    pub fn new(id: String, map: KeyMap) -> KeyMapListItem {
        KeyMapListItem { id, map }
    }
}
