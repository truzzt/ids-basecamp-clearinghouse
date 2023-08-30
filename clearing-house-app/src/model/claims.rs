use crate::model::constants::{ENV_SHARED_SECRET, SERVICE_HEADER};
use crate::util::ServiceConfig;
use chrono::{Duration, Utc};
use num_bigint::BigUint;
use ring::signature::KeyPair;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use std::env;
use std::fmt::{Display, Formatter};
use anyhow::Context;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChClaims {
    pub client_id: String,
}

impl ChClaims {
    pub fn new(client_id: &str) -> ChClaims {
        ChClaims {
            client_id: client_id.to_string(),
        }
    }
}

impl Display for ChClaims {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.client_id)
    }
}

#[derive(Debug)]
pub enum ChClaimsError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ChClaims {
    type Error = ChClaimsError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one(SERVICE_HEADER) {
            None => Outcome::Failure((Status::BadRequest, ChClaimsError::Missing)),
            Some(token) => {
                debug!("...received service header: {:?}", token);
                let service_config = request.rocket().state::<ServiceConfig>().unwrap();
                match decode_token::<ChClaims>(token, service_config.service_id.as_str()) {
                    Ok(claims) => {
                        debug!("...retrieved claims and succeed");
                        Outcome::Success(claims)
                    }
                    Err(_) => Outcome::Failure((Status::BadRequest, ChClaimsError::Invalid)),
                }
            }
        }
    }
}

pub fn get_jwks(key_path: &str) -> Option<biscuit::jwk::JWKSet<biscuit::Empty>> {
    let keypair = biscuit::jws::Secret::rsa_keypair_from_file(key_path).unwrap();

    if let biscuit::jws::Secret::RsaKeyPair(a) = keypair {
        let pk_modulus = BigUint::from_bytes_be(
            a.as_ref()
                .public_key()
                .modulus()
                .big_endian_without_leading_zero(),
        );
        let pk_e = BigUint::from_bytes_be(
            a.as_ref()
                .public_key()
                .exponent()
                .big_endian_without_leading_zero(),
        );

        let params = biscuit::jwk::RSAKeyParameters {
            n: pk_modulus,
            e: pk_e,
            ..Default::default()
        };

        let common = biscuit::jwk::CommonParameters {
            key_id: get_fingerprint(key_path),
            ..Default::default()
        };

        let jwk = biscuit::jwk::JWK::<biscuit::Empty> {
            common,
            algorithm: biscuit::jwk::AlgorithmParameters::RSA(params),
            additional: biscuit::Empty::default(),
        };

        let jwks = biscuit::jwk::JWKSet::<biscuit::Empty> { keys: vec![jwk] };
        return Some(jwks);
    }
    None
}

pub fn get_fingerprint(key_path: &str) -> Option<String> {
    let keypair = biscuit::jws::Secret::rsa_keypair_from_file(key_path).unwrap();
    if let biscuit::jws::Secret::RsaKeyPair(a) = keypair {
        let pk_modulus = a
            .as_ref()
            .public_key()
            .modulus()
            .big_endian_without_leading_zero()
            .to_vec();
        let pk_e = a
            .as_ref()
            .public_key()
            .exponent()
            .big_endian_without_leading_zero()
            .to_vec();

        let pk = openssh_keys::PublicKey::from_rsa(pk_e, pk_modulus);
        Some(pk.fingerprint())
    } else {
        None
    }
}

pub fn create_service_token(issuer: &str, audience: &str, client_id: &str) -> String {
    let private_claims = ChClaims::new(client_id);
    create_token(issuer, audience, &private_claims)
}

pub fn create_token<T: Display + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>>(
    issuer: &str,
    audience: &str,
    private_claims: &T,
) -> String {
    let signing_secret = match env::var(ENV_SHARED_SECRET) {
        Ok(secret) => biscuit::jws::Secret::Bytes(secret.to_string().into_bytes()),
        Err(_) => {
            panic!(
                "Shared Secret not configured. Please configure environment variable {}",
                ENV_SHARED_SECRET
            );
        }
    };
    let expiration_date = Utc::now() + Duration::minutes(5);

    let claims = biscuit::ClaimsSet::<T> {
        registered: biscuit::RegisteredClaims {
            issuer: Some(issuer.to_string()),
            issued_at: Some(biscuit::Timestamp::from(Utc::now())),
            audience: Some(biscuit::SingleOrMultiple::Single(audience.to_string())),
            expiry: Some(biscuit::Timestamp::from(expiration_date)),
            ..Default::default()
        },
        private: private_claims.clone(),
    };

    // Construct the JWT
    let jwt = biscuit::jws::Compact::new_decoded(
        From::from(biscuit::jws::RegisteredHeader {
            algorithm: biscuit::jwa::SignatureAlgorithm::HS256,
            ..Default::default()
        }),
        claims,
    );

    jwt.into_encoded(&signing_secret)
        .unwrap()
        .unwrap_encoded()
        .to_string()
}

pub fn decode_token<T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>>(
    token: &str,
    audience: &str,
) -> anyhow::Result<T> {
    use biscuit::Presence::Required;
    use biscuit::Validation::Validate;
    info!("START Token validated!");
    let signing_secret = match env::var(ENV_SHARED_SECRET) {
        Ok(secret) => biscuit::jws::Secret::Bytes(secret.to_string().into_bytes()),
        Err(e) => {
            error!(
                "Shared Secret not configured. Please configure environment variable {}",
                ENV_SHARED_SECRET
            );
            return Err(e.into());
        }
    };
    info!("2 Token validated!");
    let jwt: biscuit::jws::Compact<biscuit::ClaimsSet<T>, biscuit::Empty> = biscuit::JWT::<_, biscuit::Empty>::new_encoded(token);
    info!("3 Token validated!");
    let decoded_jwt = match jwt.decode(&signing_secret, biscuit::jwa::SignatureAlgorithm::HS256) {
        Ok(x) => Ok(x),
        Err(e) => {
            error!("Failed to decode token {}", e);
            Err(e)
        }
    }?;
    info!("4 Token validated!");
    let claim_presence_options = biscuit::ClaimPresenceOptions {
        issuer: Required,
        audience: Required,
        issued_at: Required,
        expiry: Required,
        ..Default::default()
    };
    info!("5 Token validated!");
    let val_options = biscuit::ValidationOptions {
        claim_presence_options,
        // issued_at: Validate(Duration::minutes(5)),
        // Issuer is not validated. Wouldn't make much of a difference if we did
        audience: Validate(audience.to_string()),
        ..Default::default()
    };
    info!("6 Token validated!");
    match decoded_jwt.validate(val_options) {
        Ok(o) => Ok(o),
        Err(e) => {
            error!("Failed validating JWT token {}", e);
            Err(e)
        }
    }?;
    info!("Token validated!");
    Ok(decoded_jwt.payload().expect("If this fails we will see it!!").private.clone())
}
