use crate::model::constants::SPLIT_CT;
use crate::model::crypto::{KeyEntry, KeyMap};
use crate::util::new_uuid;
use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::{Aes256GcmSiv, KeyInit};
use chrono::Local;
use generic_array::GenericArray;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct DocumentPart {
    pub name: String,
    pub content: String,
}

impl DocumentPart {
    pub fn new(name: String, content: String) -> DocumentPart {
        DocumentPart { name, content }
    }

    pub fn encrypt(&self, key: &[u8], nonce: &[u8]) -> anyhow::Result<Vec<u8>> {
        const EXP_KEY_SIZE: usize = 32;
        const EXP_NONCE_SIZE: usize = 12;
        // check key size
        if key.len() != EXP_KEY_SIZE {
            error!(
                "Given key has size {} but expected {} bytes",
                key.len(),
                EXP_KEY_SIZE
            );
            anyhow::bail!("Incorrect key size")
        }
        // check nonce size
        else if nonce.len() != EXP_NONCE_SIZE {
            error!(
                "Given nonce has size {} but expected {} bytes",
                nonce.len(),
                EXP_NONCE_SIZE
            );
            anyhow::bail!("Incorrect nonce size")
        } else {
            let key = GenericArray::from_slice(key);
            let nonce = GenericArray::from_slice(nonce);
            let cipher = Aes256GcmSiv::new(key);

            let pt = format_pt_for_storage(&self.name, &self.content);
            match cipher.encrypt(nonce, pt.as_bytes()) {
                Ok(ct) => Ok(ct),
                Err(e) => anyhow::bail!("Error while encrypting {}", e),
            }
        }
    }

    pub fn decrypt(key: &[u8], nonce: &[u8], ct: &[u8]) -> anyhow::Result<DocumentPart> {
        let key = GenericArray::from_slice(key);
        let nonce = GenericArray::from_slice(nonce);
        let cipher = Aes256GcmSiv::new(key);

        match cipher.decrypt(nonce, ct) {
            Ok(pt) => {
                let pt = String::from_utf8(pt)?;
                let (name, content) = restore_pt_no_dt(&pt)?;
                Ok(DocumentPart::new(name, content))
            }
            Err(e) => {
                anyhow::bail!("Error while decrypting: {}", e)
            }
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Document {
    #[serde(default = "new_uuid")]
    pub id: String,
    pub dt_id: String,
    pub pid: String,
    pub ts: i64,
    pub parts: Vec<DocumentPart>,
}

/// Documents should have a globally unique id, setting the id manually is discouraged.
impl Document {
    pub fn create_uuid() -> String {
        Uuid::new_v4().hyphenated().to_string()
    }

    // each part is encrypted using the part specific key from the key map
    // the hash is set to "0". Chaining is not done here.
    pub fn encrypt(&self, key_map: KeyMap) -> anyhow::Result<EncryptedDocument> {
        debug!("encrypting document of doc_type {}", self.dt_id);
        let mut cts = vec![];

        let keys = key_map.keys;
        let key_ct = match key_map.keys_enc {
            Some(ct) => hex::encode(ct),
            None => {
                anyhow::bail!("Missing key ct");
            }
        };

        for part in self.parts.iter() {
            // check if there's a key for this part
            let key_entry = match keys.get(&part.name) {
                Some(key_entry) => key_entry,
                None => {
                    error!("Missing key for part '{}'", &part.name);
                    anyhow::bail!("Missing key for part '{}'", &part.name);
                }
            };
            // Encrypt part
            let ct_string = match part.encrypt(key_entry.key.as_slice(), key_entry.nonce.as_slice())
            {
                Ok(ct) => hex::encode_upper(ct),
                Err(e) => {
                    error!("Error while encrypting: {}", e);
                    anyhow::bail!("Error while encrypting: {}", e);
                }
            };

            // key entry id is needed for decryption
            cts.push(format!("{}::{}", key_entry.id, ct_string));
        }
        cts.sort();

        Ok(EncryptedDocument::new(
            self.id.clone(),
            self.pid.clone(),
            self.dt_id.clone(),
            self.ts,
            key_ct,
            cts,
        ))
    }

    pub fn get_parts_map(&self) -> HashMap<String, String> {
        let mut p_map = HashMap::with_capacity(self.parts.len());
        for part in self.parts.iter() {
            p_map.insert(part.name.clone(), part.content.clone());
        }
        p_map
    }

    pub fn new(pid: String, dt_id: String, parts: Vec<DocumentPart>) -> Document {
        Document {
            id: Document::create_uuid(),
            dt_id,
            pid,
            ts: Local::now().timestamp(),
            parts,
        }
    }

    fn restore(
        id: String,
        pid: String,
        dt_id: String,
        ts: i64,
        parts: Vec<DocumentPart>,
    ) -> Document {
        Document {
            id,
            dt_id,
            pid,
            ts,
            parts,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptedDocument {
    pub id: String,
    pub pid: String,
    pub dt_id: String,
    pub ts: i64,
    pub keys_ct: String,
    pub cts: Vec<String>,
}

impl EncryptedDocument {
    /// Note: KeyMap keys need to be KeyEntry.ids in this case
    // Decryption is done without checking the hashes. Do this before calling this method
    pub fn decrypt(&self, keys: HashMap<String, KeyEntry>) -> anyhow::Result<Document> {
        let mut pts = vec![];
        for ct in self.cts.iter() {
            let ct_parts = ct.split(SPLIT_CT).collect::<Vec<&str>>();
            if ct_parts.len() != 2 {
                anyhow::bail!("Integrity violation! Ciphertexts modified");
            }
            // get key and nonce
            let key_entry = keys.get(ct_parts[0]);
            if let Some(key_entry) = key_entry {
                let key = key_entry.key.as_slice();
                let nonce = key_entry.nonce.as_slice();

                // get ciphertext
                //TODO: use error_chain?
                let ct = hex::decode(ct_parts[1])?;

                // decrypt
                match DocumentPart::decrypt(key, nonce, ct.as_slice()) {
                    Ok(part) => pts.push(part),
                    Err(e) => {
                        anyhow::bail!("Error while decrypting: {}", e);
                    }
                }
            } else {
                anyhow::bail!("Key for id '{}' does not exist!", ct_parts[0]);
            }
        }

        Ok(Document::restore(
            self.id.clone(),
            self.pid.clone(),
            self.dt_id.clone(),
            self.ts,
            pts,
        ))
    }

    pub fn new(
        id: String,
        pid: String,
        dt_id: String,
        ts: i64,
        keys_ct: String,
        cts: Vec<String>,
    ) -> EncryptedDocument {
        EncryptedDocument {
            id,
            pid,
            dt_id,
            ts,
            keys_ct,
            cts,
        }
    }
}

/// companion to format_pt_for_storage_no_dt
pub fn restore_pt_no_dt(pt: &str) -> anyhow::Result<(String, String)> {
    trace!("Trying to restore plain text");
    let vec: Vec<&str> = pt.split(SPLIT_CT).collect();
    if vec.len() != 2 {
        anyhow::bail!("Could not restore plaintext");
    }
    Ok((String::from(vec[0]), String::from(vec[1])))
}

/// formats the pt before encryption
fn format_pt_for_storage(field_name: &str, pt: &str) -> String {
    format!("{}{}{}", field_name, SPLIT_CT, pt)
}
