extern crate serde_json;

/// Struct of abstructed Open vSwitch Port
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OvsPort{
    pub name : String,
    pub uuid : String,
    pub mode : OvsPortMode
}

/// Open vSwitch Port Mode  
/// Currently, only support vlan in this library
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OvsPortMode{
    Access(u16),
    Trunk(Vec<u16>)
}

impl OvsPort{
    pub fn new(name:&str, uuid:&str, mode:&OvsPortMode) -> OvsPort{
        OvsPort{
            name: name.to_string(),
            uuid : uuid.to_string(),
            mode : mode.clone()
        }
    }
}

