use crate::ovs_controller::ovs_client;
use crate::ovs_controller::ovs_port;
use serde_json::*;
#[test]
fn test_ovs_core() {
    
    let ovsc = ovs_client::OvsClient::new("192.168.88.101", 6640);
    match ovsc{
        Err(e) => println!("{}", e),
        Ok(mut c)=>{
            let ports = c.get_ports();
            match ports{
                Ok(ports) =>{
                    println!("number of port : {0}", ports.len());
                    for port in ports{
                        //println!("{0} : {1}", port.name, port.uuid);
                        println!("{}", serde_json::to_string(&port).unwrap());
                    }
                },
                Err(e) => println!("{}", e)
            }
            
            let bridges = c.get_bridges();
            match bridges{
                Ok(bridges) =>{
                    println!("number of bridges : {0}", bridges.len());
                    for br in bridges{
                        println!("{}", serde_json::to_string(&br).unwrap());
                    }
                },
                Err(e) => println!("{}", e)
            }
            
            let add_result = c.add_port("ovsbr-local", "test", &ovs_port::OvsPortMode::Access(10));
            match add_result{
                Err(e) => {
                    println!("{}", e)
                },
                Ok(i)=>{
                    println!("{}", i)
                }
            }
        }
    }
}
