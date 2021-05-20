pub mod constants;
pub mod ids;

pub struct ServerInfo{
    im_version: String,
    connector: String,
}

impl ServerInfo{
    pub fn new(im_version: String, connector: String) -> ServerInfo{
        ServerInfo{
            im_version,
            connector,
        }
    }
}