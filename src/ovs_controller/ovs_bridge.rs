
use serde::{Serialize, Deserialize};

use crate::ovs_controller::ovs_port::OvsPort;

/// Struct of abstructed Open vSwitch Bridge
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OvsBridge{
    pub name : String,
    pub uuid : String,
    pub ports : Vec<OvsPort>
}

impl OvsBridge{
    pub fn new(name:&str, uuid:&str) -> OvsBridge{
        OvsBridge{
            name: name.to_string(),
            uuid : uuid.to_string(),
            ports : Vec::new()
        }
    }
}


