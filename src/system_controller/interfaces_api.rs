use std::io::{Error, ErrorKind};
use std::process::Command;
use std::str;
use std::net::{IpAddr, ToSocketAddrs};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub fn get_interfaces() -> Result<Vec<NetworkInterface>, std::io::Error> {
    let network_interfaces = NetworkInterface::show().unwrap();
    if !network_interfaces.is_empty() {
        Ok(network_interfaces)
    } else {
        Err(Error::new(ErrorKind::Other, "Could not find any interfaces"))
    }
}

pub fn interfaces_to_ip() -> Result<String, network_interface::Error> {
    let local_ifs = match NetworkInterface::show() {
        Ok(interface_list) => interface_list,
        Err(e) => return Err(e),
    };
    println!{"{:?}", local_ifs};
    return Ok("string".to_string())
}

pub fn get_local_ips() -> Result<Vec<IpAddr>, std::io::Error> {
    let output = Command::new("ifconfig")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).unwrap();
    let lines: Vec<&str> = output_str.split("\n").collect();
    let mut ips = Vec::new();
    for line in lines {
        if line.contains("inet ") {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let ip = parts[1];
            let addrs = (ip, 0).to_socket_addrs()?;
            for addr in addrs {
                ips.push(addr.ip());
            }
        }
    }
    if !ips.is_empty() {
        Ok(ips)
    } else {
        Err(Error::new(ErrorKind::Other, "Could not find any IP addresses"))
    }
}

pub fn get_interface_bandwidth(interface_name: &str) -> Result<f64, std::io::Error> {
    let output = Command::new("netstat")
        .arg("-bI")
        .arg(interface_name)
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).unwrap();
    let lines: Vec<&str> = output_str.split("\n").collect();
    for line in lines {
        if line.starts_with(interface_name) {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let in_bandwidth = parts[6].parse::<f64>()
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
            let out_bandwidth = parts[9].parse::<f64>()
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
            return Ok((in_bandwidth + out_bandwidth) / 8.0);
        }
    }
    Err(Error::new(ErrorKind::Other, "Could not find any bandwidth information"))
}
