mod ovs_controller;
mod system_controller;
#[macro_use]extern crate serde_json;
#[macro_use] extern crate rocket;
use system_controller::{interfaces_api, file_api};

use ovs_controller::{ovs_port, ovs_client};
use rocket::http::{Status, Method, Header};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/interfaces")]
fn interfaces() -> String {
    let local_ifs = match interfaces_api::get_interfaces() {
        Ok(interface_list) => interface_list,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };
    let interface_names: Vec<String> = local_ifs.iter().map(|itf| itf.name.to_string()).collect();
    interface_names.join(",")
}
#[get("/interfaces_to_ip")]
fn interfaces_to_ip() -> String {
    let local_ifs = match interfaces_api::interfaces_to_ip() {
        Ok(interface_list) => interface_list,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };
    local_ifs
}
#[get("/ips")]
fn ips() -> String {
    let local_ips = match interfaces_api::get_local_ips() {
        Ok(ips) => ips,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };
    let ips_str: Vec<String> = local_ips.iter().map(|ip| format!("{}", ip)).collect();
    ips_str.join(",")
}

#[post("/ovs/<bridge>/<port>/<mode>/<vlan>")]
async fn user_str(bridge: &str, port: &str, mode: &str, vlan: u16) -> Status { 
    let ovs_connection = ovs_client::OvsClient::new("192.168.88.101", 6640);
    match ovs_connection{
        Err(error) => {println!{"{}", error}; Status::Conflict},
        Ok(mut result)=>{
            let mode = match mode.to_lowercase().as_str() {
                "access" => ovs_port::OvsPortMode::Access(vlan),
                _ => ovs_port::OvsPortMode::Access(0),
            };
            let add_result = result.add_port(bridge, port, &mode);
            match add_result{
                Err(e) => {
                    println!("{}", e);
                    Status::Conflict
                },
                Ok(i)=>{
                    println!("{}", i);
                    Status::Accepted
                }
            }
        }
    }
}

#[post("/announce")]
fn announce() -> String {
    let result =  match file_api::execute_script() {
        Ok(file) => file,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };
    return result
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![user_str, ips, interfaces, interfaces_to_ip, announce])
        .attach(CORS)
}