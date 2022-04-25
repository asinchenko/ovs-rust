/*
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PortReqParam{
    String(String),
    OvsPortInsert(OvsPortInsert)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OvsPortInsert{
    #[serde(rename="name-uuid")]
    pub uuid: String,
    op: String,
    table: String,
    row:OvsPortInsertRow,
    
    #[serde(skip)]
    name:String
}

#[derive(Serialize, Deserialize, Debug)]
struct OvsPortInsertRow{
    name:String,
    interfaces:Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortReq{
    pub method : String,
    pub params : Vec<PortReqParam>
}

impl OvsPortInsert{
    pub fn new(interface_name:&str)->OvsPortInsert{
        let tmp_uuid = Uuid::new_v4();
        
        OvsPortInsert{
            uuid:tmp_uuid.to_string(),
            op:"insert".to_string(),
            table:"Port".to_string(),
            row:OvsPortInsertRow::new(&tmp_uuid.to_string(), interface_name),
            name:interface_name.to_string()
        }
    }
}

impl OvsPortInsertRow{
    pub fn new(interface_uuid:&str, interface_name:&str)->OvsPortInsertRow{
        OvsPortInsertRow{
            name:interface_uuid.to_string(),
            interfaces:vec!(interface_uuid.to_string(), interface_name.to_string())
        }
    }
}
*/
