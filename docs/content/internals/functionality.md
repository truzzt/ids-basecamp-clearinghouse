# Functionality


## Logging a message

The logging service (as an entity inside the remaining clearinghouse-app) is responsible for orchestrating the flow between document service and keyring service:

When logging a message, the message consists of two parts, originating from the IDS communication structure. There is a `header` and a `payload`.

The logging service creates a process id (if not exists) and checks the authorization.

After all prerequisites are checked and completed, the logging-service  merges `header` and `payload` into a Document starts to get the transaction counter and assigns it to the Document.

Now the document service comes into play: First checking if the document exists already, then requesting the keyring service to generate a key map for the document. The key map is then used to encrypt the document (back in the document service) and then the document is stored in the database.

Finally the transaction counter is incremented and a reciept is signed and send back to the Clearinghouse-EDC.

### Encryption

There is a randomly generated Master Key stored in the database.

Each document has a number of fields. For each document a random secret is generated. This secret is used to derive multiple secrets with the HKDF Algorithm from the original secret. These derived secrets are used to encrypt the fields of the document with AES-256-GCM-SIV.

The original secret is encrypted also with AES-256-GCM-SIV with a derived key from the Master Key and stored in the database alongside the Document.

### Detailed internal diagram

```d2
log: fn log {
  gp: fn db.get_process
  ia: fn db.is_authorized
  sp: fn db.store_process
  de: process exists? {
    shape: diamond
  }

  gp -> de
  de -> sp: No
  de -> ia: Yes
  sp -> ia
}

lm: fn log_message {

  gt: fn db.get_transaction_counter
  df: Document::from(message)
  ced: fn doc_api.create_encrypted_document {
    ed: fn db.exists_document
    gk: fn key_api.generate_keys {
      gm: fn db.get_master_key
      gdt: fn db.get_document_type
      gkm: fn generate_key_map

      gm -> gdt
      gdt -> gkm

    }
    de: fn doc.encrypt
    pt: fn db.get_document_with_previous_transaction_counter
    ad: fn db.add_document

    ed -> gk
    gk -> de
    de -> pt
    pt -> ad
  }
  itc: fn db.increment_transaction_counter

  df -> gt
  gt -> ced
  ced -> itc
}

log -> lm

lm.ced.gk.gkm -> gkm

gkm: fn generate_key_map {
  ik: fn initialize_kdf
  dk: fn derive_key_map
  rk: fn restore_kdf
  ke: fn kdf.expand
  es: fn encrypt_secret

  ik -> dk
  dk -> rk
  rk -> ke
  ke -> es

}
```