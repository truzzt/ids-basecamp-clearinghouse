pub mod constants;
pub mod ids;

pub struct ServerInfo{
    im_version: String,
    agent: String,
    connector: String,
}

#[derive(Serialize, Deserialize)]
pub struct OwnerList{
    pub owners: Vec<String>
}

impl ServerInfo{
    pub fn new(im_version: String, connector: String, agent: String) -> ServerInfo{
        ServerInfo{
            im_version,
            agent,
            connector,
        }
    }
}