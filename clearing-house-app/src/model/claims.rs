use crate::model::constants::{ENV_SHARED_SECRET, SERVICE_HEADER};
use crate::AppState;
use anyhow::Context;
use axum::extract::FromRef;
use axum::response::IntoResponse;
use num_bigint::BigUint;
use std::env;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChClaims {
    pub client_id: String,
}

impl ChClaims {
    #[must_use]
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
        }
    }
}

impl std::fmt::Display for ChClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.client_id)
    }
}

pub struct ExtractChClaims(pub ChClaims);

#[async_trait::async_trait]
impl<S> axum::extract::FromRequestParts<S> for ExtractChClaims
    where
        S: Send + Sync,
        AppState: FromRef<S>,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum::extract::State(app_state) =
            axum::extract::State::<AppState>::from_request_parts(parts, state)
                .await
                .map_err(axum::response::IntoResponse::into_response)?;
        if let Some(token) = parts.headers.get(SERVICE_HEADER) {
            let token = token.to_str().map_err(|_| {
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    format!("Invalid token in {SERVICE_HEADER}"),
                )
                    .into_response()
            })?;
            debug!("...received service header: {:?}", token);

            match decode_token::<ChClaims>(token, app_state.service_config.service_id.as_str()) {
                Ok(claims) => {
                    debug!("...retrieved claims and succeed");
                    Ok(ExtractChClaims(claims))
                }
                Err(e) => {
                    error!("...failed to retrieve and validate claims: {}", e);
                    Err((axum::http::StatusCode::BAD_REQUEST, "Invalid token").into_response())
                }
            }
        } else {
            Err((axum::http::StatusCode::BAD_REQUEST, "Missing token").into_response())
        }
    }
}

/// Returns the `JWKSet` for the RSA keypair at `key_path`
///
/// # Panics
///
/// Panics if the key at `key_path` is not a valid RSA keypair or does not exist.
#[must_use]
pub fn get_jwks(key_path: &str) -> Option<biscuit::jwk::JWKSet<biscuit::Empty>> {
    let keypair = biscuit::jws::Secret::rsa_keypair_from_file(key_path)
        .unwrap_or_else(|_| panic!("Failed to load keyfile from path {key_path}"));

    if let biscuit::jws::Secret::RsaKeyPair(a) = keypair {
        use ring::signature::KeyPair;
        let pk_modulus =
            BigUint::from_bytes_be(a.public_key().modulus().big_endian_without_leading_zero());
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

/// Returns the fingerprint of the RSA keypair at `key_path`
///
/// # Panics
///
/// Panics if the key at `key_path` is not a valid RSA keypair or does not exist.
#[must_use]
pub fn get_fingerprint(key_path: &str) -> Option<String> {
    use ring::signature::KeyPair;
    let keypair = biscuit::jws::Secret::rsa_keypair_from_file(key_path)
        .unwrap_or_else(|_| panic!("File exists at '{key_path}' and is a valid RSA keypair"));
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

/// Creates a JWT token with the given `issuer`, `audience` and `private_claims`
///
/// # Panics
///
/// Panics if the `ENV_SHARED_SECRET` is not set
pub fn create_token<
    T: std::fmt::Display + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
>(
    issuer: &str,
    audience: &str,
    private_claims: &T,
) -> String {
    let secret = env::var(ENV_SHARED_SECRET).unwrap_or_else(|_| panic!("Shared Secret not configured. Please configure environment variable {ENV_SHARED_SECRET}"));
    let signing_secret = biscuit::jws::Secret::Bytes(secret.to_string().into_bytes());
    let expiration_date = chrono::Utc::now() + chrono::TimeDelta::try_minutes(5).expect("5 minutes is a valid time delta");

    let claims = biscuit::ClaimsSet::<T> {
        registered: biscuit::RegisteredClaims {
            issuer: Some(issuer.to_string()),
            issued_at: Some(biscuit::Timestamp::from(chrono::Utc::now())),
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
        .expect("Encoded JWT with the signing secret")
        .unwrap_encoded()
        .to_string()
}

/// Decodes the given `token` and validates it against the given `audience`
///
/// # Errors
///
/// Returns an error if the token is invalid or the audience is not as expected.
pub fn decode_token<T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>>(
    token: &str,
    audience: &str,
) -> anyhow::Result<T> {
    use biscuit::Presence::Required;
    use biscuit::Validation::Validate;
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
    let jwt: biscuit::jws::Compact<biscuit::ClaimsSet<T>, biscuit::Empty> =
        biscuit::JWT::<_, biscuit::Empty>::new_encoded(token);
    let decoded_jwt = match jwt.decode(&signing_secret, biscuit::jwa::SignatureAlgorithm::HS256) {
        Ok(x) => Ok(x),
        Err(e) => {
            error!("Failed to decode token {}", e);
            Err(e)
        }
    }?;
    let claim_presence_options = biscuit::ClaimPresenceOptions {
        issuer: Required,
        audience: Required,
        issued_at: Required,
        expiry: Required,
        ..Default::default()
    };
    let val_options = biscuit::ValidationOptions {
        claim_presence_options,
        // Issuer is not validated. Wouldn't make much of a difference if we did
        // issued_at: Validate(Duration::minutes(5)),
        audience: Validate(audience.to_string()),
        ..Default::default()
    };

    decoded_jwt
        .validate(val_options)
        .with_context(|| "Failed validating JWT")?;
    Ok(decoded_jwt.payload()?.private.clone())
}

#[cfg(test)]
mod test {
    #[test]
    fn get_fingerprint() {
        let fingerprint = super::get_fingerprint("keys/private_key.der").expect("Fingerprint can be generated");
        assert_eq!(fingerprint, "Qra//29Frxbj5hh5Azef+G36SeiOm9q7s8+w8uGLD28");
    }
}
