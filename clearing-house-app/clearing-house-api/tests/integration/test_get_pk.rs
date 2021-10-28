use crate::CH_API;
use core_lib::errors::*;
use crate::ch_api_client::ClearingHouseApiClient;
use core_lib::api::ApiClient;

#[test]
fn test_get_pk() -> Result<()>{
    // configure client_api
    let ch_api: ClearingHouseApiClient = ApiClient::new(CH_API);

    // run the test
    let jwks = ch_api.get_pk()?;
    println!("jwks: {:#?}", jwks);
    // The returned payload should contain an RSA key
    if let biscuit::jwk::AlgorithmParameters::RSA(params) = &jwks.keys[0].algorithm{
        println!("e: {:#?}", params.e);
        println!("n: {:#?}", params.n);
    }

    Ok(())
}
