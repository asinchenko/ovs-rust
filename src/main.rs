mod ovs_controller;
use ovs_controller::{ovs_port, ovs_client};
use rocket::http::Status;

#[macro_use] extern crate rocket;

#[get("/<name>")]
fn index(name: &str) -> String {
    format!("Hello {name}")
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, user_str])
}