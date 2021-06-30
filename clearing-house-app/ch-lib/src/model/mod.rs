pub mod constants;
pub mod ids;

pub struct ServerInfo{
    im_version: String,
    agent: String,
    connector: String,
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

#[derive(Serialize, Deserialize)]
pub struct OwnerList{
    pub owners: Vec<String>
}

impl OwnerList{
    pub fn new(owners: Vec<String>) -> OwnerList{
        OwnerList{
            owners,
        }
    }
}