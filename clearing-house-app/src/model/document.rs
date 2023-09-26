use crate::model::constants::SPLIT_CT;
use crate::model::crypto::{KeyEntry, KeyMap};
use crate::model::util::new_uuid;
use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::{Aes256GcmSiv, KeyInit};
use base64::Engine;
use blake2_rfc::blake2b::Blake2b;
use chrono::Local;
use generic_array::GenericArray;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct DocumentPart {
    pub name: String,
    pub content: Option<String>,
}

impl DocumentPart {
    pub fn new(name: String, content: Option<String>) -> DocumentPart {
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

            match &self.content {
                Some(pt) => {
                    let pt = format_pt_for_storage(&self.name, pt);
                    match cipher.encrypt(nonce, pt.as_bytes()) {
                        Ok(ct) => Ok(ct),
                        Err(e) => anyhow::bail!("Error while encrypting {}", e),
                    }
                }
                None => {
                    error!("Tried to encrypt empty document part.");
                    anyhow::bail!("Nothing to encrypt");
                }
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
                Ok(DocumentPart::new(name, Some(content)))
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
    pub tc: i64,
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
            if part.content.is_none() {
                // no content, so we skip this one
                continue;
            }
            // check if there's a key for this part
            if !keys.contains_key(&part.name) {
                error!("Missing key for part '{}'", &part.name);
                anyhow::bail!("Missing key for part '{}'", &part.name);
            }
            // get the key for this part
            let key_entry = keys.get(&part.name).unwrap();
            let ct = part.encrypt(key_entry.key.as_slice(), key_entry.nonce.as_slice());
            if ct.is_err() {
                warn!("Encryption error. No ct received!");
                anyhow::bail!("Encryption error. No ct received!");
            }
            let ct_string = hex::encode_upper(ct.unwrap());

            // key entry id is needed for decryption
            cts.push(format!("{}::{}", key_entry.id, ct_string));
        }
        cts.sort();

        Ok(EncryptedDocument::new(
            self.id.clone(),
            self.pid.clone(),
            self.dt_id.clone(),
            self.ts,
            self.tc,
            key_ct,
            cts,
        ))
    }

    pub fn get_formatted_tc(&self) -> String {
        format_tc(self.tc)
    }

    pub fn get_parts_map(&self) -> HashMap<String, Option<String>> {
        let mut p_map = HashMap::new();
        for part in self.parts.iter() {
            p_map.insert(part.name.clone(), part.content.clone());
        }
        p_map
    }

    pub fn new(pid: String, dt_id: String, tc: i64, parts: Vec<DocumentPart>) -> Document {
        Document {
            id: Document::create_uuid(),
            dt_id,
            pid,
            ts: Local::now().timestamp(),
            tc,
            parts,
        }
    }

    fn restore(
        id: String,
        pid: String,
        dt_id: String,
        ts: i64,
        tc: i64,
        parts: Vec<DocumentPart>,
    ) -> Document {
        Document {
            id,
            dt_id,
            pid,
            ts,
            tc,
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
    pub tc: i64,
    pub hash: String,
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
            if key_entry.is_none() {
                anyhow::bail!("Key for id '{}' does not exist!", ct_parts[0]);
            }
            let key = key_entry.unwrap().key.as_slice();
            let nonce = key_entry.unwrap().nonce.as_slice();

            // get ciphertext
            //TODO: use error_chain?
            let ct = hex::decode(ct_parts[1]).unwrap();

            // decrypt
            match DocumentPart::decrypt(key, nonce, ct.as_slice()) {
                Ok(part) => pts.push(part),
                Err(e) => {
                    anyhow::bail!("Error while decrypting: {}", e);
                }
            }
        }

        Ok(Document::restore(
            self.id.clone(),
            self.pid.clone(),
            self.dt_id.clone(),
            self.ts,
            self.tc,
            pts,
        ))
    }

    pub fn get_formatted_tc(&self) -> String {
        format_tc(self.tc)
    }

    pub fn hash(&self) -> String {
        let mut hasher = Blake2b::new(64);

        hasher.update(self.id.as_bytes());
        hasher.update(self.pid.as_bytes());
        hasher.update(self.dt_id.as_bytes());
        hasher.update(self.get_formatted_tc().as_bytes());
        hasher.update(self.ts.to_string().as_bytes());
        hasher.update(self.hash.as_bytes());
        hasher.update(self.keys_ct.as_bytes());
        let mut cts = self.cts.clone();
        cts.sort();
        for ct in cts.iter() {
            hasher.update(ct.as_bytes());
        }

        let res = base64::engine::general_purpose::STANDARD.encode(hasher.finalize());
        debug!("hashed cts: '{}'", &res);
        res
    }

    pub fn new(
        id: String,
        pid: String,
        dt_id: String,
        ts: i64,
        tc: i64,
        keys_ct: String,
        cts: Vec<String>,
    ) -> EncryptedDocument {
        EncryptedDocument {
            id,
            pid,
            dt_id,
            ts,
            tc,
            hash: String::from("0"),
            keys_ct,
            cts,
        }
    }
}

/// companion to format_pt_for_storage
pub fn restore_pt(pt: &str) -> anyhow::Result<(String, String, String)> {
    trace!("Trying to restore plain text");
    let vec: Vec<&str> = pt.split(SPLIT_CT).collect();
    if vec.len() != 3 {
        anyhow::bail!("Could not restore plaintext");
    }
    Ok((
        String::from(vec[0]),
        String::from(vec[1]),
        String::from(vec[2]),
    ))
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

fn format_tc(tc: i64) -> String {
    format!("{:08}", tc)
}

#[cfg(test)]
mod test {
    /// Purpose of this test case: The `base64::encode` function has been deprecated in favor of
    /// `base64::engine::Engine::encode`. This test case ensures that the new function works as
    /// expected.
    #[test]
    fn hash() {
        let doc = super::EncryptedDocument::new(
            String::from("id"),
            String::from("pid"),
            String::from("dt_id"),
            42,
            12,
            String::from("keys_ct"),
            vec![String::from("ct1"), String::from("ct2")],
        );

        let hash = doc.hash();
        assert_eq!("X/BsEutzaPbi555duyusiD9z5aUCwE7oNIMteMtdYLEAqJ7FJ0Ln13J3t1Qw8MMJhLCb9rRE8bRbqHtV4mYqRA==", hash);
    }
}
