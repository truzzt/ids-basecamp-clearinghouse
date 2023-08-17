pub mod errors {
    use error_chain::error_chain;
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        foreign_links {
            Conversion(std::num::TryFromIntError);
            Figment(rocket::figment::Error);
            HexError(hex::FromHexError);
            Io(::std::io::Error) #[cfg(unix)];
            Mongodb(mongodb::error::Error);
            MongodbBson(mongodb::bson::de::Error);
            SerdeJson(serde_json::error::Error);
            Uft8Error(std::string::FromUtf8Error);
            BiscuitError(biscuit::errors::Error);
            EnvVariable(::std::env::VarError);
        }
    }
}